// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use connector::NodeConnector;

mod api;
mod cfg;
mod cmd;
mod connector;
mod db;
mod model;
mod plug;
mod store;
mod svc;

use db::PoolWrapper;

#[tokio::main]
async fn main() {
    // Initialize the SQLite database connection pool asynchronously.
    let pool = PoolWrapper::new().await;

    tauri::Builder::default()
        .manage(NodeConnector::new())
        .manage(pool)
        .plugin(plug::account::init())
        .plugin(plug::coinjoin::init())
        .plugin(plug::statechain::init())
        .invoke_handler(tauri::generate_handler![
            /* App commands */
            cmd::app::get_init_state,
            /* Auth commands */
            cmd::auth::save_password,
            cmd::auth::save_room_id,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
