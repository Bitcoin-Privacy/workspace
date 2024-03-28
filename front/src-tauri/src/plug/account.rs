use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

use crate::cmd::account;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("account")
        .invoke_handler(tauri::generate_handler![
            // Modifier
            account::create_master,
            account::add_account, // NOTE: not used yet
            account::create_tx,
            // Accessors
            account::get_accounts,
            account::get_account,
            account::get_utxo,
            account::get_balance,
        ])
        .build()
}
