use shared::intf::coinjoin::{CheckSignatureRes, GetStatusRes, RoomDto};
use tauri::{
    command,
    plugin::{Builder, TauriPlugin},
    Wry,
};

use tauri::State;

use crate::{connector::NodeConnector, db::PoolWrapper, svc::coinjoin, util, TResult};

pub fn init() -> TauriPlugin<Wry> {
    Builder::new("coinjoin")
        .invoke_handler(tauri::generate_handler![
            // Modifier
            register, sign_txn, // Accessors
            get_rooms, get_status, get_signed
        ])
        .build()
}

// Modifiers --------------------------------------

/// Register to CoinJoin Protocol
#[command]
async fn register(
    pool: State<'_, PoolWrapper>,
    conn: State<'_, NodeConnector>,
    window: tauri::Window,
    deriv: &str,
    address: &str,
    amount: u64,
) -> TResult<()> {
    coinjoin::register(&pool, &conn, window, deriv, amount, address)
        .await
        .map_err(util::to_string)?;
    Ok(())
}

#[command]
async fn sign_txn(deriv: &str, room_id: &str) -> TResult<()> {
    coinjoin::sign_txn(deriv, room_id)
        .await
        .map_err(util::to_string)
}

// Accessors --------------------------------------

#[command]
async fn get_rooms(deriv: &str) -> TResult<Vec<RoomDto>> {
    coinjoin::get_rooms(deriv).await.map_err(util::to_string)
}

#[command]
async fn get_status(room_id: &str) -> TResult<GetStatusRes> {
    coinjoin::get_status(room_id).await.map_err(util::to_string)
}

#[command]
async fn get_signed(deriv: &str, room_id: &str) -> TResult<CheckSignatureRes> {
    coinjoin::get_signed(deriv, room_id)
        .await
        .map_err(util::to_string)
}
