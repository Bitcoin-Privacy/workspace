use std::str::FromStr;

use bitcoin::{address::NetworkChecked, Address, Network};
use serde::{Deserialize, Serialize};
use wallet::core::{Account, AddrType};

pub trait AccountActions {
    fn get_derivation_path(&self) -> (u32, u32);
    fn get_addr(&self) -> String;
    fn get_checked_addr(&self) -> Address<NetworkChecked>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountDTO {
    address_type: AddrType,
    account_number: u32,
    sub_account_number: u32,
    network: Network,
    address: String,
}

impl AccountActions for AccountDTO {
    fn get_derivation_path(&self) -> (u32, u32) {
        return (self.account_number, self.sub_account_number);
    }

    fn get_addr(&self) -> String {
        self.address.clone()
    }

    fn get_checked_addr(&self) -> Address<NetworkChecked> {
        let addr = Address::from_str(&self.address).unwrap();
        addr.require_network(Network::Testnet).unwrap()
    }
}

impl AccountActions for Account {
    fn get_derivation_path(&self) -> (u32, u32) {
        return (self.account_number, self.sub_account_number);
    }

    fn get_addr(&self) -> String {
        return self.get_key(0).unwrap().address.to_string();
    }

    fn get_checked_addr(&self) -> Address<NetworkChecked> {
        return self.get_key(0).unwrap().address.clone();
    }
}

impl From<Account> for AccountDTO {
    fn from(value: Account) -> Self {
        AccountDTO {
            address_type: value.address_type,
            account_number: value.account_number,
            sub_account_number: value.sub_account_number,
            network: value.network,
            address: value.get_addr(),
        }
    }
}
