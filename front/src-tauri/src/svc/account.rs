use shared::api;
use shared::model::Utxo;
use wallet::core::{Account, MasterAccount};

use crate::store::master_account::get_master;

pub fn get_internal_account(derivation_path: &str) -> Result<Account, String> {
    let master_account: MasterAccount = get_master().expect("Master account does not exist");
    let parsed_path = parse_derivation_path(&derivation_path).map_err(|e| e.to_string())?;
    let account = master_account.accounts().get(&parsed_path);
    match account {
        Some(account) => Ok(account.clone()),
        None => Err("Account not found".to_string()),
    }
}

pub fn parse_derivation_path(deriv: &str) -> Result<(u32, u32), &'static str> {
    let parts: Vec<&str> = deriv.split('/').collect();
    if parts.len() == 2 {
        let part0 = parts[0]
            .parse::<u32>()
            .map_err(|_| "First part of the path is not a valid u32")?;
        let part1 = parts[1]
            .parse::<u32>()
            .map_err(|_| "Second part of the path is not a valid u32")?;
        Ok((part0, part1))
    } else {
        Err("Derivation path must be exactly two components separated by '/'")
    }
}

pub async fn get_utxos_set(addr: &str, amount: u64) -> Result<Vec<Utxo>, String> {
    let mut utxos = api::get_utxo(addr).await?;
    // Sort UTXOs in descending order by value
    utxos.sort_by(|a, b| b.value.cmp(&a.value));

    let mut selected_utxos: Vec<Utxo> = Vec::new();
    let mut total: u64 = 0;

    for utxo in utxos {
        if total >= amount {
            break;
        }
        selected_utxos.push(utxo.clone());
        total += utxo.value;
    }

    if total >= amount {
        Ok(selected_utxos)
    } else {
        Err("Do not have compatible UTXOs".to_string()) // Not enough funds
    }
}
