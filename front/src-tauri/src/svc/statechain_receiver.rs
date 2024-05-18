use anyhow::Result;

use bitcoin::hex::parse;
use bitcoin::{hex::DisplayHex, secp256k1::SecretKey};
use bitcoin::{Address, Network, XOnlyPublicKey};

use secp256k1::{Keypair, PublicKey, Scalar, Secp256k1};
use std::str::FromStr;

use crate::model::Statecoin;
use crate::svc::statechain::sign_message;
use crate::svc::statechain_sender::create_bk_tx_for_receiver;
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
    let account_address = Address::from_str(&account_address)?;
    let account_address = account_address.require_network(Network::Testnet)?;
    let secp = Secp256k1::new();
    let (owner_seckey, auth_seckey) = match pool.get_seckey_by_authkey(&authkey).await? {
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

    //  let o2 = SecretKey::from_str(&o2)?;
    let auth_seckey = SecretKey::from_str(&auth_seckey)?;
    // let negated_o2 = o2.negate();
    let x1 = parsed_transfer_msg.x1;

    // let t1 = hex::decode(t1)?;
    // let t1: [u8; 32] = t1.try_into().unwrap();
    // let t1_scalar = Scalar::from_be_bytes(t1)?;
    // let t2 = negated_o2.add_tweak(&t1_scalar)?;
    // let t2_str = t2.secret_bytes().to_lower_hex_string();

    // println!("t2 : {}", t2_str);
    //let signed_msg = sign_message(&t2_str, &o2).to_string();
    let signed_msg = sign_message(&x1, &auth_seckey).to_string();

    let updatekey_res =
        statechain::update_new_key(&conn, &x1, &signed_msg, &statechain_id, &authkey).await?;
    //let auth_secret_key = SecretKey::from_str(&auth_seckey)?;

    let signed_statechain_id = sign_message(&statechain_id, &auth_seckey);

    let aggregated_pubkey = PublicKey::from_str(&parsed_transfer_msg.agg_pubkey)?;

    let aggregated_address = Address::p2tr(
        &secp,
        aggregated_pubkey.x_only_public_key().0,
        None,
        Network::Testnet,
    );

    // pub struct Statecoin {
    //     pub tx_n: i64,
    //     pub owner_seckey: String,
    //     pub signed_statechain_id: String,
    //     pub aggregated_pubkey: String,
    //     pub aggregated_address: String,
    //     pub funding_txid: String,
    //     pub funding_vout: i64,
    //     pub key_agg_ctx: String,
    //     pub amount: i64,
    //     pub account: String,
    //     pub spend_key: String,
    // }

    // pub struct TransferMessage {
    //     pub txn: u64,
    //     pub backup_txs: String,
    //     pub x1: String,
    //     pub statechain_id: String,
    //     pub agg_pubkey: String,
    //     pub key_agg_ctx: String,
    //     pub funding_txid: String,
    //     pub funding_vout: u64,
    //     pub amount: u64,
    //     pub spend_key : String,
    // }

    let statecoin = Statecoin {
        tx_n: parsed_transfer_msg.txn as i64,
        signed_statechain_id: signed_statechain_id.to_string(),
        aggregated_pubkey: parsed_transfer_msg.agg_pubkey,
        funding_txid: parsed_transfer_msg.funding_txid,
        funding_vout: parsed_transfer_msg.funding_vout as i64,
        key_agg_ctx: parsed_transfer_msg.key_agg_ctx,
        amount: parsed_transfer_msg.amount as i64,
        spend_key: parsed_transfer_msg.spend_key,
    };

    let bk = create_bk_tx_for_receiver(conn, &statechain_id, &statecoin, &account_address).await?;
    println!("NEW backup transaction : {}", bk);

    pool.update_unverifed_statecoin(
        &statechain_id,
        &statecoin,
        &bk,
        &authkey,
        &aggregated_address.to_string(),
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

    Ok("Message : verify ok".to_string())
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
        &authkey.to_string(),
        &statechain_keypairs
            .auth_seckey
            .secret_bytes()
            .to_lower_hex_string(),
        &statechain_keypairs.owner_pubkey.to_string(),
        &statechain_keypairs
            .owner_seckey
            .secret_bytes()
            .to_lower_hex_string(),
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
