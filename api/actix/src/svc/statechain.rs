use std::str::FromStr;

use actix_web::web::Data;
use anyhow::Result;
use bitcoin::{
    hashes::sha256, hex::{Case, DisplayHex}, key::{Keypair, TapTweak, TweakedKeypair}, secp256k1::{rand, PublicKey, Secp256k1, SecretKey}, TapSighash, XOnlyPublicKey
};
use musig2::{AggNonce, BinaryEncoding, KeyAggContext, PartialSignature, SecNonce};
use rand::RngCore;
use secp256k1::{schnorr::Signature, Message, Scalar};

use crate::repo::statechain::{StatechainRepo, TraitStatechainRepo};
use shared::intf::statechain::{
    CreateBkTxnRes, DepositRes, GetNonceRes, GetPartialSignatureRes, GetTransferMessageRes,
    KeyRegisterRes, UpdateKeyRes, VerifyStatecoinRes,
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
    let keypair = Keypair::new(&secp, &mut rand::thread_rng());
    let tweaked_keypair = keypair.tap_tweak(&secp, None);
    let seckey = SecretKey::from_keypair(&tweaked_keypair.to_inner());
    let pubkey = PublicKey::from_keypair(&tweaked_keypair.to_inner());

    // let nonce_seed = [0xACu8; 32];
    // let secnonce = musig2::SecNonceBuilder::new(nonce_seed).build();

    // let pubnonce = secnonce.public_nonce();

    let statecoin = repo
        .create_deposit_tx(token_id, &auth_key, &pubkey, &seckey, amount)
        .await
        .map_err(|e| format!("Failed to add deposit: {}", e))?;

    let res = DepositRes {
        se_pubkey_1: pubkey.to_string(),
        statechain_id: statecoin.id.to_string(),
    };

    Ok(res)
}

pub async fn create_bk_txn(
    repo: &Data<StatechainRepo>,
    statechain_id: &str,
    scriptpubkey: &str,
    txn: &str,
) -> Result<CreateBkTxnRes> {
    let statecoin = repo.get_by_id(statechain_id).await?;

    let sk = SecretKey::from_str(&statecoin.server_private_key)?;
    let secp = Secp256k1::new();
    let keypair = Keypair::from_secret_key(&secp, &sk);

    // let parsed_tx = consensus::deserialize::<Transaction>(&hex::decode(txn)?)?;

    let sighash = TapSighash::from_str(txn)?;

    // let sighash_type = TapSighashType::Default;

    // let mut unsigned_txn = parsed_tx.clone();
    // let mut sighasher = SighashCache::new(&mut unsigned_txn);

    // let input_index = 0;

    // let secp = Secp256k1::new();

    // let prevouts = vec![TxOut {
    //     value: Amount::from_sat(statecoin.amount as u64),
    //     script_pubkey: ScriptBuf::from_hex(scriptpubkey)?,
    // }];
    // let prevouts = Prevouts::All(&prevouts);

    // let sighash = sighasher
    //     .taproot_key_spend_signature_hash(input_index, &prevouts, sighash_type)
    //     .expect("failed to construct sighash");

    let tweaked: TweakedKeypair = keypair.tap_tweak(&secp, None);
    let msg = Message::from(sighash);

    let signature = secp.sign_schnorr(&msg, &tweaked.to_inner());

    // let signature = bitcoin::taproot::Signature {
    //     sig: signature,
    //     hash_ty: sighash_type,
    // };

    let res = CreateBkTxnRes {
        sig: signature.to_string(),
        rand_key: "".to_string(),
    };

    Ok(res)
}

pub async fn get_nonce(repo: &Data<StatechainRepo>, statechain_id: &str) -> Result<GetNonceRes> {
    let mut nonce_seed = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut nonce_seed);

    let secnonce = musig2::SecNonceBuilder::new(nonce_seed).build();
    let pubnonce = secnonce.public_nonce();
    repo.update_nonce(&secnonce.to_bytes().to_lower_hex_string(), &statechain_id)
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
) -> Result<GetPartialSignatureRes> {
    // if !verify_signature(&repo, &signed_statechain_id, &statechain_id).await? {
    //     bail!("Invalid signature")
    // }

    let statecoin = repo.get_by_id(statechain_id).await?;

    println!("messsagee : {}", parsed_tx);

    let secnonce = statecoin.sec_nonce.unwrap();
    println!("nonce 2 : {}", secnonce);
    let seckey = SecretKey::from_str(&statecoin.server_private_key)?;
    let secnonce = SecNonce::from_hex(&secnonce).unwrap();

    let key_agg_ctx = KeyAggContext::from_hex(serialized_key_agg_ctx).unwrap();

    println!(
        "agg-ctx and pubnonce {},{}",
        serialized_key_agg_ctx, agg_pubnonce
    );

    let agg_nonce = AggNonce::from_str(agg_pubnonce).unwrap();

    let our_partial_signature: PartialSignature =
        musig2::sign_partial(&key_agg_ctx, seckey, secnonce, &agg_nonce, parsed_tx)?;

    let final_sig = our_partial_signature.serialize().to_hex_string(Case::Lower);

    Ok(GetPartialSignatureRes {
        partial_signature: final_sig,
    })
}

pub async fn register_key(
    repo: &Data<StatechainRepo>,
    statechain_id: &str,
    auth_pubkey_2: &str,
) -> Result<KeyRegisterRes> {
    let x1 = Scalar::random();

    let parsed_x1 = x1.to_be_bytes().to_lower_hex_string();

    repo.create_statechain_transfer(statechain_id, auth_pubkey_2, &parsed_x1)
        .await?;
    println!("register key, randowm :{}", parsed_x1);
    Ok(KeyRegisterRes {
        random_key: parsed_x1,
    })
}

pub async fn update_tranfer_message(
    repo: &Data<StatechainRepo>,
    authkey: &str,
    transfer_msg: &str,
) -> Result<(), String> {
    repo.update_transfer_message(authkey, transfer_msg)
        .await
        .map_err(|e| format!("Failed to update_tranfer_message: {}", e))?;

    Ok(())
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
) -> Result<bool, anyhow::Error> {
    let auth_key = repo.get_auth_key_by_statechain_id(&statechain_id).await?;

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
        .get_auth_key_transfer_by_statechain_id(&statechain_id)
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
    let secrets = repo
        .get_seckey_and_random_by_statechain_id(&statechain_id)
        .await?;

    let s1 = SecretKey::from_str(&secrets.server_private_key)?;

    let t2 = hex::decode(t2)?;

    let t2: [u8; 32] = t2.try_into().unwrap();

    let t2 = Scalar::from_be_bytes(t2)?;

    let x1 = SecretKey::from_str(&secrets.random_key)?;

    let negated_x1 = x1.negate();

    let t2_negate_x1 = negated_x1.add_tweak(&t2)?.secret_bytes();

    let t2_negate_x1_scalar = Scalar::from_be_bytes(t2_negate_x1)?;

    let s2 = s1.add_tweak(&t2_negate_x1_scalar)?;
    let secp = Secp256k1::new();
    let new_owner_pubkey = PublicKey::from_secret_key(&secp, &s2);

    repo.update_new_owner(
        statechain_id,
        authkey,
        &s2.secret_bytes().to_lower_hex_string(),
        &new_owner_pubkey.to_string(),
    )
    .await?;

    //repo.delete_statecoin_transfer(authkey).await?;

    Ok(UpdateKeyRes { status: 1 })
}
