// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cfg::DATABASE_PATH;
use connector::NodeConnector;

pub mod api;
pub mod cfg;
pub mod cmd;
pub mod connector;
pub mod db;
pub mod error;
pub mod model;
pub mod store;
pub mod svc;

fn main() {
    let db: sled::Db = sled::open(DATABASE_PATH).unwrap();
    let cloned_db = db.clone();
    tauri::Builder::default()
        .setup(move |_| {
            cmd::app::init(&cloned_db.clone());
            Ok(())
        })
        .manage(db::PoolWrapper { pool: db })
        .manage(NodeConnector::new())
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
            /*
             * CoinJoin commands
             */
            cmd::coinjoin::get_rooms,
            cmd::coinjoin::get_tx,
            cmd::coinjoin::get_status,
            //---
            cmd::coinjoin::register,
            // cmd::coinjoin::sign_tx,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
