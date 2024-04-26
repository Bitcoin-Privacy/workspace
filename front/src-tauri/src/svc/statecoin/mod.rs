use std::str::FromStr;

use crate::{
    connector::NodeConnector,
    db::PoolWrapper,
    svc::{statechain::create_deposit_txn, statecoin::client_config::ClientConfig},
};

use anyhow::Result;
use bitcoin::{consensus, secp256k1::PublicKey};
use shared::api::broadcast_txn;
use statechain_core::deposit::create_aggregated_address;

pub mod broadcast_backup_tx;
pub mod client_config;
pub mod coin_status;
mod deposit;
pub mod sqlite_manager;
pub mod transaction;
pub mod transfer_receiver;
pub mod transfer_sender;
pub mod utils;
pub mod wallet;
pub mod withdraw;

static TOKEN: &str = "abc";

pub async fn deposit(
    pool: &PoolWrapper,
    _conn: &NodeConnector,
    deriv: &str,
    amount: u32,
) -> Result<String> {
    let client_config = ClientConfig::load().await;

    let token_id = uuid::Uuid::parse_str(TOKEN)?;
    let wallet = pool.get_wallet("Master").await?;
    let mut wallet = deposit::init(&client_config, &wallet, token_id).await?;

    let coin = wallet.coins.last_mut().unwrap();

    let aggregated_public_key = create_aggregated_address(coin, wallet.network.clone())?;
    let aggr_pk = PublicKey::from_str(&aggregated_public_key.aggregate_pubkey)?;

    coin.amount = Some(amount);
    coin.aggregated_address = Some(aggregated_public_key.aggregate_address.clone());
    coin.aggregated_pubkey = Some(aggregated_public_key.aggregate_pubkey);

    // Create deposit transaction
    let deposit_tx = create_deposit_txn(pool, deriv, amount as u64, &aggr_pk).await?;
    let deposit_tx_hex = consensus::encode::serialize_hex(&deposit_tx);
    let broadcast_res = broadcast_txn(&deposit_tx_hex).await;
    println!("BROADCASTED {:#?}", broadcast_res);

    pool.update_wallet(&wallet).await?;

    Ok(aggregated_public_key.aggregate_address)
}
