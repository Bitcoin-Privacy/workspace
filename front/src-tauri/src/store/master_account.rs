use std::sync::{Mutex, MutexGuard};

use bitcoin::Network;
use lazy_static::lazy_static;

use crate::cfg::PASSPHRASE;
use crate::model::AccountAddressType;
use crate::svc::account::{Account, MasterAccount};
use crate::svc::mnemonic::Mnemonic;
use crate::svc::unlocker::Unlocker;

lazy_static! {
    pub static ref MASTER_ACCOUNT: Mutex<Option<MasterAccount>> = Mutex::new(None);
}

pub fn initialize_master_account(
    mnemonic: &Mnemonic,
    birth: u64,
    network: Network,
    passphrase: &str,
    pd_passphrase: Option<&str>,
) {
    let mut singleton = MASTER_ACCOUNT.lock().unwrap();
    if singleton.is_none() {
        let mut master =
            MasterAccount::from_mnemonic(mnemonic, birth, network, passphrase, pd_passphrase)
                .unwrap();
        // Create default account
        let mut unlocker = Unlocker::new_for_master(&master, PASSPHRASE).unwrap();

        let account_0 = Account::new(&mut unlocker, AccountAddressType::P2WPKH, 0, 0, 10).unwrap();
        let account_1 = Account::new(&mut unlocker, AccountAddressType::P2WPKH, 1, 0, 10).unwrap();
        master.add_account(account_0);
        master.add_account(account_1);
        *singleton = Some(master);
    }
}

pub fn get_mut_master() -> MutexGuard<'static, Option<MasterAccount>> {
    MASTER_ACCOUNT.lock().unwrap()
}

pub fn get_master() -> Option<MasterAccount> {
    MASTER_ACCOUNT.lock().unwrap().clone()
}
