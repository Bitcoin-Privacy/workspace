use anyhow::Result;
use bitcoin::{
    absolute::LockTime,
    consensus,
    hex::DisplayHex,
    secp256k1::{rand, PublicKey, Secp256k1, SecretKey},
    transaction, Address, Amount, Network, OutPoint, ScriptBuf, Sequence, TapSighash,
    TapSighashType, Transaction, TxIn, TxOut, Txid, Witness,
};
use ecies;
use musig2::{AggNonce, BinaryEncoding, KeyAggContext, PartialSignature, PubNonce, SecNonce};
use rand::RngCore;
use secp256k1::Message;
use serde_json::json;

use std::str::FromStr;

use crate::{api::statechain, cfg::BASE_TX_FEE, db::PoolWrapper, model::Statecoin};
use shared::intf::statechain::TransferMessage;

use crate::connector::NodeConnector;

pub async fn execute(
    conn: &NodeConnector,
    pool: &PoolWrapper,
    pubkey: &str,
    auth_publickey: &str,
    statechain_id: &str,
) -> Result<String> {
    //1. generate tx2 by key

    let b_pubkey = bitcoin::PublicKey::from_str(pubkey)?;
    let receiver_address = Address::p2wpkh(&b_pubkey, Network::Testnet)?;
    let statecoin = pool.get_statecoin_by_id(statechain_id).await?;
    let authkey = &auth_publickey[2..];
    // let tx =
    //     create_bk_tx_for_receiver(&conn, &statechain_id, &statecoin, &receiver_address).await?;

    //2. send register new owner

    let register_new_owner_res = statechain::register_new_owner(
        conn,
        statechain_id,
        &statecoin.signed_statechain_id,
        authkey,
    )
    .await?;
    let x1 = register_new_owner_res.random_key;
    // println!("x1 {}", x1);
    // let x1 = hex::decode(x1)?;
    // let x1: [u8; 32] = x1.try_into().unwrap();
    // let x1 = Scalar::from_be_bytes(x1)?;

    // //3.compute t1

    // let t1 = compute_t1(&SecretKey::from_str(&statecoin.owner_seckey)?, &x1);

    //3. create transfer message
    let transfer_message = TransferMessage {
        txn: statecoin.tx_n as u64,
        backup_txs: statecoin.bk_tx,
        x1,
        statechain_id: statechain_id.to_string(),
        agg_pubkey: statecoin.aggregated_pubkey.to_string(),
        key_agg_ctx: statecoin.key_agg_ctx.to_string(),
        funding_txid: statecoin.funding_txid.to_string(),
        funding_vout: statecoin.funding_vout as u64,
        amount: statecoin.amount as u64,
        spend_key: statecoin.spend_key,
    };

    let encrypted_msg_string = encrypt_transfer_message(&transfer_message, auth_publickey)?;
    statechain::create_transfer_msg(conn, &encrypted_msg_string, authkey).await?;
    println!("delete id : {}", statechain_id);
    pool.delete_statecoin_by_statechain_id(statechain_id)
        .await?;
    Ok("send success".to_string())
}

pub async fn create_bk_tx_for_receiver(
    conn: &NodeConnector,
    statechain_id: &str,
    statecoin: &Statecoin,
    receiver_address: &Address,
) -> Result<String> {
    let amount = statecoin.amount as u64;
    let agg_pubkey = PublicKey::from_str(&statecoin.aggregated_pubkey)?;
    let vout = 0_i64;
    let key_agg_ctx = KeyAggContext::from_hex(&statecoin.key_agg_ctx).unwrap();
    let secp = Secp256k1::new();
    let seckey = &statecoin.spend_key;
    let seckey = SecretKey::from_str(seckey).unwrap();
    let agg_scriptpubkey = ScriptBuf::new_p2tr(&secp, agg_pubkey.x_only_public_key().0, None);
    let scriptpubkey = agg_scriptpubkey.to_hex_string();

    let prev_outpoint = OutPoint {
        txid: Txid::from_str(&statecoin.funding_txid)?,
        vout: vout as u32,
    };

    let input = TxIn {
        previous_output: prev_outpoint,
        script_sig: ScriptBuf::default(),
        sequence: Sequence(0xFFFFFFFE),
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

    let sighash_type = TapSighashType::Default;
    let get_nonce_res =
        statechain::get_nonce(conn, statechain_id, &statecoin.signed_statechain_id).await?;
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

    let serialized_key_agg_ctx = key_agg_ctx
        .to_bytes()
        .to_hex_string(bitcoin::hex::Case::Lower);

    let unsigned_tx_hex = consensus::encode::serialize_hex(&unsigned_tx);

    let get_sign_res = statechain::get_partial_signature(
        conn,
        &serialized_key_agg_ctx,
        statechain_id,
        &statecoin.signed_statechain_id,
        &unsigned_tx_hex,
        &agg_pubnonce_str,
        &scriptpubkey,
    )
    .await?;

    unsigned_tx.lock_time = LockTime::from_time(get_sign_res.n_lock_time as u32)?;

    let sighash = &get_sign_res.sighash;
    let sighash = TapSighash::from_str(sighash)?;
    let msg = Message::from(sighash);
    let msg = msg.as_ref();
    let our_partial_signature: PartialSignature =
        musig2::sign_partial(&key_agg_ctx, seckey, secnonce, &agg_pubnonce, msg)
            .expect("error creating partial signature");

    let server_signature = get_sign_res.partial_sig;

    let partial_signatures = [
        our_partial_signature,
        PartialSignature::from_hex(&server_signature).unwrap(),
    ];

    let agg_pubkey_tw: PublicKey = key_agg_ctx.aggregated_pubkey();
    println!("tx tweaked public key : {}", agg_pubkey_tw);

    for (i, partial_signature) in partial_signatures.into_iter().enumerate() {
        if i == 0 {
            // Don't bother verifying our own signature
            continue;
        }

        let their_pubkey: PublicKey = key_agg_ctx.get_pubkey(i).unwrap();
        let their_pubnonce = &public_nonces[i];

        musig2::verify_partial(
            &key_agg_ctx,
            partial_signature,
            &agg_pubnonce,
            their_pubkey,
            their_pubnonce,
            msg,
        )
        .expect("received invalid signature from a peer");
    }

    let final_signature: secp256k1::schnorr::Signature =
        musig2::aggregate_partial_signatures(&key_agg_ctx, &agg_pubnonce, partial_signatures, msg)
            .expect("error aggregating signatures");

    musig2::verify_single(agg_pubkey_tw, final_signature, msg)
        .expect("aggregated signature must be valid");

    let signature = bitcoin::taproot::Signature {
        sig: final_signature,
        hash_ty: sighash_type,
    };

    let mut wit = Witness::new();
    wit.push(signature.to_vec());

    unsigned_tx.input[0].witness = wit;

    let tx_hex = consensus::encode::serialize_hex(&unsigned_tx);
    Ok(tx_hex)
}

pub fn encrypt_transfer_message(
    transfer_message: &TransferMessage,
    auth_publickey: &str,
) -> Result<String> {
    let transfer_msg_json = json!(&transfer_message);

    let transfer_msg_json_str = serde_json::to_string_pretty(&transfer_msg_json).unwrap();

    let msg = transfer_msg_json_str.as_bytes();

    let auth_pubkey = PublicKey::from_str(auth_publickey)?;

    let serialized_new_auth_pubkey = auth_pubkey.serialize();
    let encrypted_msg = ecies::encrypt(&serialized_new_auth_pubkey, msg).unwrap();

    let encrypted_msg_string = hex::encode(encrypted_msg);

    println!("encrypted transfer message : {}", encrypted_msg_string);
    Ok(encrypted_msg_string)
}
