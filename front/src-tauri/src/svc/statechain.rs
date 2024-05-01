use anyhow::{anyhow, Result};
use bitcoin::{
    absolute::{self, LockTime},
    consensus::{self, serde::hex},
    hashes::sha256,
    hex::DisplayHex,
    secp256k1::{rand, Keypair, PublicKey, Secp256k1, SecretKey},
    sighash::{Prevouts, SighashCache},
    string::FromHexStr,
    transaction::{self, Version},
    Address, Amount, EcdsaSighashType, Network, OutPoint, ScriptBuf, Sequence, TapSighash,
    TapSighashType, Transaction, TxIn, TxOut, Txid, WPubkeyHash, Witness, XOnlyPublicKey,
};

use musig2::{
    secp::MaybeScalar, AggNonce, BinaryEncoding, KeyAggContext, PartialSignature, PubNonce,
    SecNonce,
};

use rand::RngCore;
use secp256k1::{schnorr::Signature, Message, Scalar};
use serde::Serialize;
use statechain_core::transfer::receiver;
use std::{
    ops::{Add, ControlFlow},
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    api::statechain::{self},
    cfg::{BASE_TX_FEE, INIT_NLOCKTIME, INTERVAL},
    db::PoolWrapper,
    model::{AccountActions, StateCoin, StateCoinInfo},
};
use shared::intf::statechain::{
    CreateBkTxnReq, CreateBkTxnRes, DepositInfo, DepositReq, DepositRes, TransferMessage,
};

use crate::connector::NodeConnector;

use super::account;

pub async fn deposit(
    pool: &PoolWrapper,
    conn: &NodeConnector,
    deriv: &str,
    amount: u64,
) -> Result<DepositInfo> {
    let secp = Secp256k1::new();

    let auth_keypair = Keypair::new(&secp, &mut rand::thread_rng());
    let auth_seckey = SecretKey::from_keypair(&auth_keypair);
    let xonly_auth_pubkey = XOnlyPublicKey::from_keypair(&auth_keypair).0;

    let (account, _) = account::get_account(deriv).unwrap();
    let account_address = account.get_addr();

    let req = DepositReq {
        token_id: "abc".to_string(),
        addr: xonly_auth_pubkey.to_string(),
        amount: amount as u32,
    };
    let body = serde_json::to_value(req)?;
    let res = conn.post("statechain/deposit", &body).await?;

    let json: DepositRes = serde_json::from_value(res)?;
    // response
    let se_pubkey = json.se_pubkey_1;
    let statechain_id = json.statechain_id;

    //gen o1
    let owner_keypair = Keypair::new(&secp, &mut rand::thread_rng());
    let owner_seckey = SecretKey::from_keypair(&owner_keypair);
    let owner_pubkey = PublicKey::from_keypair(&owner_keypair);
    let signed_statechain_id = sign_message(&statechain_id, &auth_seckey).to_string();

    //gen auth_key

    // combine 2 address
    let (aggregated_pubkey, aggregated_pubkey_tw, aggregated_address, key_agg_ctx) =
        aggregate_pubkeys(owner_pubkey, PublicKey::from_str(&se_pubkey).unwrap());

    println!(
        "agg pub key {}",
        aggregated_pubkey_tw.x_only_public_key().0.to_string()
    );

    let (funding_txid, vout, deposit_tx) =
        create_deposit_transaction(&deriv, amount, &aggregated_pubkey).await?;
    let output_address = Address::from_str(&account_address).unwrap();
    let checked_output_address = output_address.require_network(Network::Testnet).unwrap();

    let bk_tx = create_bk_tx(
        &conn,
        &owner_seckey,
        &statechain_id,
        &signed_statechain_id,
        &funding_txid,
        0,
        amount,
        0,
        &aggregated_pubkey,
        &key_agg_ctx,
        &checked_output_address,
    )
    .await?;

    if let Err(e) = pool
        .create_statecoin(
            &statechain_id,
            &signed_statechain_id,
            &account_address,
            amount,
            &auth_seckey,
            &xonly_auth_pubkey,
            &aggregated_pubkey.to_string(),
            &aggregated_address.to_string(),
            &owner_seckey,
            &owner_pubkey,
            &key_agg_ctx,
            &funding_txid,
            vout,
            &deposit_tx,
            0,
            0,
        )
        .await
    {
        panic!("Failed to insert statecoin data {:?}", e);
    }

    pool.create_bk_tx(&statechain_id, &bk_tx, 0, 0).await?;
    Ok(DepositInfo {
        aggregated_address: aggregated_address.to_string(),
        deposit_tx_hex: consensus::encode::serialize_hex(&deposit_tx),
    })
}

pub async fn create_deposit_transaction(
    deriv: &str,
    amount: u64,
    aggregated_pubkey: &PublicKey,
) -> Result<(String, u64, String)> {
    let (account, mut unlocker) = account::get_account(deriv).unwrap();
    let selected_utxos = account::get_utxos_set(&account.get_addr(), amount + BASE_TX_FEE).await?;
    let secp = Secp256k1::new();
    let mut fee: u64 = 0;
    let input: Vec<TxIn> = selected_utxos
        .iter()
        .map(|utxo| {
            fee += utxo.value;
            println!("utxos set :{}", utxo.value);
            TxIn {
                previous_output: OutPoint::new(utxo.txid.parse().unwrap(), utxo.vout.into()),
                script_sig: ScriptBuf::from_bytes(vec![]),
                sequence: Sequence::MAX,
                witness: Witness::new(),
            }
        })
        .collect();

    let mut output: Vec<TxOut> = Vec::new();

    output.push(TxOut {
        value: Amount::from_sat(amount),
        script_pubkey: ScriptBuf::new_p2tr(&secp, aggregated_pubkey.x_only_public_key().0, None),
    });

    let (change, overflow) = fee.overflowing_sub(amount + BASE_TX_FEE);
    if overflow {
        return Err(anyhow!("Total amount cannot cover amount and fee"));
    }
    if change > 0 {
        output.push(TxOut {
            value: Amount::from_sat(change),
            script_pubkey: account.get_checked_addr().script_pubkey(),
        });
    }

    let deposit_tx = Transaction {
        version: Version::TWO,
        lock_time: absolute::LockTime::ZERO,
        input: input,
        output: output,
    };

    let mut unsigned_deposit_tx = deposit_tx.clone();

    let sighash_type = EcdsaSighashType::All;
    let mut sighasher = SighashCache::new(&mut unsigned_deposit_tx);

    let future_tasks: Vec<_> = deposit_tx
        .input
        .iter()
        .enumerate()
        .map(|(index, input)| {
            tokio::spawn(tokio::spawn(account::find_and_join_txn(
                index,
                input.clone(),
            )))
        })
        .collect();

    let mut results = Vec::new();
    for job in future_tasks {
        results.push(job.await.unwrap().unwrap().unwrap());
    }

    let res = results.iter().try_for_each(|(index, input, tx)| {
        match account::sign(
            &secp,
            &mut sighasher,
            sighash_type,
            &account,
            &mut unlocker,
            index,
            input,
            tx,
        ) {
            Ok(_) => ControlFlow::Continue(()),
            Err(e) => ControlFlow::Break(e),
        }
    });
    if let ControlFlow::Break(e) = res {
        return Err(e);
    }

    let tx_hex = consensus::encode::serialize_hex(&unsigned_deposit_tx);
    println!("deposit tx hex: {:?}", tx_hex);
    println!("deposit tx raw {:#?}", unsigned_deposit_tx);
    let funding_txid = unsigned_deposit_tx.txid().to_string();
    let funding_vout = 0 as u64;
    Ok((funding_txid, funding_vout, tx_hex.to_string()))
}

pub async fn create_bk_tx(
    conn: &NodeConnector,
    seckey: &SecretKey,
    statechain_id: &str,
    signed_statechain_id: &str,
    txid: &str,
    n_lock_time: u32,
    amount: u64,
    vout: i64,
    agg_pubkey: &PublicKey,
    key_agg_ctx: &KeyAggContext,
    receiver_address: &Address,
) -> Result<String> {
    let secp = Secp256k1::new();

    let agg_scriptpubkey = ScriptBuf::new_p2tr(&secp, agg_pubkey.x_only_public_key().0, None);

    println!(
        "Public key agg: {}",
        agg_pubkey.x_only_public_key().0.to_string()
    );

    let prev_outpoint = OutPoint {
        txid: Txid::from_str(&txid)?,
        vout: vout as u32,
    };
    let sq = if n_lock_time == 0 {
        Sequence::ENABLE_RBF_NO_LOCKTIME
    } else {
        Sequence::ENABLE_LOCKTIME_NO_RBF
    };
    let input = TxIn {
        previous_output: prev_outpoint,
        script_sig: ScriptBuf::default(),
        sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
        witness: Witness::default(),
    };

    let spend = TxOut {
        value: Amount::from_sat(amount - BASE_TX_FEE),
        script_pubkey: receiver_address.script_pubkey(),
    };

    let mut unsigned_tx = Transaction {
        version: transaction::Version::TWO, // Post BIP-68.
        lock_time: LockTime::ZERO,          // Ignore the locktime.
        input: vec![input],                 // Input goes into index 0.
        output: vec![spend],                // Outputs, order does not matter.
    };

    let utxo = TxOut {
        value: Amount::from_sat(amount),
        script_pubkey: agg_scriptpubkey,
    };

    println!("utxo that bk sign:{:#?}", utxo);

    let prevouts = vec![utxo];
    let prevouts = Prevouts::All(&prevouts);
    let mut sighasher = SighashCache::new(&mut unsigned_tx);

    let sighash_type = TapSighashType::All;
    let sighash = sighasher
        .taproot_key_spend_signature_hash(0, &prevouts, sighash_type)
        .expect("failed to construct sighash");

    println!("sighash : {}", sighash.to_string());

    let message = sighash.to_string();
    let parsed_msg = message.clone();
    let msg_clone = parsed_msg.clone();
    let msg = parsed_msg.clone();

    println!("messsageee : {}", msg);

    let get_nonce_res = statechain::get_nonce(&conn, statechain_id, &signed_statechain_id).await?;
    let server_pubnonce = get_nonce_res.server_nonce;

    let mut nonce_seed = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut nonce_seed);

    let secnonce = SecNonce::build(nonce_seed).with_seckey(*seckey).build();

    let our_public_nonce = secnonce.public_nonce();

    let public_nonces = [
        our_public_nonce,
        server_pubnonce.parse::<PubNonce>().unwrap(),
    ];

    let agg_pubnonce: AggNonce = public_nonces.iter().sum();

    let agg_pubnonce_str = agg_pubnonce.to_string();

    let our_partial_signature: PartialSignature =
        musig2::sign_partial(&key_agg_ctx, *seckey, secnonce, &agg_pubnonce, message)
            .expect("error creating partial signature");

    let serialized_key_agg_ctx = key_agg_ctx
        .to_bytes()
        .to_hex_string(bitcoin::hex::Case::Lower);

    let get_sign_res = statechain::get_partial_signature(
        &conn,
        &serialized_key_agg_ctx,
        &statechain_id,
        &signed_statechain_id,
        &msg_clone,
        &agg_pubnonce_str,
    )
    .await?;

    let server_signature = get_sign_res.partial_signature;

    let partial_signatures = [
        our_partial_signature,
        PartialSignature::from_hex(&server_signature).unwrap(),
    ];

    let final_signature: secp256k1::schnorr::Signature = musig2::aggregate_partial_signatures(
        &key_agg_ctx,
        &agg_pubnonce,
        partial_signatures,
        msg_clone,
    )
    .expect("error aggregating signatures");

    let agg_pubkey_tw: PublicKey = key_agg_ctx.aggregated_pubkey();
    println!("tx public key : {}", agg_pubkey_tw.to_string());

    musig2::verify_single(agg_pubkey_tw, final_signature, msg)
        .expect("aggregated signature must be valid");

    let signature = bitcoin::taproot::Signature {
        sig: final_signature,
        hash_ty: sighash_type,
    };

    println!(
        "signature byte: {:#?}",
        signature.to_vec().to_lower_hex_string()
    );

    let mut wit = Witness::new();
    wit.push(signature.to_vec());
    *sighasher.witness_mut(0).unwrap() = wit;

    let tx = sighasher.into_transaction();

    println!("Bk tx raw: {:#?}", tx);

    let tx_hex = consensus::encode::serialize_hex(&tx);

    println!("Bk tx hex: {}", tx_hex);

    Ok(tx_hex)
}

pub fn sign_message(msg: &str, seckey: &SecretKey) -> Signature {
    let secp = Secp256k1::new();
    let message = Message::from_hashed_data::<sha256::Hash>(msg.to_string().as_bytes());
    let keypair = Keypair::from_seckey_slice(&secp, seckey.as_ref()).unwrap();
    let signed_message = secp.sign_schnorr(&message, &keypair);

    signed_message
}

pub fn aggregate_pubkeys(
    owner_pubkey: PublicKey,
    se_pubkey: PublicKey,
) -> (PublicKey, PublicKey, Address, KeyAggContext) {
    let secp = Secp256k1::new();
    let mut pubkeys: Vec<PublicKey> = vec![];
    pubkeys.push(owner_pubkey);
    pubkeys.push(se_pubkey);
    let key_agg_ctx_tw = KeyAggContext::new(pubkeys.clone())
        .unwrap()
        .with_unspendable_taproot_tweak()
        .unwrap();

    let aggregated_pubkey: PublicKey = key_agg_ctx_tw.aggregated_pubkey_untweaked();
    let aggregated_pubkey_tw: PublicKey = key_agg_ctx_tw.aggregated_pubkey();

    let aggregated_address = Address::p2tr(
        &secp,
        aggregated_pubkey.x_only_public_key().0,
        None,
        Network::Testnet,
    );

    (
        aggregated_pubkey,
        aggregated_pubkey_tw,
        aggregated_address,
        key_agg_ctx_tw,
    )
}

pub fn compute_t1(owner_seckey: &SecretKey, random_key: &Scalar) -> SecretKey {
    let res = owner_seckey.add_tweak(random_key).unwrap();
    res
}

pub async fn list_statecoins(pool: &PoolWrapper, deriv: &str) -> Result<Vec<StateCoinInfo>> {
    let (account, _) = account::get_account(deriv).unwrap();
    let account_address = account.get_addr();

    match pool.list_statecoins_by_account(&account_address).await {
        Ok(statecoins) => Ok(statecoins),
        Err(e) => Err(e),
    }
}

pub fn calculate_nlocktime_for_bk(num_owner: u64, init_nlocktime: u64) -> Result<u64> {
    let current_time = SystemTime::now();

    // Calculate the Unix time by subtracting the UNIX epoch time
    let current_unix_time = current_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let new_nlocktime = init_nlocktime - (num_owner - 1) * INTERVAL;
    if current_unix_time >= new_nlocktime {
        return Ok(0);
    }

    Ok(new_nlocktime)
}

pub async fn create_bk_tx_for_receiver(
    conn: &NodeConnector,
    pool: &PoolWrapper,
    statechain_id: &str,
    statecoin: &StateCoin,
    receiver_address: &Address,
    n_lock_time: u32,
) -> Result<String> {
    let amount = statecoin.amount as u64;

    let agg_pubkey = PublicKey::from_str(&statecoin.aggregated_pubkey)?;
    let vout = 0 as i64;
    let key_agg_ctx = KeyAggContext::from_hex(&statecoin.key_agg_ctx).unwrap();

    let secp = Secp256k1::new();
    let seckey = pool
        .get_seckey_by_id(&statechain_id)
        .await
        .unwrap()
        .unwrap();
    let seckey = SecretKey::from_str(&seckey).unwrap();

    let agg_scriptpubkey = ScriptBuf::new_p2tr(&secp, agg_pubkey.x_only_public_key().0, None);

    println!(
        "Public key agg: {}",
        agg_pubkey.x_only_public_key().0.to_string()
    );

    let prev_outpoint = OutPoint {
        txid: Txid::from_str(&statecoin.funding_txid)?,
        vout: vout as u32,
    };
    let input = TxIn {
        previous_output: prev_outpoint,
        script_sig: ScriptBuf::default(),
        sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
        witness: Witness::default(),
    };

    let spend = TxOut {
        value: Amount::from_sat(amount - BASE_TX_FEE),
        script_pubkey: receiver_address.script_pubkey(),
    };

    let mut unsigned_tx = Transaction {
        version: transaction::Version::TWO, // Post BIP-68.
        lock_time: LockTime::ZERO,          // Ignore the locktime.
        input: vec![input],                 // Input goes into index 0.
        output: vec![spend],                // Outputs, order does not matter.
    };

    let utxo = TxOut {
        value: Amount::from_sat(amount),
        script_pubkey: agg_scriptpubkey,
    };

    println!("utxo that bk sign:{:#?}", utxo);

    let prevouts = vec![utxo];
    let prevouts = Prevouts::All(&prevouts);
    let mut sighasher = SighashCache::new(&mut unsigned_tx);

    let sighash_type = TapSighashType::All;
    let sighash = sighasher
        .taproot_key_spend_signature_hash(0, &prevouts, sighash_type)
        .expect("failed to construct sighash");

    println!("sighash : {}", sighash.to_string());

    let message = sighash.to_string();
    let parsed_msg = message.clone();
    let msg_clone = parsed_msg.clone();
    let msg = parsed_msg.clone();

    println!("messsageee : {}", msg);

    let get_nonce_res =
        statechain::get_nonce(&conn, statechain_id, &statecoin.signed_statechain_id).await?;
    let server_pubnonce = get_nonce_res.server_nonce;

    let mut nonce_seed = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut nonce_seed);

    let secnonce = SecNonce::build(nonce_seed).with_seckey(seckey).build();

    let our_public_nonce = secnonce.public_nonce();

    let public_nonces = [
        our_public_nonce,
        server_pubnonce.parse::<PubNonce>().unwrap(),
    ];

    let agg_pubnonce: AggNonce = public_nonces.iter().sum();

    let agg_pubnonce_str = agg_pubnonce.to_string();

    let our_partial_signature: PartialSignature =
        musig2::sign_partial(&key_agg_ctx, seckey, secnonce, &agg_pubnonce, message)
            .expect("error creating partial signature");

    let serialized_key_agg_ctx = key_agg_ctx
        .to_bytes()
        .to_hex_string(bitcoin::hex::Case::Lower);

    let get_sign_res = statechain::get_partial_signature(
        &conn,
        &serialized_key_agg_ctx,
        &statechain_id,
        &statecoin.signed_statechain_id,
        &msg_clone,
        &agg_pubnonce_str,
    )
    .await?;

    let server_signature = get_sign_res.partial_signature;

    let partial_signatures = [
        our_partial_signature,
        PartialSignature::from_hex(&server_signature).unwrap(),
    ];

    let final_signature: secp256k1::schnorr::Signature = musig2::aggregate_partial_signatures(
        &key_agg_ctx,
        &agg_pubnonce,
        partial_signatures,
        msg_clone,
    )
    .expect("error aggregating signatures");

    let agg_pubkey_tw: PublicKey = key_agg_ctx.aggregated_pubkey();
    println!("tx public key : {}", agg_pubkey_tw.to_string());

    musig2::verify_single(agg_pubkey_tw, final_signature, msg)
        .expect("aggregated signature must be valid");

    let signature = bitcoin::taproot::Signature {
        sig: final_signature,
        hash_ty: sighash_type,
    };

    println!(
        "signature byte: {:#?}",
        signature.to_vec().to_lower_hex_string()
    );

    let mut wit = Witness::new();
    wit.push(signature.to_vec());
    *sighasher.witness_mut(0).unwrap() = wit;

    let tx = sighasher.into_transaction();

    println!("Bk tx raw: {:#?}", tx);

    let tx_hex = consensus::encode::serialize_hex(&tx);

    println!("Bk tx hex: {}", tx_hex);

    Ok(tx_hex)
}

pub async fn send_statecoin(
    conn: &NodeConnector,
    pool: &PoolWrapper,
    pubkey: &str,
    authkey: &str,
    statechain_id: &str,
) -> Result<String> {
    //1. generate tx2 by key
    // let s = ScriptBuf::new_p2wpkh(WPubkeyHash::from)
    let b_pubkey = bitcoin::PublicKey::from_str(pubkey)?;
    let receiver_address = Address::p2wpkh(&b_pubkey, Network::Testnet)?;
    let statecoin = pool.get_statecoin_by_id(&statechain_id).await?;

    let tx = create_bk_tx_for_receiver(
        &conn,
        &pool,
        &statechain_id,
        &statecoin,
        &receiver_address,
        0,
    )
    .await?;

    //2. send register new owner

    let register_new_owner_res = statechain::register_new_owner(
        &conn,
        &statechain_id,
        &statecoin.signed_statechain_id,
        authkey,
    )
    .await?;
    let x1 = register_new_owner_res.random_key;
    println!("x1 {}", x1);
    let bytes = x1
        .as_bytes()
        .chunks(2)
        .map(|chunk| u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16).unwrap())
        .collect::<Vec<u8>>();
    let mut random = [0u8; 32];
    random.copy_from_slice(&bytes[..32]);


    let x1 = Scalar::from_le_bytes(random)?;

    println!("random x1 {:#?}", x1.to_le_bytes().to_lower_hex_string());

    //3.compute t1

    let t1 = compute_t1(&SecretKey::from_str(&statecoin.owner_seckey)?, &x1);

    //3. create transfer message

    let mut backup_txs = pool.get_bk_tx_by_statechain_id(&statechain_id).await?;
    backup_txs.push(tx);
    let transfer_message = TransferMessage {
        total_owner: statecoin.tx_n,
        backup_txs: backup_txs,
        t1: t1.display_secret().to_string(),
        statechain_id: statechain_id.to_string(),
        agg_pubkey: statecoin.aggregated_pubkey.to_string(),
    };

    statechain::create_transfer_msg(&conn, &transfer_message, authkey).await?;

    Ok("send success".to_string())
}
