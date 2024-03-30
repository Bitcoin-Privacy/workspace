// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use connector::NodeConnector;

mod api;
mod cfg;
mod connector;
mod db;
mod model;
mod plug;
mod store;
mod svc;
mod util;

use db::PoolWrapper;

type TResult<T> = Result<T, String>;

#[tokio::main]
async fn main() {
    let pool = PoolWrapper::new().await;
    let node_conn = NodeConnector::new(cfg::CFG.service_url.clone());
    tauri::Builder::default()
        .manage(node_conn)
        .manage(pool)
        .plugin(plug::app::init())
        .plugin(plug::coinjoin::init())
        .plugin(plug::statechain::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
