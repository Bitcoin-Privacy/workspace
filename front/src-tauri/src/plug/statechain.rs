use shared::intf::statechain::{AggregatedPublicKey, DepositRes};
use tauri::{
    command,
    plugin::{Builder, TauriPlugin},
    Runtime, State,
};

use crate::{connector::NodeConnector, db::PoolWrapper, svc::statechain, util, TResult};

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
pub async fn deposit(
    pool: State<'_, PoolWrapper>,
    conn: State<'_, NodeConnector>,
    deriv: &str,
    amount: u64,
) -> TResult<AggregatedPublicKey> {
    statechain::deposit(&pool, &conn, &deriv, amount)
        .await
        .map_err(util::to_string)
}

// Accessors --------------------------------------
