use std::str::FromStr;

use actix_web::{web::Data, Result};

use bitcoin::hashes::sha256;
use openssl::pkey::Public;
use secp256k1::{
    rand, schnorr::Signature, Keypair, Message, PublicKey, Secp256k1, SecretKey, XOnlyPublicKey,
};
use shared::intf::statechain::DepositRes;

use crate::repo::statechain::{StatechainRepo, TraitStatechainRepo};
pub async fn create_deposit(
    repo: &Data<StatechainRepo>,
    token_id: &str,
    auth_pubkey: &str,
    amount: u32,
) -> Result<DepositRes, String> {
    let auth_key = match XOnlyPublicKey::from_str(&auth_pubkey) {
        Ok(key) => key,
        Err(err) => return Err(format!("Invalid auth public key: {}", err)),
    };

    let statechain_id = uuid::Uuid::new_v4().as_simple().to_string();

    let secp = Secp256k1::new();
    let (secret_key, pub_key) = secp.generate_keypair(&mut rand::thread_rng());

    repo.create_deposit_tx(
        &token_id,
        &auth_key,
        &pub_key,
        &secret_key,
        &statechain_id,
        amount,
    )
    .await
    .map_err(|e| format!("Failed to add deposit: {}", e))?;

    let res = DepositRes {
        se_pubkey_1: pub_key.to_string(),
        statechain_id: statechain_id,
    };

    Ok(res)
}

pub async fn verify_signature(
    repo: &Data<StatechainRepo>,
    sign_message_hex: &str,
    statechain_id: &str,
) -> bool {
    let auth_key = repo
        .get_auth_key_by_statechain_id(&statechain_id)
        .await
        .unwrap();

    let pub_key = XOnlyPublicKey::from_str(&auth_key).unwrap();
    let signed_message = Signature::from_str(sign_message_hex).unwrap();
    let msg = Message::from_hashed_data::<sha256::Hash>(statechain_id.to_string().as_bytes());

    let secp = Secp256k1::new();
    secp.verify_schnorr(&signed_message, &msg, &pub_key).is_ok()
}
