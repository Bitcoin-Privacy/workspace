use anyhow::Result;

use bitcoin::{hex::DisplayHex, secp256k1::SecretKey};
use bitcoin::{Address, Network, XOnlyPublicKey};

use secp256k1::{Keypair, PublicKey, Scalar, Secp256k1};
use std::str::FromStr;

use crate::svc::statechain::sign_message;
use crate::{api::statechain, db::PoolWrapper, model::AccountActions};
use ecies;
use shared::intf::statechain::{StatechainAddress, TransferMessage};

use crate::connector::NodeConnector;

use super::account;
use super::statechain::generate_auth_owner_keypairs;

pub async fn execute(
    conn: &NodeConnector,
    pool: &PoolWrapper,
    deriv: &str,
    transfer_message: &str,
    authkey: &str,
) -> Result<String> {
    let (account, _) = account::get_account(deriv).unwrap();
    let account_address = account.get_addr();
    let secp = Secp256k1::new();
    let (o2, auth_seckey) = match pool.get_seckey_by_authkey(&authkey).await? {
        Some(key) => key,
        None => panic!("No seckey"),
    };

    let parsed_transfer_msg = decrypt_transfer_msg(transfer_message, &auth_seckey)?;
    println!("transfer_message {:#?}", parsed_transfer_msg);
    let statechain_id = parsed_transfer_msg.statechain_id.clone();

    match verify_transfer_statecoin(&conn, &pool, &parsed_transfer_msg).await {
        Ok(s) => println!("{}", s),
        Err(e) => panic!("Invalid transfer message: {}", e),
    }

    let o2 = SecretKey::from_str(&o2)?;
    let negated_o2 = o2.negate();
    let t1 = parsed_transfer_msg.t1;

    let t1 = hex::decode(t1)?;
    let t1: [u8; 32] = t1.try_into().unwrap();

    let t1_scalar = Scalar::from_be_bytes(t1)?;

    let t2 = negated_o2.add_tweak(&t1_scalar)?;
    let t2_str = t2.secret_bytes().to_lower_hex_string();
    let signed_msg = sign_message(&t2_str, &o2).to_string();

    statechain::update_new_key(&conn, &t2_str, &signed_msg, &statechain_id, &authkey).await?;
    // pub struct TransferMessage {
    //     pub total_owner: u64,
    //     pub backup_txs: Vec<String>,
    //     pub t1: String,
    //     pub statechain_id: String,
    //     pub agg_pubkey: String,
    //     pub key_agg_ctx : String,
    //     pub funding_txid: String,
    //     pub funding_vout: u64,
    // }
    let auth_secret_key = SecretKey::from_str(&auth_seckey)?;

    let signed_statechain_id = sign_message(&statechain_id, &auth_secret_key);

    let aggregated_pubkey = PublicKey::from_str(&parsed_transfer_msg.agg_pubkey)?;

    let aggregated_address = Address::p2tr(
        &secp,
        aggregated_pubkey.x_only_public_key().0,
        None,
        Network::Testnet,
    );

    pool.update_unverifed_statecoin(
        &statechain_id,
        &signed_statechain_id.to_string(),
        parsed_transfer_msg.txn,
        0 as u64,
        &parsed_transfer_msg.key_agg_ctx,
        &parsed_transfer_msg.agg_pubkey,
        &aggregated_address.to_string(),
        &parsed_transfer_msg.funding_txid,
        parsed_transfer_msg.funding_vout,
        "test",
        parsed_transfer_msg.amount,
    )
    .await?;

    Ok("verify OK".to_string())
}

pub async fn verify_transfer_statecoin(
    conn: &NodeConnector,
    pool: &PoolWrapper,
    transfer_message: &TransferMessage,
) -> Result<String, anyhow::Error> {
    let bk_txs = &transfer_message.backup_txs;

    Ok("hehe".to_string())
}

pub async fn generate_statechain_address(pool: &PoolWrapper, deriv: &str) -> Result<String> {
    let (account, _) = account::get_account(deriv).unwrap();
    let account_address = account.get_addr();
    let statechain_keypairs = generate_auth_owner_keypairs()?;
    let authkey = statechain_keypairs.auth_pubkey.x_only_public_key().0;
    let statechain_address = StatechainAddress {
        owner_pubkey: statechain_keypairs.owner_pubkey.to_string(),
        authkey: statechain_keypairs.auth_pubkey.to_string(),
    };

    let address = serde_json::to_value(statechain_address)?.to_string();
    pool.create_unverified_statecoin(
        &account_address,
        &statechain_keypairs
            .auth_seckey
            .secret_bytes()
            .to_lower_hex_string(),
        &authkey.to_string(),
        &statechain_keypairs
            .owner_seckey
            .secret_bytes()
            .to_lower_hex_string(),
        &statechain_keypairs.owner_pubkey.to_string(),
    )
    .await?;
    Ok(hex::encode(address))
}

pub fn decrypt_transfer_msg(encrypted_message: &str, auth_seckey: &str) -> Result<TransferMessage> {
    let auth_seckey = SecretKey::from_str(auth_seckey)?;

    let decoded_enc_message = hex::decode(encrypted_message)?;

    let decrypted_msg = ecies::decrypt(
        auth_seckey.secret_bytes().as_slice(),
        decoded_enc_message.as_slice(),
    )
    .unwrap();

    let decrypted_msg_str = String::from_utf8(decrypted_msg)?;

    let transfer_msg: TransferMessage = serde_json::from_str(decrypted_msg_str.as_str())?;

    Ok(transfer_msg)
}
