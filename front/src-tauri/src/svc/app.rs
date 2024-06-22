use std::{ops::ControlFlow, str::FromStr};

use anyhow::{anyhow, Result};
use bitcoin::{
    absolute, consensus, hashes::sha256, secp256k1::Secp256k1, sighash::SighashCache,
    transaction::Version, Address, Amount, EcdsaSighashType, Network, OutPoint, ScriptBuf,
    Sequence, Transaction, TxIn, TxOut, Witness,
};
use secp256k1::hashes::Hash;
use shared::api::broadcast_tx;
use shared::model::Utxo;
use wallet::core::{Account, MasterKeyEntropy, Mnemonic};

use crate::{
    cfg::{BASE_TX_FEE, PASSPHRASE},
    db::PoolWrapper,
    model::{AccountActions, AccountDTO, InitState},
    store::master_account::{get_master, initialize_master_account},
};

use super::account;

/// Initialize function, should be called when setup the application
/// - Load password
/// - Load master account
/// - Init subaccount
/// - Return the app state
pub async fn init(pool: &PoolWrapper) -> Result<InitState> {
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

pub async fn signup(pool: &PoolWrapper, password: &str) -> Result<()> {
    let hash = sha256::Hash::hash(password.as_bytes());
    pool.set_password(&hash.to_string()).await
}

pub async fn signin(pool: &PoolWrapper, password: &str) -> Result<bool> {
    let hash = sha256::Hash::hash(password.as_bytes());
    let pw = pool.get_password().await?;
    match pw {
        Some(pw) => Ok(hash.to_string() == pw),
        None => Err(anyhow!("Password not found")),
    }
}

pub async fn create_master(pool: &PoolWrapper) -> Result<Vec<String>> {
    let mnemonic = Mnemonic::new_random(MasterKeyEntropy::Sufficient)?;
    let seed = mnemonic.to_seed_phrase();

    pool.set_seed(&seed.join(" ")).await?;

    initialize_master_account(&mnemonic, 0, Network::Testnet, PASSPHRASE, None);

    Ok(seed)
}

pub async fn add_account() {
    // println!("Add account");
    // let mut master = get_mut_master();
    // let mut unlocker = Unlocker::new_for_master(master.as_ref().unwrap(), PASSPHRASE).unwrap();
    //
    // let account = Account::new(&mut unlocker, AddrType::P2PKH, 0, 0, 10).unwrap();
    // master.as_mut().unwrap().add_account(account);
    //
    // println!("Master Account: {:#?}", master);
}

pub async fn create_txn(deriv: &str, receiver: &str, amount: u64) -> Result<()> {
    let (account, mut unlocker) = account::get_account(deriv).unwrap();
    let selected_utxos = account.get_utxo(amount + BASE_TX_FEE).await?;

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
        return Err(anyhow!("Total amount cannot cover amount and fee"));
    }
    if change > 0 {
        output.push(TxOut {
            value: Amount::from_sat(change),
            script_pubkey: account.get_checked_addr().script_pubkey(),
        });
    }
    let addr = Address::from_str(receiver).unwrap();
    let checked_addr = addr.require_network(Network::Testnet).unwrap();

    output.push(TxOut {
        value: Amount::from_sat(amount),
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
        return Err(e);
    }

    let tx_hex = consensus::encode::serialize_hex(&unsigned_tx);
    println!("hash: {:?}", tx_hex);
    println!("{:#?}", unsigned_tx);
    let broadcast_tx_res = broadcast_tx(tx_hex).await;
    println!("Broadcast transaction response {:#?}", broadcast_tx_res);

    Ok(())
}

pub fn get_accounts() -> Result<Vec<AccountDTO>> {
    let master_account = get_master().expect("Master account does not exist");
    let accts = master_account
        .accounts()
        .values()
        .map(|e| <Account as Into<AccountDTO>>::into((*e).clone()))
        .collect();
    Ok(accts)
}

pub fn get_account(deriv: &str) -> Result<AccountDTO> {
    let account = account::get_internal_account(deriv)?;
    Ok(account.into())
}

pub async fn get_utxos(address: &str) -> Result<Vec<Utxo>> {
    shared::api::get_utxo(address).await
}

pub async fn get_status(txid: &str) -> Result<bool> {
    shared::api::get_status(txid).await
}

pub async fn get_balance(address: &str) -> Result<u64> {
    let utxos = get_utxos(address).await?;
    Ok(utxos.iter().map(|utxo| utxo.value).sum())
}
