use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

use crate::cmd::coinjoin;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("coinjoin")
        .invoke_handler(tauri::generate_handler![
            // Modifier
            coinjoin::register,
            coinjoin::sign_tx,
            // Accessors
            coinjoin::get_rooms,
            coinjoin::get_tx,
            coinjoin::get_status,
        ])
        .build()
}
