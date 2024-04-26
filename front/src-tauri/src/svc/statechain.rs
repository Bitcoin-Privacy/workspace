use anyhow::{anyhow, Result};
use bitcoin::{
    absolute, consensus,
    hashes::sha256,
    hex::DisplayHex,
    secp256k1::{rand, schnorr::Signature, Keypair, Message, PublicKey, Secp256k1, SecretKey},
    sighash::{Prevouts, SighashCache},
    transaction::{self, Version},
    Address, Amount, EcdsaSighashType, Network, OutPoint, ScriptBuf, Sequence, TapSighashType,
    Transaction, TxIn, TxOut, Witness, XOnlyPublicKey,
};
use musig2::{AggNonce, BinaryEncoding, KeyAggContext, PartialSignature, PubNonce, SecNonce};
use secp256k1::All;
use statechain_core::deposit::{create_aggregated_address, create_aggregated_pubkey};
use std::{ops::ControlFlow, str::FromStr};

use crate::{
    api::statechain, cfg::BASE_TX_FEE, db::PoolWrapper, model::AccountActions,
    store::master_account::WALLET,
};
use shared::{
    api::broadcast_txn,
    intf::statechain::{DepositInfo, StatecoinDto},
};

use crate::connector::NodeConnector;

use super::account;

/*
Init statecoin
pub fn new_statecoin()
Deposit init - get server pubkey + statechain\_id --> coin ->> wallet (db)
--> aggregated adddress + server pubkey
*/

fn new_keypair(secp: &Secp256k1<All>) -> (SecretKey, PublicKey) {
    let keypair = Keypair::new(secp, &mut rand::thread_rng());
    let sk = SecretKey::from_keypair(&keypair);
    let pk = sk.public_key(secp);
    (sk, pk)
}

/// Desposit function
/// - Create a new statecoin in local
/// - Request to the server to get server's public key
/// - Generate aggregated address
pub async fn deposit(
    pool: &PoolWrapper,
    conn: &NodeConnector,
    deriv: &str,
    amount: u64,
) -> Result<DepositInfo> {
    // Init stuffs
    let secp = Secp256k1::new();

    let auth_keypair = Keypair::new(&secp, &mut rand::thread_rng());
    let auth_seckey = SecretKey::from_keypair(&auth_keypair);
    let xonly_auth_pubkey = XOnlyPublicKey::from_keypair(&auth_keypair).0;

    let (account, _) = account::get_account(deriv).unwrap();
    let account_address = account.get_addr();

    // Request to the server to Register
    // and get server's publickey and statechain id
    let response = statechain::deposit(conn, xonly_auth_pubkey.to_string(), amount as u32).await?;
    let se_pubkey = response.se_pubkey_1;
    let statechain_id = response.statechain_id;

    {
        let mut wallet = WALLET.lock().unwrap();
        let coin = wallet.as_mut().unwrap().coins.last_mut().unwrap();
        let aggregated_public_key = create_aggregated_address(coin, String::from("testnet"))?;

        coin.amount = Some(amount as u32);
        coin.aggregated_address = Some(aggregated_public_key.aggregate_address.clone());
        coin.aggregated_pubkey = Some(aggregated_public_key.aggregate_pubkey);
    }

    //gen o1
    let (owner_sk, owner_pk) = new_keypair(&secp);
    println!("KEYPAIR - PRIV: {:#?}", owner_sk.display_secret());
    println!("KEYPAIR - PUBL: {:#?}", owner_pk.to_string());

    //gen auth_key

    // combine 2 address
    let aggr = create_aggregated_pubkey(&owner_pk.to_string(), &se_pubkey, "testnet")?;

    println!("agg pub key {}", aggr.aggregate_pubkey);
    let aggr_pubkey = PublicKey::from_str(&aggr.aggregate_pubkey)?;

    if let Err(e) = pool
        .insert_statecoin(
            &statechain_id,
            &account_address,
            amount,
            &auth_seckey,
            &xonly_auth_pubkey,
            &aggr.aggregate_pubkey,
            &aggr.aggregate_address,
            &owner_sk,
            &owner_pk,
        )
        .await
    {
        panic!("Failed to insert statecoin data {:?}", e);
    }

    let deposit_tx = create_deposit_txn(pool, deriv, amount, &aggr_pubkey).await?;

    let txid = deposit_tx.txid().to_string();

    // create_bk_tx(
    //     pool,
    //     conn,
    //     &key_agg_ctx,
    //     &aggr_pubkey,
    //     &aggregated_pubkey_tw,
    //     &aggregated_address,
    //     &account_address,
    //     &txid,
    //     0,
    //     amount,
    //     &statechain_id,
    // )
    // .await?;

    Ok(DepositInfo {
        aggregated_address: aggr.aggregate_address,
        deposit_tx_hex: consensus::encode::serialize_hex(&deposit_tx),
    })
}

pub async fn create_deposit_txn(
    pool: &PoolWrapper,
    deriv: &str,
    amount: u64,
    aggregated_pubkey: &PublicKey,
) -> Result<Transaction> {
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
        input,
        output,
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
    let funding_vout = 0_u64;
    unsigned_deposit_tx.txid();

    // WARN: Do not broadcast here
    let res = broadcast_txn(&tx_hex).await;
    println!("BROADCASTED {:#?}", res);

    // let _ = pool
    //     .update_deposit_tx(
    //         statechain_id,
    //         &funding_txid,
    //         funding_vout,
    //         "CONFIRM",
    //         &tx_hex,
    //     )
    //     .await?;
    //
    Ok(unsigned_deposit_tx)
}

pub async fn create_bk_tx(
    pool: &PoolWrapper,
    conn: &NodeConnector,
    key_agg_ctx: &KeyAggContext,
    agg_pubkey: &PublicKey,
    agg_pubkey_tw: &PublicKey,
    agg_address: &Address,
    receiver_address: &str,
    txid: &str,
    vout: u32,
    amount: u64,
    statechain_id: &str,
) -> Result<()> {
    let secp = Secp256k1::new();
    let seckey = pool.get_seckey_by_id(statechain_id).await?.unwrap();
    let seckey = SecretKey::from_str(&seckey).unwrap();

    let agg_scriptpubkey = ScriptBuf::new_p2tr(&secp, agg_pubkey.x_only_public_key().0, None);

    println!("Public key agg: {}", agg_pubkey.x_only_public_key().0);

    let prev_outpoint = OutPoint {
        txid: txid.parse().unwrap(),
        vout,
    };

    let input = TxIn {
        previous_output: prev_outpoint,
        script_sig: ScriptBuf::default(),
        sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
        witness: Witness::default(),
    };

    let output_address = Address::from_str(receiver_address).unwrap();
    let checked_output_address = output_address.require_network(Network::Testnet).unwrap();
    let spend = TxOut {
        value: Amount::from_sat(amount - BASE_TX_FEE),
        script_pubkey: checked_output_address.script_pubkey(),
    };

    let mut unsigned_tx = Transaction {
        version: transaction::Version::TWO,  // Post BIP-68.
        lock_time: absolute::LockTime::ZERO, // Ignore the locktime.
        input: vec![input],                  // Input goes into index 0.
        output: vec![spend],                 // Outputs, order does not matter.
    };

    let utxo = TxOut {
        value: Amount::from_sat(amount),
        script_pubkey: agg_scriptpubkey,
    };

    println!("utxo that bk sign:{:#?}", utxo);

    println!("pub key tw: {}", agg_pubkey_tw.x_only_public_key().0);

    let prevouts = vec![utxo];
    let prevouts = Prevouts::All(&prevouts);
    let mut sighasher = SighashCache::new(&mut unsigned_tx);

    let sighash_type = TapSighashType::All;
    let sighash = sighasher
        .taproot_key_spend_signature_hash(0, &prevouts, sighash_type)
        .expect("failed to construct sighash");

    println!("sighash : {}", sighash);

    println!("hash tab sighash {}", sighash.as_raw_hash());

    let message = sighash.to_string();

    let parsed_msg = message.clone();
    let msg_clone = parsed_msg.clone();
    let msg = parsed_msg.clone();

    println!("messsageee : {}", msg);

    let signed_statechain_id = sign_message(statechain_id, &seckey).to_string();

    let get_nonce_res = statechain::get_nonce(conn, statechain_id, &signed_statechain_id).await?;
    let server_pubnonce = get_nonce_res.server_nonce;

    let nonce_seed = [0xACu8; 32];

    let secnonce = SecNonce::build(nonce_seed).with_seckey(seckey).build();

    let our_public_nonce = secnonce.public_nonce();

    let public_nonces = [
        our_public_nonce,
        server_pubnonce.parse::<PubNonce>().unwrap(),
    ];

    let agg_pubnonce: AggNonce = public_nonces.iter().sum();

    let agg_pubnonce_str = agg_pubnonce.to_string();

    let our_partial_signature: PartialSignature =
        musig2::sign_partial(key_agg_ctx, seckey, secnonce, &agg_pubnonce, message)
            .expect("error creating partial signature");

    let serialized_key_agg_ctx = key_agg_ctx
        .to_bytes()
        .to_hex_string(bitcoin::hex::Case::Lower);

    let get_sign_res = statechain::get_partial_signature(
        conn,
        &serialized_key_agg_ctx,
        statechain_id,
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
        key_agg_ctx,
        &agg_pubnonce,
        partial_signatures,
        msg_clone,
    )
    .expect("error aggregating signatures");

    musig2::verify_single(*agg_pubkey_tw, final_signature, msg)
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

    let tx_hex = consensus::encode::serialize_hex(&tx);
    pool.update_bk_tx(statechain_id, &tx_hex, &agg_pubnonce.to_string())
        .await?;

    println!("Bk tx hex: {}", tx_hex);

    Ok(())
}

pub fn sign_message(msg: &str, seckey: &SecretKey) -> Signature {
    let secp = Secp256k1::new();
    let message = Message::from_hashed_data::<sha256::Hash>(msg.to_string().as_bytes());
    let keypair = Keypair::from_seckey_slice(&secp, seckey.as_ref()).unwrap();
    let signed_message = secp.sign_schnorr(&message, &keypair);

    signed_message
}

pub async fn get_statecoins(conn: &NodeConnector, addr: &str) -> Result<Vec<StatecoinDto>> {
    statechain::get_statecoins(conn, addr).await
}
