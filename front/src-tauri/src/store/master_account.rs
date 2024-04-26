use std::sync::{Mutex, MutexGuard};

use anyhow::Result;
use bitcoin::Network;
use lazy_static::lazy_static;
use statechain_core::wallet::Wallet;
use wallet::core::{Account, AddrType, MasterAccount, Mnemonic, Unlocker};

use crate::cfg::{CFG, PASSPHRASE};

lazy_static! {
    pub static ref MASTER: Mutex<Option<MasterAccount>> = Mutex::new(None);
    pub static ref WALLET: Mutex<Option<Wallet>> = Mutex::new(None);
}

pub fn initialize_master_account(
    mnemonic: &Mnemonic,
    birth: u64,
    network: Network,
    passphrase: &str,
    pd_passphrase: Option<&str>,
) {
    let mut singleton = MASTER.lock().unwrap();
    if singleton.is_none() {
        let mut master =
            MasterAccount::from_mnemonic(mnemonic, birth, network, passphrase, pd_passphrase)
                .unwrap();
        // Create default account
        let mut unlocker = Unlocker::new_for_master(&master, PASSPHRASE).unwrap();

        let account_0 = Account::new(&mut unlocker, AddrType::P2WPKH, 0, 0, 10).unwrap();
        let account_1 = Account::new(&mut unlocker, AddrType::P2WPKH, 1, 0, 10).unwrap();
        master.add_account(account_0);
        master.add_account(account_1);
        *singleton = Some(master);
    }
}

pub fn get_mut_master() -> MutexGuard<'static, Option<MasterAccount>> {
    MASTER.lock().unwrap()
}

pub fn get_master() -> Option<MasterAccount> {
    MASTER.lock().unwrap().clone()
}

pub fn get_mut_wallet() -> MutexGuard<'static, Option<Wallet>> {
    WALLET.lock().unwrap()
}

pub fn get_wallet() -> Option<Wallet> {
    WALLET.lock().unwrap().clone()
}

pub async fn create_wallet(mnemonic: &Mnemonic) -> Result<Wallet> {
    let blockheight = 0;

    let wallet = Wallet {
        name: String::from("Master wallet"),
        mnemonic: mnemonic.to_string(),
        version: String::from("0.1.0"),
        state_entity_endpoint: CFG.service_url.clone(),
        electrum_endpoint: String::from(""),
        network: String::from("testnet"),
        blockheight,
        initlock: 0,
        interval: 0,
        tokens: Vec::new(),
        activities: Vec::new(),
        coins: Vec::new(),
    };

    // save wallet to database

    Ok(wallet)
}
