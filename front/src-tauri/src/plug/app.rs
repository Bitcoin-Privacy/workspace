use shared::model::Utxo;
use shared::util;

use tauri::{
    command,
    plugin::{Builder, TauriPlugin},
    Runtime, State,
};

use crate::{
    db::PoolWrapper,
    model::{AccountDTO, InitState},
    svc::app,
    TResult,
};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("app")
        .invoke_handler(tauri::generate_handler![
            // Modifiers
            signup,
            signin,
            create_master,
            add_account, // NOTE: not used yet
            create_txn,
            // Accessors
            get_init_state,
            get_accounts,
            get_account,
            get_utxos,
            get_balance,
        ])
        .build()
}

// Modifiers --------------------------------------

#[command]
async fn signup(pool: State<'_, PoolWrapper>, password: &str) -> TResult<()> {
    app::signup(&pool, password)
        .await
        .map_err(util::to_string)?;
    Ok(())
}

#[command]
async fn signin(pool: State<'_, PoolWrapper>, password: &str) -> TResult<bool> {
    app::signin(&pool, password).await.map_err(util::to_string)
}

#[command]
async fn create_master(pool: State<'_, PoolWrapper>) -> TResult<Vec<String>> {
    app::create_master(&pool).await.map_err(util::to_string)
}

#[command]
async fn add_account() -> TResult<()> {
    app::add_account().await;
    Ok(())
}

#[command]
async fn create_txn(deriv: &str, receiver: &str, amount: u64) -> TResult<()> {
    app::create_txn(deriv, receiver, amount)
        .await
        .map_err(util::to_string)
}

// Accessors --------------------------------------
#[command]
async fn get_init_state(pool: State<'_, PoolWrapper>) -> TResult<InitState> {
    app::init(&pool).await.map_err(util::to_string)
}

#[command]
fn get_accounts() -> TResult<Vec<AccountDTO>> {
    app::get_accounts().map_err(util::to_string)
}

#[command]
fn get_account(deriv: &str) -> TResult<AccountDTO> {
    app::get_account(deriv).map_err(util::to_string)
}

#[command]
async fn get_utxos(address: &str) -> TResult<Vec<Utxo>> {
    app::get_utxos(address).await.map_err(util::to_string)
}

#[command]
async fn get_balance(address: &str) -> TResult<u64> {
    app::get_balance(address).await.map_err(util::to_string)
}
