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
        .plugin(plug::app::init())
        .plugin(plug::coinjoin::init())
        .plugin(plug::statechain::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
