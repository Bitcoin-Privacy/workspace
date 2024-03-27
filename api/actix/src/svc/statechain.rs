use std::str::FromStr;

use actix_web::{web::Data, Result};

use secp256k1::{rand, PublicKey, Secp256k1, SecretKey, XOnlyPublicKey};
use shared::intf::statechain::DepositRes;

use crate::repo::statechain::{StatechainRepo, TraitStatechainRepo};
pub async fn create_deposit(
    repo: &Data<StatechainRepo>,
    token_id: &str,
    auth_pubkey: &str,
    amount: u32,
) -> Result<DepositRes, String> {
    let auth_key = match PublicKey::from_str(&auth_pubkey) {
        Ok(key) => key,
        Err(err) => return Err(format!("Invalid auth public key: {}", err)),
    };
 
    let statechain_id = uuid::Uuid::new_v4().as_simple().to_string();

    let secp = Secp256k1::new();
    let secret_key = SecretKey::new(&mut rand::thread_rng());
    let pub_key = PublicKey::from_secret_key(&secp, &secret_key);

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
