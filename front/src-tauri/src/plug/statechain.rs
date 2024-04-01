use shared::intf::statechain::DepositRes;
use tauri::{
    command,
    plugin::{Builder, TauriPlugin},
    Runtime, State,
};

use crate::{connector::NodeConnector, svc::statechain, util, TResult};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("statechain")
        .invoke_handler(tauri::generate_handler![
            // Modifier
            deposit,
            // Accessors
        ])
        .build()
}

// Modifiers --------------------------------------

#[command]
pub async fn deposit(conn: State<'_, NodeConnector>, amount: u64) -> TResult<DepositRes> {
    statechain::deposit(&conn, amount)
        .await
        .map_err(util::to_string)
}

// Accessors --------------------------------------
