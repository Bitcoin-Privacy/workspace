// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use connector::NodeConnector;

pub mod api;
pub mod cfg;
pub mod cmd;
pub mod connector;
pub mod db;
pub mod model;
pub mod store;
pub mod svc;

use db::PoolWrapper;

mod plug;

#[tokio::main]
async fn main() {
    // Initialize the SQLite database connection pool asynchronously.
    let pool = PoolWrapper::new().await;

    tauri::Builder::default()
        .manage(NodeConnector::new())
        .manage(pool)
        .plugin(plug::coinjoin::init())
        .plugin(plug::statechain::init())
        .invoke_handler(tauri::generate_handler![
            /*
             * App commands
             */
            cmd::app::get_init_state,
            /*
             * Auth commands
             */
            cmd::auth::save_password,
            cmd::auth::save_room_id,
            /*
             * Account commands
             */
            cmd::account::get_accounts,
            cmd::account::get_account,
            cmd::account::get_utxo,
            cmd::account::get_balance,
            cmd::account::print_master, // WARN: For debugging purpose only
            //---
            cmd::account::create_master,
            cmd::account::add_account, // NOTE: not used yet
            cmd::account::create_tx,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
