use std::str::FromStr;

use crate::db::PoolWrapper;
use anyhow::Result;
use bitcoin::{
    bip32::Xpub, secp256k1::{rand, schnorr::Signature, Keypair, PublicKey, Secp256k1, SecretKey}, Address
};

use shared::intf::statechain::{AggregatedPublicKey, DepositReq, DepositRes};

use crate::{connector::NodeConnector, store::master_account::get_master};

use super::account;
use statechain_core::utils::get_network;

pub async fn deposit(
    pool: &PoolWrapper,
    conn: &NodeConnector,
    deriv: &str,
    amount: u64,
) -> Result<AggregatedPublicKey> {
    let secp = Secp256k1::new();
    // let keypair = Keypair::new(&secp, &mut rand::thread_rng());
    // let xonly_pubkey = XOnlyPublicKey::from_keypair(&keypair).0;

    let auth_keypair = Keypair::new(&secp, &mut rand::thread_rng());
    let auth_seckey = SecretKey::from_keypair(&auth_keypair);
    let auth_pubkey = PublicKey::from_keypair(&auth_keypair);
   

    let acct = account::get_internal_account(deriv)?;
    let req = DepositReq {
        token_id: "abc".to_string(),
        addr: auth_pubkey.to_string(),
        amount: amount as u32,
    };
    println!("Deposit request {:#?}", req);
    let body = serde_json::to_value(req)?;
    let res = conn.post("statechain/deposit", &body).await?;

    let json: DepositRes = serde_json::from_value(res)?;
    println!("Deposit response {:#?}", json);
    // response
    let se_pubkey = json.se_pubkey_1;
    let statechain_id = json.statechain_id;

    //gen o1
    let owner_keypair = Keypair::new(&secp, &mut rand::thread_rng());
    let owner_seckey = SecretKey::from_keypair(&owner_keypair);
    let owner_pubkey = PublicKey::from_keypair(&owner_keypair);
    
    //gen auth_key

    // combine 2 address
    let key = create_aggregated_address(owner_pubkey.to_string(), se_pubkey, "testnet".to_string())
        .unwrap();

    if let Err(e) = pool
        .insert_statecoin(
            &statechain_id,
            &acct.master_public.to_string(),
            amount,
            &auth_seckey,
            &auth_pubkey,
            &key.aggregated_pubkey,
            &key.aggregated_address,
            &owner_seckey,
            &owner_pubkey,
        )
        .await
    {
        panic!("Failed to insert statecoin data {:?}", e);
    }

    Ok(key)
}

pub fn create_aggregated_address(
    k1: String,
    k2: String,
    network: String,
) -> Result<AggregatedPublicKey> {
    let secp = Secp256k1::new();
    let network = get_network(&network)?;
    let pub_k1 = PublicKey::from_str(&k1)?;
    let pub_k2 = PublicKey::from_str(&k2)?;

    let aggregated_pubkey = pub_k1.combine(&pub_k2)?;

    let aggregated_address = Address::p2tr(
        &secp,
        aggregated_pubkey.x_only_public_key().0,
        None,
        network,
    );

    Ok(AggregatedPublicKey {
        aggregated_pubkey: aggregated_pubkey.to_string(),
        aggregated_address: aggregated_address.to_string(),
    })
}
