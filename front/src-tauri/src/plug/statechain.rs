use bitcoin::consensus;
use shared::intf::statechain::{DepositInfo, DepositRes};
use tauri::{
    command,
    plugin::{Builder, TauriPlugin},
    Runtime, State,
};

use crate::{
    connector::NodeConnector, db::PoolWrapper, model::StateCoinInfo, svc::statechain, util, TResult,
};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("statechain")
        .invoke_handler(tauri::generate_handler![
            // Modifier
            deposit,
            list_statecoins,
            send_statecoin //create_deposit_tx
                           // Accessors
        ])
        .build()
}

// Modifiers --------------------------------------

#[command]
pub async fn deposit(
    pool: State<'_, PoolWrapper>,
    conn: State<'_, NodeConnector>,
    deriv: &str,
    amount: u64,
) -> TResult<DepositInfo> {
    statechain::deposit(&pool, &conn, &deriv, amount)
        .await
        .map_err(util::to_string)
}

#[command]
pub async fn list_statecoins(
    pool: State<'_, PoolWrapper>,
    deriv: &str,
) -> TResult<Vec<StateCoinInfo>> {
    statechain::list_statecoins(&pool, &deriv)
        .await
        .map_err(util::to_string)
}

#[command]
pub async fn send_statecoin(
    pool: State<'_, PoolWrapper>,
    conn: State<'_, NodeConnector>,
    pubkey: &str,
    authkey: &str,
    statechain_id: &str,
) -> TResult<String> {
    statechain::send_statecoin(&conn, &pool, pubkey, authkey, statechain_id)
        .await
        .map_err(util::to_string)
}

// #[command]
// pub async fn create_bk_tx(
//     pool: State<'_, PoolWrapper>,
//     conn: State<'_, NodeConnector>,
//     agg_pubkey: &str,
//     agg_address: &str,
//     receiver_address: &str,
//     txid: &str,
//     vout: u32,
//     amount: u64,
//     statechain_id: &str,
// ) -> TResult<String> {
//     let res = statechain::create_bk_tx(
//         &pool,
//         &conn,
//         &agg_pubkey,
//         &agg_address,
//         &receiver_address,
//         &txid,
//         vout,
//         amount,
//         &statechain_id,
//     )
//     .await.unwrap();
//     Ok(consensus::encode::serialize_hex(&res))
// }

// #[command]
// pub async fn create_deposit_tx(
//     pool: State<'_, PoolWrapper>,
//     deriv: &str,
//     amount: u64,
//     aggregated_address: &str,
//     statechain_id : &str,
// ) -> TResult<String> {
//     statechain::create_deposit_transaction(&pool, &deriv, amount, &aggregated_address,&statechain_id)
//         .await
//         .map_err(util::to_string)
// }
//Accessors --------------------------------------
