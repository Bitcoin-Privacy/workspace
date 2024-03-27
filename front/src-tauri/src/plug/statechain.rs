use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

use crate::cmd::statechain;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("statechain")
        .invoke_handler(tauri::generate_handler![
            // Modifier
            statechain::deposit,
            // Accessors
        ])
        .build()
}
