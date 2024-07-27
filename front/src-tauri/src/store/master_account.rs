use std::sync::{Mutex, MutexGuard};

use bitcoin::Network;
use lazy_static::lazy_static;
use wallet::core::{Account, AddrType, MasterAccount, Mnemonic, Unlocker};

use crate::cfg::PASSPHRASE;

lazy_static! {
    pub static ref MASTER: Mutex<Option<MasterAccount>> = Mutex::new(None);
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
        let account_2 = Account::new(&mut unlocker, AddrType::P2WPKH, 2, 0, 10).unwrap();
        let account_3 = Account::new(&mut unlocker, AddrType::P2WPKH, 3, 0, 10).unwrap();
        master.add_account(account_0);
        master.add_account(account_1);
        master.add_account(account_2);
        master.add_account(account_3);
        *singleton = Some(master);
    }
}

pub fn get_mut_master() -> MutexGuard<'static, Option<MasterAccount>> {
    MASTER.lock().unwrap()
}

pub fn get_master() -> Option<MasterAccount> {
    MASTER.lock().unwrap().clone()
}
