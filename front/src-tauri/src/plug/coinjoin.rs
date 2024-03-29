use shared::intf::coinjoin::{GetStatusRes, GetUnsignedTxnRes};
use tauri::{
    command,
    plugin::{Builder, TauriPlugin},
    Runtime,
};

use tauri::State;

use crate::{db::PoolWrapper, model::RoomEntity, svc::coinjoin, util, TResult};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("coinjoin")
        .invoke_handler(tauri::generate_handler![
            // Modifier
            register, sign_tx, // Accessors
            get_rooms, get_txn, get_status,
        ])
        .build()
}

// Modifiers --------------------------------------

/// Register to CoinJoin Protocol
#[command]
async fn register(
    pool: State<'_, PoolWrapper>,
    // window: tauri::Window,
    deriv: &str,
    address: &str,
    amount: u64,
) -> TResult<()> {
    coinjoin::register(&pool, deriv, amount, address)
        .await
        .map_err(util::to_string)?;
    Ok(())
}

#[command]
async fn sign_tx(pool: State<'_, PoolWrapper>, deriv: &str, room_id: &str) -> TResult<()> {
    coinjoin::sign_tx(&pool, deriv, room_id)
        .await
        .map_err(util::to_string)
}

// Accessors --------------------------------------

#[command]
async fn get_txn(room_id: &str) -> TResult<GetUnsignedTxnRes> {
    crate::api::coinjoin::get_txn(room_id)
        .await
        .map_err(util::to_string)
}

#[command]
async fn get_status(room_id: &str) -> TResult<GetStatusRes> {
    coinjoin::get_status(room_id).await.map_err(util::to_string)
}

#[command]
async fn get_rooms(pool: State<'_, PoolWrapper>, deriv: &str) -> TResult<Vec<RoomEntity>> {
    coinjoin::get_rooms(&pool, deriv)
        .await
        .map_err(util::to_string)
}
