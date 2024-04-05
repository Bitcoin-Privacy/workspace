use std::str::FromStr;

use actix_web::web::Data;
use anyhow::{anyhow, Result};
use bitcoin::{
    bip32::Xpub,
    consensus,
    hashes::Hash,
    hex::DisplayHex,
    key::{Keypair, TapTweak, TweakedKeypair, UntweakedPublicKey, Verification},
    secp256k1::{rand, Message, PublicKey, Secp256k1, SecretKey},
    sighash::{Prevouts, SighashCache},
    Amount, OutPoint, ScriptBuf, TapSighashType, Transaction, TxIn, TxOut, Txid,
};
use hex::ToHex;

use crate::repo::statechain::{StatechainRepo, TraitStatechainRepo};
use shared::{
    intf::statechain::{CreateBkTxnRes, DepositRes},
    model::Txn,
};

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
    scriptpubkey: &str,
    txn: &str,
) -> Result<CreateBkTxnRes> {
    let statecoin = repo.get_by_id(statechain_id).await?;

    let sk = SecretKey::from_str(&statecoin.server_private_key)?;
    let secp = Secp256k1::new();
    let keypair = Keypair::from_secret_key(&secp, &sk);

    let parsed_tx = consensus::deserialize::<Transaction>(&hex::decode(txn)?)?;

    let sighash_type = TapSighashType::Default;

    let mut unsigned_txn = parsed_tx.clone();
    let mut sighasher = SighashCache::new(&mut unsigned_txn);

    let input_index = 0;

    let secp = Secp256k1::new();

    let prevouts = vec![TxOut {
        value: Amount::from_sat(statecoin.amount as u64),
        script_pubkey: ScriptBuf::from_hex(scriptpubkey)?,
    }];
    let prevouts = Prevouts::All(&prevouts);

    let sighash = sighasher
        .taproot_key_spend_signature_hash(input_index, &prevouts, sighash_type)
        .expect("failed to construct sighash");

    let tweaked: TweakedKeypair = keypair.tap_tweak(&secp, None);
    let msg = Message::from(sighash);
    let signature = secp.sign_schnorr(&msg, &tweaked.to_inner());
    let signature = bitcoin::taproot::Signature {
        sig: signature,
        hash_ty: sighash_type,
    };

    let res = CreateBkTxnRes {
        sig: hex::encode(signature.to_vec()),
        rand_key: "".to_string(),
    };

    Ok(res)
}
