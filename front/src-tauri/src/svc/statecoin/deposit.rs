use anyhow::{anyhow, Ok, Result};
use statechain_core::{
    deposit::{create_aggregated_address, create_deposit_msg1},
    transaction::get_user_backup_address,
    utils::get_blockheight,
    wallet::{BackupTx, Coin, CoinStatus, Wallet},
};

use crate::{connector::NodeConnector, db::PoolWrapper};

use super::{
    client_config::ClientConfig, sqlite_manager::update_wallet, transaction::new_transaction,
};

/// Get deposit address
/// - Create statecoin in local
/// - Request to server to get server's publickey
/// - Generate aggregated address
pub async fn register(
    pool: &PoolWrapper,
    _conn: &NodeConnector,
    client_config: &ClientConfig,
    wallet_name: &str,
    token_id: &str,
    amount: u32,
) -> Result<String> {
    let token_id = uuid::Uuid::parse_str(token_id)?;
    let wallet = pool.get_wallet(wallet_name).await?;
    let mut wallet = init(client_config, &wallet, token_id).await?;

    let coin = wallet.coins.last_mut().unwrap();

    let aggregated_public_key = create_aggregated_address(coin, wallet.network.clone())?;

    coin.amount = Some(amount);
    coin.aggregated_address = Some(aggregated_public_key.aggregate_address.clone());
    coin.aggregated_pubkey = Some(aggregated_public_key.aggregate_pubkey);

    pool.update_wallet(&wallet).await?;

    Ok(aggregated_public_key.aggregate_address)
}

/// Deposit init - get server pubkey + statechain\_id --> coin ->> wallet (db)
pub async fn init(
    client_config: &ClientConfig,
    wallet: &Wallet,
    token_id: uuid::Uuid,
) -> Result<Wallet> {
    let mut wallet = wallet.clone();

    let coin = wallet.get_new_coin()?;

    wallet.coins.push(coin.clone());

    update_wallet(&client_config.pool, &wallet).await?;

    // Init
    let deposit_msg_1 = create_deposit_msg1(&coin, &token_id.to_string())?;
    let endpoint = client_config.statechain_entity.clone();
    let path = "statechain/deposit";
    let client = client_config.get_reqwest_client()?;
    let request = client.post(format!("{}/{}", endpoint, path));
    let response = request.json(&deposit_msg_1).send().await?;
    if response.status() != 200 {
        let response_body = response.text().await?;
        return Err(anyhow!(response_body));
    }
    let value = response.text().await?;

    // server pubkey, statechain_id
    let deposit_msg_1_response: statechain_core::deposit::DepositMsg1Response =
        serde_json::from_str(value.as_str())?;

    let deposit_init_result =
        statechain_core::deposit::handle_deposit_msg_1_response(&coin, &deposit_msg_1_response)?;

    let coin = wallet.coins.last_mut().unwrap();

    coin.statechain_id = Some(deposit_init_result.statechain_id);
    coin.signed_statechain_id = Some(deposit_init_result.signed_statechain_id);
    coin.server_pubkey = Some(deposit_init_result.server_pubkey);

    update_wallet(&client_config.pool, &wallet).await?;

    Ok(wallet)
}

pub async fn get_token(client_config: &ClientConfig) -> Result<String> {
    let endpoint = client_config.statechain_entity.clone();
    let path = "deposit/get_token";

    let client = client_config.get_reqwest_client()?;
    let request = client.get(&format!("{}/{}", endpoint, path));

    let response = request.send().await?;

    if response.status() != 200 {
        let response_body = response.text().await?;
        return Err(anyhow!(response_body));
    }

    let value = response.text().await?;

    let token: statechain_core::deposit::TokenID = serde_json::from_str(value.as_str())?;

    return Ok(token.token_id);
}

/// Create backup txn
pub async fn create_tx1(
    client_config: &ClientConfig,
    coin: &mut Coin,
    wallet_netwotk: &str,
    tx0_hash: &str,
    tx0_vout: u32,
) -> Result<BackupTx> {
    if coin.status != CoinStatus::INITIALISED {
        return Err(anyhow!(
            "The coin with the public key {} is not in the INITIALISED state",
            coin.user_pubkey.to_string()
        ));
    }

    if coin.utxo_txid.is_some() && coin.utxo_vout.is_some() {
        return Err(anyhow!(
            "The coin with the public key {} has already been deposited",
            coin.user_pubkey.to_string()
        ));
    }
    coin.utxo_txid = Some(tx0_hash.to_string());
    coin.utxo_vout = Some(tx0_vout);

    coin.status = CoinStatus::IN_MEMPOOL;

    let to_address = get_user_backup_address(coin, wallet_netwotk.to_string())?;

    // NOTE: New Backup transaction here
    let signed_tx = new_transaction(
        client_config,
        coin,
        &to_address,
        0,
        false,
        None,
        wallet_netwotk,
    )
    .await?;

    if coin.public_nonce.is_none() {
        return Err(anyhow::anyhow!("coin.public_nonce is None"));
    }

    if coin.blinding_factor.is_none() {
        return Err(anyhow::anyhow!("coin.blinding_factor is None"));
    }

    if coin.statechain_id.is_none() {
        return Err(anyhow::anyhow!("coin.statechain_id is None"));
    }

    let backup_tx = BackupTx {
        tx_n: 1,
        tx: signed_tx,
        client_public_nonce: coin.public_nonce.as_ref().unwrap().to_string(),
        server_public_nonce: coin.server_public_nonce.as_ref().unwrap().to_string(),
        client_public_key: coin.user_pubkey.clone(),
        server_public_key: coin.server_pubkey.as_ref().unwrap().to_string(),
        blinding_factor: coin.blinding_factor.as_ref().unwrap().to_string(),
    };

    let block_height = Some(get_blockheight(&backup_tx)?);
    coin.locktime = block_height;

    Ok(backup_tx)
}
