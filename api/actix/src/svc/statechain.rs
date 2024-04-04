use anyhow::Result;
use bitcoin::{bip32::Xpub, consensus, Transaction};

use std::str::FromStr;

use actix_web::web::Data;

use bitcoin::secp256k1::{rand, PublicKey, Secp256k1, SecretKey};
use shared::intf::statechain::{CreateBkTxnRes, DepositRes};

use crate::repo::statechain::{StatechainRepo, TraitStatechainRepo};

pub async fn create_deposit(
    repo: &Data<StatechainRepo>,
    token_id: &str,
    auth_pubkey: &str,
    amount: u32,
) -> Result<DepositRes, String> {
    println!("Auth pubkey {}", auth_pubkey);
    let auth_key = match Xpub::from_str(auth_pubkey) {
        Ok(key) => key,
        Err(err) => return Err(format!("Invalid auth public key: {}", err)),
    };

    let secp = Secp256k1::new();
    let secret_key = SecretKey::new(&mut rand::thread_rng());
    let pub_key = PublicKey::from_secret_key(&secp, &secret_key);

    let statecoin = repo
        .create_deposit_tx(
            token_id,
            &auth_key.to_pub().inner,
            &pub_key,
            &secret_key,
            amount,
        )
        .await
        .map_err(|e| format!("Failed to add deposit: {}", e))?;

    let res = DepositRes {
        se_pubkey_1: pub_key.to_string(),
        statechain_id: statecoin.id.to_string(),
    };

    Ok(res)
}

pub async fn create_bk_txn(
    repo: &Data<StatechainRepo>,
    statechain_id: &str,
    txn: &str,
) -> Result<CreateBkTxnRes> {
    let statecoin = repo.get_statecoin(statechain_id).await?;

    let privkey = SecretKey::from_str(&statecoin.server_private_key)?;

    let parsed_tx = consensus::deserialize::<Transaction>(&hex::decode(txn)?)?;

    let res = CreateBkTxnRes {
        signed_txn_bk: txn.to_string(),
        rand_key: "".to_string(),
    };

    Ok(res)
}
