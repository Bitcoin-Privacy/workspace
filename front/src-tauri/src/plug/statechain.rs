use shared::intf::statechain::{DepositInfo, StatechainAddress};
use tauri::{
    command,
    plugin::{Builder, TauriPlugin},
    Runtime, State,
};

use crate::{
    cfg::BASE_TX_FEE,
    connector::NodeConnector,
    db::PoolWrapper,
    model::{StatecoinCard, StatecoinDetail, TransferStateCoinInfo},
    svc::{statechain, statechain_deposit, statechain_receiver, statechain_sender},
    util, TResult,
};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("statechain")
        .invoke_handler(tauri::generate_handler![
            // Modifier
            deposit,
            list_statecoins,
            send_statecoin,
            list_transfer_statecoins,
            verify_transfer_statecoin,
            generate_statechain_address,
            get_statecoin_detail_by_id,
            withdraw_statecoin //create_deposit_tx
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
    if amount < BASE_TX_FEE {
        return Err(util::to_string(
            "Amount is less than the base transaction fee",
        ));
    }
    statechain_deposit::execute(&pool, &conn, deriv, amount)
        .await
        .map_err(util::to_string)
}

#[command]
pub async fn list_statecoins(
    pool: State<'_, PoolWrapper>,
    deriv: &str,
) -> TResult<Vec<StatecoinCard>> {
    statechain::list_statecoins(&pool, deriv)
        .await
        .map_err(util::to_string)
}

#[command]
pub async fn send_statecoin(
    pool: State<'_, PoolWrapper>,
    conn: State<'_, NodeConnector>,
    address: &str,
    statechain_id: &str,
) -> TResult<String> {
    let address = hex::decode(address).map_err(util::to_string)?;
    let json_address = match std::str::from_utf8(&address) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let parsed_address: StatechainAddress = serde_json::from_str(json_address).unwrap();
    println!("transfer_message {:#?}", parsed_address);

    statechain_sender::execute(
        &conn,
        &pool,
        &parsed_address.owner_pubkey,
        &parsed_address.authkey,
        statechain_id,
    )
    .await
    .map_err(util::to_string)
}

#[command]
pub async fn list_transfer_statecoins(
    pool: State<'_, PoolWrapper>,
    conn: State<'_, NodeConnector>,
    deriv: &str,
) -> TResult<Vec<TransferStateCoinInfo>> {
    statechain::list_transfer_statecoins(&conn, &pool, deriv)
        .await
        .map_err(util::to_string)
}

#[command]
pub async fn verify_transfer_statecoin(
    pool: State<'_, PoolWrapper>,
    conn: State<'_, NodeConnector>,
    deriv: &str,
    transfer_message: &str,
    authkey: &str,
) -> TResult<String> {
    statechain_receiver::execute(&conn, &pool, deriv, transfer_message, authkey)
        .await
        .map_err(util::to_string)
}

#[command]
pub async fn generate_statechain_address(
    pool: State<'_, PoolWrapper>,
    deriv: &str,
) -> TResult<String> {
    statechain_receiver::generate_statechain_address(&pool, deriv)
        .await
        .map_err(util::to_string)
}

#[command]
pub async fn get_statecoin_detail_by_id(
    pool: State<'_, PoolWrapper>,
    statechain_id: &str,
) -> TResult<StatecoinDetail> {
    statechain::get_statecoin_detail_by_id(&pool, statechain_id)
        .await
        .map_err(util::to_string)
}

#[command]
pub async fn withdraw_statecoin(
    pool: State<'_, PoolWrapper>,
    conn: State<'_, NodeConnector>,
    statechain_id: &str,
    deriv: &str,
) -> TResult<()> {
    let _ = statechain::withdraw_statecoin(&conn, &pool, statechain_id, deriv)
        .await
        .map_err(util::to_string);
    Ok(())
}
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
