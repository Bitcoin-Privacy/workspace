use anyhow::Result;
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
async fn init(pool: &PoolWrapper) -> Result<InitState> {
    let state = match pool.get_password().await? {
        Some(_) => match pool.get_seed().await? {
            Some(seed) => {
                let mnemonic = Mnemonic::from_str(&seed).unwrap();
                initialize_master_account(&mnemonic, 0, Network::Testnet, PASSPHRASE, None);
                InitState::CreatedWallet
            }
            None => InitState::CreatedPassword,
        },
        None => InitState::BrandNew,
    };
    Ok(state)
}

#[tauri::command]
pub async fn get_init_state(pool: State<'_, PoolWrapper>) -> Result<InitState, String> {
    init(&pool).await.map_err(|e| e.to_string())
}
