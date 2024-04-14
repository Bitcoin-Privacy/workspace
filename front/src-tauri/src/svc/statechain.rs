use anyhow::{anyhow, Result};
use bitcoin::{
    absolute,
    consensus::{self, serde::hex::Lower},
    hashes::sha256,
    hex::DisplayHex,
    key::{TapTweak, TweakedKeypair},
    secp256k1::{rand, Keypair, PublicKey, Secp256k1, SecretKey},
    sighash::{Prevouts, SighashCache},
    transaction::Version,
    Address, Amount, EcdsaSighashType, Network, OutPoint, ScriptBuf, Sequence, TapSighashType,
    Transaction, TxIn, TxOut, Txid, Witness,
};

use musig2::{
    AggNonce, BinaryEncoding, FirstRound, KeyAggContext, PartialSignature, PubNonce, SecNonce,
    SecNonceSpices, SecondRound,
};

use curve25519_dalek::scalar::Scalar;

use secp256k1::{schnorr::Signature, Message};
use serde::Serialize;
use serde_json::Serializer;

use std::{num::ParseIntError, ops::ControlFlow, str::FromStr};

use crate::{api::statechain, cfg::BASE_TX_FEE, db::PoolWrapper, model::AccountActions};
use shared::intf::statechain::{AggregatedPublicKey, DepositReq, DepositRes};

use crate::connector::NodeConnector;

use super::account;

pub async fn deposit(
    pool: &PoolWrapper,
    conn: &NodeConnector,
    deriv: &str,
    amount: u64,
) -> Result<AggregatedPublicKey> {
    let secp = Secp256k1::new();

    let auth_keypair = Keypair::new(&secp, &mut rand::thread_rng());
    let auth_seckey = SecretKey::from_keypair(&auth_keypair);
    let auth_pubkey = PublicKey::from_keypair(&auth_keypair);

    let (account, _) = account::get_account(deriv).unwrap();
    let account_address = account.get_addr();

    let req = DepositReq {
        token_id: "abc".to_string(),
        addr: auth_pubkey.to_string(),
        amount: amount as u32,
    };
    println!("Deposit request {:#?}", req);
    let body = serde_json::to_value(req)?;
    let res = conn.post("statechain/deposit", &body).await?;

    let json: DepositRes = serde_json::from_value(res)?;
    println!("Deposit response {:#?}", json);
    // response
    let se_pubkey = json.se_pubkey_1;
    let statechain_id = json.statechain_id;

    //gen o1
    let owner_keypair = Keypair::new(&secp, &mut rand::thread_rng());
    let owner_seckey = SecretKey::from_keypair(&owner_keypair);
    let owner_pubkey = PublicKey::from_keypair(&owner_keypair);

    //gen auth_key

    // combine 2 address
    let mut pubkeys: Vec<PublicKey> = vec![];
    pubkeys.push(se_pubkey.parse::<PublicKey>().unwrap());
    pubkeys.push(owner_pubkey);
    let key_agg_ctx = KeyAggContext::new(pubkeys).unwrap();

    let aggregated_pubkey: PublicKey = key_agg_ctx.aggregated_pubkey();

    let aggregated_address = Address::p2tr(
        &secp,
        aggregated_pubkey.x_only_public_key().0,
        None,
        Network::Testnet,
    );

    let key = AggregatedPublicKey {
        aggregated_pubkey: aggregated_pubkey.to_string(),
        aggregated_address: aggregated_address.to_string(),
    };

    if let Err(e) = pool
        .insert_statecoin(
            &statechain_id,
            &account_address,
            amount,
            &auth_seckey,
            &auth_pubkey,
            &aggregated_pubkey.to_string(),
            &aggregated_address.to_string(),
            &owner_seckey,
            &owner_pubkey,
        )
        .await
    {
        panic!("Failed to insert statecoin data {:?}", e);
    }

    let txid = create_deposit_transaction(
        &pool,
        &deriv,
        amount,
        &aggregated_address.to_string(),
        &statechain_id,
    )
    .await?;

    let tx = create_bk_tx(
        &pool,
        &conn,
        &key_agg_ctx,
        &aggregated_pubkey,
        &aggregated_address,
        &account_address,
        &txid,
        1,
        amount,
        &statechain_id,
    )
    .await
    .unwrap();
    println!("bk tx : {}", consensus::encode::serialize_hex(&tx));

    Ok(key)
}

// pub fn create_aggregated_address(
//     k1: String,
//     k2: String,
//     network: Network,
// ) -> (AggregatedPublicKey, KeyAggContext) {
//     let secp = Secp256k1::new();

//     let mut pubkeys: Vec<PublicKey> = vec;
//     pubkeys.push(k1.parse::<PublicKey>().unwrap());
//     pubkeys.push(k2.parse::<PublicKey>().unwrap());
//     let key_agg_ctx = KeyAggContext::new(pubkeys).unwrap();

//     let aggregated_pubkey: PublicKey = key_agg_ctx.aggregated_pubkey();

//     let aggregated_address = Address::p2tr(
//         &secp,
//         aggregated_pubkey.x_only_public_key().0,
//         None,
//         network,
//     );

//     (
//         AggregatedPublicKey {
//             aggregated_pubkey: aggregated_pubkey.to_string(),
//             aggregated_address: aggregated_address.to_string(),
//         },
//         key_agg_ctx,
//     )
// }

pub async fn create_deposit_transaction(
    pool: &PoolWrapper,
    deriv: &str,
    amount: u64,
    aggregated_address: &str,
    statechain_id: &str,
) -> Result<String> {
    let (account, mut unlocker) = account::get_account(deriv).unwrap();
    let selected_utxos = account::get_utxos_set(&account.get_addr(), amount).await?;

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

    let addr = Address::from_str(aggregated_address).unwrap();
    let checked_addr = addr.require_network(Network::Testnet).unwrap();

    output.push(TxOut {
        value: Amount::from_sat(amount),
        script_pubkey: checked_addr.script_pubkey(),
    });

    let deposit_tx = Transaction {
        version: Version::TWO,
        lock_time: absolute::LockTime::ZERO,
        input: input,
        output: output,
    };

    let mut unsigned_deposit_tx = deposit_tx.clone();

    let secp = Secp256k1::new();
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
    println!("deposit transaction: {:?}", tx_hex);
    println!("{:#?}", unsigned_deposit_tx);
    let funding_txid = unsigned_deposit_tx.txid().to_string();
    let funding_vout = 1 as u64;
    let _ = pool
        .update_deposit_tx(
            &statechain_id,
            &funding_txid,
            funding_vout,
            "CONFIRM",
            &tx_hex,
        )
        .await?;

    Ok(funding_txid)
}

pub async fn create_bk_tx(
    pool: &PoolWrapper,
    conn: &NodeConnector,
    key_agg_ctx: &KeyAggContext,
    agg_pubkey: &PublicKey,
    agg_address: &Address,
    receiver_address: &str,
    txid: &str,
    vout: u32,
    amount: u64,
    statechain_id: &str,
) -> Result<Transaction> {
    let secp = Secp256k1::new();
    let seckey = pool
        .get_seckey_by_id(&statechain_id)
        .await
        .unwrap()
        .unwrap();
    let seckey = SecretKey::from_str(&seckey).unwrap();

    let agg_scriptpubkey = agg_address.script_pubkey();

    let utxo = TxOut {
        value: Amount::from_sat(amount),
        script_pubkey: agg_scriptpubkey,
    };

    let prev_outpoint = OutPoint {
        txid: Txid::from_str(txid).unwrap(),
        vout: vout,
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
        value: Amount::from_sat(amount),
        script_pubkey: checked_output_address.script_pubkey(),
    };

    let unsigned_tx = Transaction {
        version: Version::TWO,               // Post BIP-68.
        lock_time: absolute::LockTime::ZERO, // Ignore the locktime.
        input: vec![input],                  // Input goes into index 0.
        output: vec![spend],                 // Outputs, order does not matter.
    };

    let sighash_type = TapSighashType::Default;
    let prevouts = vec![utxo];
    let prevouts = Prevouts::All(&prevouts);

    let mut sighasher = SighashCache::new(&mut unsigned_tx);

    let sighash = sighasher
        .taproot_key_spend_signature_hash(1, &prevouts, sighash_type)
        .expect("failed to construct sighash");

    let msg = Message::from(sighash); // message to sign

    let parsed_msg = msg.to_string();

    let msg_clone = parsed_msg.clone();
    let msg = parsed_msg.clone();

    let signed_statechain_id = sign_message(&statechain_id, &seckey).to_string();

    let get_nonce_res = statechain::get_nonce(&conn, statechain_id, &signed_statechain_id).await?;
    let server_pubnonce = get_nonce_res.server_nonce;

    let nonce_seed = [0xACu8; 32];

    let secnonce = SecNonce::build(nonce_seed)
        .with_seckey(seckey)
        .with_message(&parsed_msg)
        .with_aggregated_pubkey(*agg_pubkey)
        .with_extra_input(&(0 as u32).to_be_bytes())
        .build();

    let our_public_nonce = secnonce.public_nonce();

    let public_nonces = [
        our_public_nonce,
        server_pubnonce.parse::<PubNonce>().unwrap(),
    ];

    let agg_pubnonce: AggNonce = public_nonces.iter().sum();

    let agg_pubnonce_str = agg_pubnonce.to_string();

    let our_partial_signature: PartialSignature =
        musig2::sign_partial(&key_agg_ctx, seckey, secnonce, &agg_pubnonce, parsed_msg)
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

    let final_signature: [u8; 64] = musig2::aggregate_partial_signatures(
        &key_agg_ctx,
        &agg_pubnonce,
        partial_signatures,
        msg_clone,
    )
    .expect("error aggregating signatures");

    musig2::verify_single(*agg_pubkey, &final_signature, msg)
        .expect("aggregated signature must be valid");

    let signature = bitcoin::taproot::Signature {
        sig: Signature::from_slice(&final_signature.to_vec()).unwrap(),
        hash_ty: sighash_type,
    };

    *sighasher.witness_mut(0).unwrap() = Witness::p2tr_key_spend(&signature);


    let tx = sighasher.into_transaction();


    println!(
        "Backup transaction: {}",
        consensus::encode::serialize_hex(&tx)
    );

    Ok(unsigned_tx)
}

fn sign_message(msg: &str, seckey: &SecretKey) -> Signature {
    let secp = Secp256k1::new();

    // Convert the hash result to a slice
    let message = Message::from_hashed_data::<sha256::Hash>(msg.to_string().as_bytes());
    let keypair = Keypair::from_seckey_slice(&secp, seckey.as_ref()).unwrap();
    let signed_message = secp.sign_schnorr(&message, &keypair);

    signed_message
}
