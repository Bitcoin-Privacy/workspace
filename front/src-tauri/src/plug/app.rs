use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

use crate::cmd::app;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("app")
        .invoke_handler(tauri::generate_handler![
            // Modifier
            app::save_password,
            app::save_room_id, // TODO: Remove
            app::signin,
            app::create_master,
            app::add_account, // NOTE: not used yet
            app::create_tx,
            // Accessors
            app::get_accounts,
            app::get_account,
            app::get_utxo,
            app::get_balance,
        ])
        .build()
}
