use std::{ops::ControlFlow, str::FromStr};

use bitcoin::{
    absolute, consensus, secp256k1::Secp256k1, sighash::SighashCache, transaction::Version,
    Address, Amount, EcdsaSighashType, Network, OutPoint, ScriptBuf, Sequence, Transaction, TxIn,
    TxOut, Witness,
};
use tauri::State;

use shared::{api, model::Utxo};
use wallet::core::{Account, AddrType, MasterAccount, MasterKeyEntropy, Mnemonic, Unlocker};

use crate::{
    cfg::{BASE_TX_FEE, PASSPHRASE},
    db::PoolWrapper,
    model::{AccountActions, AccountDTO},
    store::master_account::{get_master, get_mut_master, initialize_master_account},
    svc::account,
};

#[tauri::command]
pub fn print_master() {
    let master = get_master();
    println!("GET Master Account: {:#?}", master);
}

#[tauri::command]
pub fn add_account() {
    println!("Add account");
    let mut master = get_mut_master();
    let mut unlocker = Unlocker::new_for_master(master.as_ref().unwrap(), PASSPHRASE).unwrap();

    let account = Account::new(&mut unlocker, AddrType::P2PKH, 0, 0, 10).unwrap();
    master.as_mut().unwrap().add_account(account);

    println!("Master Account: {:#?}", master);
}

// NOTE: - New Version HERE ---------------------------------------------
#[tauri::command]
pub fn create_master(state: State<'_, PoolWrapper>) -> Result<Vec<String>, String> {
    let mnemonic = Mnemonic::new_random(MasterKeyEntropy::Sufficient).map_err(|e| e.to_string())?;
    let seed = mnemonic.to_seed_phrase();
    let birth = 0;

    let _ = state
        .pool
        .insert(
            b"seedphrase",
            bincode::serialize(&seed.clone().join(" ")).unwrap(),
        )
        .expect("Cannot insert seedphrase");
    let _ = state
        .pool
        .insert(b"birth", bincode::serialize(&birth).unwrap())
        .expect("Cannot insert birth");

    initialize_master_account(&mnemonic, birth, Network::Testnet, PASSPHRASE, None);

    Ok(seed)
}

#[tauri::command]
pub fn get_accounts() -> Vec<AccountDTO> {
    let master_account: MasterAccount = get_master().expect("Master account does not exist");
    master_account
        .accounts()
        .values()
        .map(|e| <Account as Into<AccountDTO>>::into((*e).clone()))
        .collect()
}

#[tauri::command]
pub fn get_account(deriv: &str) -> Result<AccountDTO, String> {
    let account = account::get_internal_account(deriv).map_err(|e| e.to_string())?;
    Ok(account.into())
}

#[tauri::command]
pub async fn get_utxo(address: &str) -> Result<Vec<Utxo>, String> {
    api::get_utxo(address).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_balance(address: &str) -> Result<u64, String> {
    let utxos = api::get_utxo(address).await.map_err(|e| e.to_string())?;
    Ok(utxos.iter().map(|utxo| utxo.value).sum())
}

#[tauri::command]
pub async fn create_tx(deriv: &str, receiver: &str, amount: u64) -> Result<u64, String> {
    let (account, mut unlocker) = account::get_account(deriv).unwrap();

    let selected_utxos = account::get_utxos_set(&account.get_addr(), amount)
        .await
        .map_err(|e| e.to_string())?;

    let mut fee: u64 = 0;
    let input: Vec<TxIn> = selected_utxos
        .iter()
        .map(|utxo| {
            fee += utxo.value;
            TxIn {
                previous_output: OutPoint::new(utxo.txid.parse().unwrap(), utxo.vout.into()),
                script_sig: ScriptBuf::from_bytes(vec![]),
                sequence: Sequence::MAX,
                witness: Witness::new(),
            }
        })
        .collect();

    // Output for the receiver
    let mut output: Vec<TxOut> = Vec::new();
    let (change, overflow) = fee.overflowing_sub(amount + BASE_TX_FEE);
    if overflow {
        return Err("Total amount cannot cover amount and fee".to_string());
    }
    if change > 0 {
        output.push(TxOut {
            value: Amount::from_sat(change as u64),
            script_pubkey: account.get_checked_addr().script_pubkey(),
        });
    }
    let addr = Address::from_str(receiver).unwrap();
    let checked_addr = addr.require_network(Network::Testnet).unwrap();

    output.push(TxOut {
        value: Amount::from_sat(amount as u64),
        script_pubkey: checked_addr.script_pubkey(),
    });

    let tx = Transaction {
        version: Version::TWO,
        lock_time: absolute::LockTime::ZERO,
        input,
        output,
    };

    let mut unsigned_tx = tx.clone();

    let secp = Secp256k1::new();
    let sighash_type = EcdsaSighashType::All;
    let mut sighasher = SighashCache::new(&mut unsigned_tx);

    let future_tasks: Vec<_> = tx
        .input
        .iter()
        .enumerate()
        .map(|(index, input)| {
            tokio::spawn(tokio::spawn(account::find_and_join_txn(
                index,
                input.clone(),
            )))
        })
        .collect();

    let mut results = Vec::new();
    for job in future_tasks {
        results.push(job.await.unwrap().unwrap().unwrap());
    }

    let res = results.iter().try_for_each(|(index, input, tx)| {
        match account::sign(
            &secp,
            &mut sighasher,
            sighash_type,
            &account,
            &mut unlocker,
            index,
            input,
            tx,
        ) {
            Ok(_) => ControlFlow::Continue(()),
            Err(e) => ControlFlow::Break(e),
        }
    });
    if let ControlFlow::Break(e) = res {
        return Err(e.to_string());
    }

    let tx_hex = consensus::encode::serialize_hex(&unsigned_tx);
    println!("hash: {:?}", tx_hex);
    println!("{:#?}", unsigned_tx);

    Ok(0)
}
