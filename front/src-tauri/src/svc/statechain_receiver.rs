use anyhow::{anyhow, Result};

use bitcoin::{consensus, Address, Network, Transaction};
use bitcoin::{hex::DisplayHex, secp256k1::SecretKey};

use openssl::sha::Sha256;
use secp256k1::{PublicKey, Secp256k1};
use shared::api::get_transaction_existence;
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
    let auth_seckey = SecretKey::from_str(&auth_seckey)?;

    let statechain_id = parsed_transfer_msg.statechain_id.clone();

    let signed_id = sign_message(&statechain_id, &auth_seckey).to_string();

    match verify_transfer_statecoin(conn, pool, &parsed_transfer_msg, &signed_id).await {
        Ok(s) => println!("{}", s),
        Err(e) => panic!("Invalid transfer message: {}", e),
    }

    let x1 = parsed_transfer_msg.x1;

    let signed_msg = sign_message(&x1, &auth_seckey).to_string();

    let updatekey_res =
        statechain::update_new_key(conn, &x1, &signed_msg, &statechain_id, authkey).await?;
    
    let signed_statechain_id = sign_message(&statechain_id, &auth_seckey);

    let aggregated_pubkey = PublicKey::from_str(&parsed_transfer_msg.agg_pubkey)?;

    let aggregated_address = Address::p2tr(
        &secp,
        aggregated_pubkey.x_only_public_key().0,
        None,
        Network::Testnet,
    );

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

    pool.update_unverifed_statecoin(
        &statechain_id,
        &statecoin,
        &bk,
        authkey,
        &aggregated_address.to_string(),
    )
    .await?;

    Ok("verify OK".to_string())
}

pub async fn verify_transfer_statecoin(
    conn: &NodeConnector,
    pool: &PoolWrapper,
    transfer_message: &TransferMessage,
    signed_id: &str,
) -> Result<String> {
    let bk_txs = &transfer_message.backup_txs;

    let previous_tx = consensus::deserialize::<Transaction>(&hex::decode(bk_txs)?)?;
    let statechain_id = &transfer_message.statechain_id;

    let txn = transfer_message.txn;
    let n_lock_time = previous_tx.lock_time.to_consensus_u32();

    let txn_n_lock_time = txn.to_string() + &n_lock_time.to_string();
    let mut hasher = Sha256::new();
    hasher.update(txn_n_lock_time.as_bytes());
    let client_commitment = hasher.finish().to_lower_hex_string();

    
    let res = statechain::get_verification_statecoin(conn, statechain_id, signed_id).await?;

    //check the previous transaction is valid and not spent
    if client_commitment != res.txn_n_lock_time_commitment {
        return Err(anyhow!(
            "Incompatiple commitment, txn and n_lock_time are inconsistent"
        ));
    }

    //check the funding transaction is not spent

    let txid = previous_tx.txid().to_string();
    let is_broadcasted = get_transaction_existence(&txid).await?;
    if is_broadcasted {
        return Err(anyhow!("Previous backup transaction is broadcasted"));
    }

    //check the spend key is valid

    // send confirm for the server to update the new

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
