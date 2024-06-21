use std::{
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use actix_web::web::Data;
use anyhow::Result;
use bitcoin::{
    absolute::LockTime,
    consensus,
    hashes::sha256,
    hex::{Case, DisplayHex},
    secp256k1::{rand, PublicKey, Secp256k1, SecretKey},
    sighash::{Prevouts, SighashCache},
    Amount, ScriptBuf, TapSighashType, Transaction, TxOut, XOnlyPublicKey,
};
use musig2::{AggNonce, BinaryEncoding, KeyAggContext, PartialSignature, SecNonce};
use rand::RngCore;
use secp256k1::{schnorr::Signature, Message, Parity};

use crate::repo::statechain::{StatechainRepo, TraitStatechainRepo};
use shared::{
    intf::statechain::{
        DepositRes, GetNonceRes, GetPartialSignatureRes, GetTransferMessageRes, KeyRegisterRes,
        UpdateKeyRes, VerifyStatecoinRes,
    },
    model::Status,
};

pub async fn create_deposit(
    repo: &Data<StatechainRepo>,
    token_id: &str,
    auth_pubkey: &str,
    amount: u32,
) -> Result<DepositRes, String> {
    println!("Auth pubkey {}", auth_pubkey);
    let auth_key = match XOnlyPublicKey::from_str(auth_pubkey) {
        Ok(key) => key,
        Err(err) => return Err(format!("Invalid auth public key: {}", err)),
    };

    let secp = Secp256k1::new();
    let mut seckey = SecretKey::new(&mut rand::thread_rng());
    let (_, parity) = PublicKey::from_secret_key(&secp, &seckey).x_only_public_key();

    if parity == Parity::Odd {
        seckey = seckey.negate();
    }

    let pubkey = PublicKey::from_secret_key(&secp, &seckey);
    let current_time = SystemTime::now();

    // Calculate the Unix time by subtracting the UNIX epoch time
    let current_unix_time = current_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let init_nlock_time = current_unix_time + 60 * 60 * 24 * 30 * 4;
    println!(
        "current time, nlocktime, {}, {}",
        current_unix_time, init_nlock_time
    );
    let statecoin = repo
        .create_deposit_tx(
            token_id,
            &auth_key,
            &pubkey,
            &seckey,
            amount,
            init_nlock_time,
        )
        .await
        .map_err(|e| format!("Failed to add deposit: {}", e))?;

    let res = DepositRes {
        se_pubkey_1: pubkey.to_string(),
        statechain_id: statecoin.id.to_string(),
    };
    Ok(res)
}

pub async fn get_nonce(repo: &Data<StatechainRepo>, statechain_id: &str) -> Result<GetNonceRes> {
    let mut nonce_seed = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut nonce_seed);
    let secnonce = musig2::SecNonceBuilder::new(nonce_seed).build();
    let pubnonce = secnonce.public_nonce();
    repo.update_nonce(&secnonce.to_bytes().to_lower_hex_string(), statechain_id)
        .await?;

    Ok(GetNonceRes {
        server_nonce: pubnonce.to_string(),
    })
}

pub async fn get_sig(
    repo: &Data<StatechainRepo>,
    serialized_key_agg_ctx: &str,
    statechain_id: &str,
    parsed_tx: &str,
    agg_pubnonce: &str,
    script_pubkey: &str,
) -> Result<GetPartialSignatureRes> {
    let statecoin = repo.get_by_id(statechain_id).await?;
    let secnonce = statecoin.sec_nonce.unwrap();
    let seckey = SecretKey::from_str(&statecoin.server_private_key)?;
    let secnonce = SecNonce::from_hex(&secnonce).unwrap();
    let key_agg_ctx = KeyAggContext::from_hex(serialized_key_agg_ctx).unwrap();
    let agg_nonce = AggNonce::from_str(agg_pubnonce).unwrap();
    let sighash_type = TapSighashType::Default;
    let n_lock_time = statecoin.n_lock_time;
    let txn = statecoin.txn as u64;

    let new_lock_time = n_lock_time - txn * 60 * 60 * 24 * 1;

    let tx = consensus::deserialize::<Transaction>(&hex::decode(parsed_tx)?)?;
    let mut unsigned_txn = tx.clone();

    unsigned_txn.lock_time = LockTime::from_time(new_lock_time as u32)?;
    let mut sighasher = SighashCache::new(&mut unsigned_txn);
    let input_index = 0;

    let prevouts = vec![TxOut {
        value: Amount::from_sat(statecoin.amount as u64),
        script_pubkey: ScriptBuf::from_hex(script_pubkey)?,
    }];
    let prevouts = Prevouts::All(&prevouts);

    let sighash = sighasher
        .taproot_key_spend_signature_hash(input_index, &prevouts, sighash_type)
        .expect("failed to construct sighash");

    let sighash_str = sighash.to_string();
    let msg = Message::from(sighash);
    let msg = msg.as_ref();

    let our_partial_signature: PartialSignature =
        musig2::sign_partial(&key_agg_ctx, seckey, secnonce, &agg_nonce, msg)?;

    let final_sig = our_partial_signature.serialize().to_hex_string(Case::Lower);

    Ok(GetPartialSignatureRes {
        sighash: sighash_str,
        partial_sig: final_sig,
        n_lock_time: new_lock_time,
    })
}

pub async fn withdraw(
    repo: &Data<StatechainRepo>,
    serialized_key_agg_ctx: &str,
    statechain_id: &str,
    parsed_tx: &str,
    agg_pubnonce: &str,
    script_pubkey: &str,
) -> Result<GetPartialSignatureRes> {
    let statecoin = repo.get_by_id(statechain_id).await?;
    let secnonce = statecoin.sec_nonce.unwrap();
    let seckey = SecretKey::from_str(&statecoin.server_private_key)?;
    let secnonce = SecNonce::from_hex(&secnonce).unwrap();
    let key_agg_ctx = KeyAggContext::from_hex(serialized_key_agg_ctx).unwrap();
    let agg_nonce = AggNonce::from_str(agg_pubnonce).unwrap();
    let sighash_type = TapSighashType::Default;

    let tx = consensus::deserialize::<Transaction>(&hex::decode(parsed_tx)?)?;
    let mut unsigned_txn = tx.clone();
    let mut sighasher = SighashCache::new(&mut unsigned_txn);
    let input_index = 0;

    let prevouts = vec![TxOut {
        value: Amount::from_sat(statecoin.amount as u64),
        script_pubkey: ScriptBuf::from_hex(script_pubkey)?,
    }];
    let prevouts = Prevouts::All(&prevouts);

    let sighash = sighasher
        .taproot_key_spend_signature_hash(input_index, &prevouts, sighash_type)
        .expect("failed to construct sighash");

    let sighash_str = sighash.to_string();
    let msg = Message::from(sighash);
    let msg = msg.as_ref();

    let our_partial_signature: PartialSignature =
        musig2::sign_partial(&key_agg_ctx, seckey, secnonce, &agg_nonce, msg)?;

    let final_sig = our_partial_signature.serialize().to_hex_string(Case::Lower);

    //_ = repo.delete_statecoin_by_id(statechain_id).await?;

    Ok(GetPartialSignatureRes {
        sighash: sighash_str,
        partial_sig: final_sig,
        n_lock_time: 0_u64,
    })
}

pub async fn register_key(
    repo: &Data<StatechainRepo>,
    statechain_id: &str,
    auth_pubkey_2: &str,
) -> Result<KeyRegisterRes> {
    let secp = Secp256k1::new();
    let mut x1 = SecretKey::new(&mut rand::thread_rng());
    let (_, parity) = PublicKey::from_secret_key(&secp, &x1).x_only_public_key();

    if parity == Parity::Odd {
        x1 = x1.negate();
    }

    let x1_point = PublicKey::from_secret_key(&secp, &x1);
    let x1_point = x1_point.to_string();

    let parsed_x1 = x1.secret_bytes().to_lower_hex_string();

    println!("X1 : {}, {}", parsed_x1, x1_point);

    repo.create_statechain_transfer(statechain_id, auth_pubkey_2, &parsed_x1, &x1_point)
        .await?;

    Ok(KeyRegisterRes {
        random_key: parsed_x1,
    })
}

pub async fn create_transfer_message(
    repo: &Data<StatechainRepo>,
    authkey: &str,
    transfer_msg: &str,
) -> Result<Status, String> {
    repo.update_transfer_message(authkey, transfer_msg)
        .await
        .map_err(|e| format!("Failed to update_tranfer_message: {}", e))?;

    Ok(Status { confirmed: true })
}

pub async fn get_tranfer_message(
    repo: &Data<StatechainRepo>,
    authkey: &str,
) -> Result<GetTransferMessageRes, String> {
    let msg = repo
        .get_transfer_message(authkey)
        .await
        .map_err(|e| format!("Failed to get transfer message: {}", e))?;

    Ok(GetTransferMessageRes {
        transfer_message: msg,
    })
}

pub async fn verify_signature(
    repo: &Data<StatechainRepo>,
    signature: &str,
    statechain_id: &str,
) -> Result<bool> {
    let auth_key = repo.get_auth_key_by_statechain_id(statechain_id).await?;

    let pub_key = XOnlyPublicKey::from_str(&auth_key)?;
    let signed_message = Signature::from_str(signature).unwrap();
    let msg = Message::from_hashed_data::<sha256::Hash>(statechain_id.to_string().as_bytes());

    let secp = Secp256k1::new();
    Ok(secp.verify_schnorr(&signed_message, &msg, &pub_key).is_ok())
}

pub async fn verify_receiver(
    repo: &Data<StatechainRepo>,
    signature: &str,
    statechain_id: &str,
) -> Result<bool, anyhow::Error> {
    let auth_key = repo
        .get_auth_key_transfer_by_statechain_id(statechain_id)
        .await?;

    let pub_key = XOnlyPublicKey::from_str(&auth_key.authkey)?;
    let signed_message = Signature::from_str(signature).unwrap();
    let msg = Message::from_hashed_data::<sha256::Hash>(statechain_id.to_string().as_bytes());

    let secp = Secp256k1::new();
    Ok(secp.verify_schnorr(&signed_message, &msg, &pub_key).is_ok())
}

pub async fn verify_statecoin(
    repo: &Data<StatechainRepo>,
    statechain_id: &str,
) -> Result<VerifyStatecoinRes, String> {
    let info = repo
        .get_verify_statecoin(statechain_id)
        .await
        .map_err(|e| format!("Failed to get transfer message: {}", e))?;

    Ok(VerifyStatecoinRes {
        txn: info.txn,
        server_pubkey: info.server_public_key,
        random_point: info.random_point,
    })
}

pub async fn update_key(
    repo: &Data<StatechainRepo>,
    authkey: &str,
    statechain_id: &str,
    t2: &str,
) -> Result<UpdateKeyRes> {
    // let secrets = repo
    //     .get_seckey_and_random_by_statechain_id(&statechain_id)
    //     .await?;

    // let s1 = SecretKey::from_str(&secrets.server_private_key)?;

    // let t2 = hex::decode(t2)?;

    // let t2: [u8; 32] = t2.try_into().unwrap();

    // let t2 = Scalar::from_be_bytes(t2)?;

    // let x1 = SecretKey::from_str(&secrets.random_key)?;

    // let negated_x1 = x1.negate();

    // let t2_negate_x1 = negated_x1.add_tweak(&t2)?.secret_bytes();

    // let t2_negate_x1_scalar = Scalar::from_be_bytes(t2_negate_x1)?;

    // let s2 = s1.add_tweak(&t2_negate_x1_scalar)?;
    // let secp = Secp256k1::new();

    // let new_owner_pubkey = PublicKey::from_secret_key(&secp, &s2);

    repo.update_new_owner(
        statechain_id,
        authkey,
        // &s2.secret_bytes().to_lower_hex_string(),
        // &new_owner_pubkey.to_string(),
    )
    .await?;

    repo.delete_statecoin_transfer(authkey).await?;

    Ok(UpdateKeyRes { status: 1 })
}
