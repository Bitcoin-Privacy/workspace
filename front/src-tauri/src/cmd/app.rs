use bitcoin::Network;
use tauri::State;
use wallet::core::Mnemonic;

use crate::{
    cfg::PASSPHRASE, db::PoolWrapper, model::InitState,
    store::master_account::initialize_master_account,
};

/// Initialize function, should be called when setup the application
/// - Load password
/// - Load master account
/// - Init subaccount
/// - Return the app state
#[tauri::command]
pub fn init(db: &sled::Db) -> InitState {
    let password = db.get(b"password").expect("Cannot get password");

    match password {
        Some(password) => {
            let password = bincode::deserialize::<String>(&password).unwrap();
            let seedphrase = db.get(b"seedphrase").expect("Cannot get seedphrase");
            let birth = db.get(b"birth").expect("Cannot get birth");
            if let (Some(seedphrase), Some(birth)) = (seedphrase, birth) {
                let seedphrase = bincode::deserialize::<String>(&seedphrase).unwrap();
                let birth = bincode::deserialize::<u64>(&birth).unwrap();

                let mnemonic = Mnemonic::from_str(&seedphrase).unwrap();
                initialize_master_account(&mnemonic, birth, Network::Testnet, PASSPHRASE, None);

                InitState::CreatedWallet(password)
            } else {
                InitState::CreatedPassword(password)
            }
        }
        None => InitState::BrandNew,
    }
}

#[tauri::command]
pub fn get_init_state(state: State<'_, PoolWrapper>) -> InitState {
    init(&state.pool)
}
