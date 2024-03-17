use std::str::FromStr;

use bitcoin::{
    absolute, consensus,
    secp256k1::{Message, Secp256k1, SecretKey},
    sighash::SighashCache,
    transaction::Version,
    Address, Amount, EcdsaSighashType, Network, OutPoint, ScriptBuf, Sequence, Transaction, TxIn,
    TxOut, Witness,
};
use tauri::State;

use shared::{
    api,
    model::{Txn, Utxo},
};

use wallet::core::{Account, AddrType, MasterAccount, MasterKeyEntropy, Mnemonic, Unlocker};

use crate::{
    cfg::{BASE_TX_FEE, PASSPHRASE},
    db::PoolWrapper,
    model::{AccountActions, AccountDTO},
    store::master_account::{get_master, get_mut_master, initialize_master_account},
    svc::account::{get_internal_account, get_utxos_set, parse_derivation_path},
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
    let account = get_internal_account(deriv)?;
    Ok(account.into())
}

#[tauri::command]
pub async fn get_utxo(address: &str) -> Result<Vec<Utxo>, String> {
    api::get_utxo(address).await
}

#[tauri::command]
pub async fn get_balance(address: &str) -> Result<u64, String> {
    let utxos = api::get_utxo(address).await?;
    Ok(utxos.iter().map(|utxo| utxo.value).sum())
}

#[tauri::command]
pub async fn create_tx(deriv: &str, receiver: &str, amount: u64) -> Result<u64, String> {
    let master: MasterAccount = get_master().expect("Master account does not exist");
    let parsed_path = parse_derivation_path(deriv).map_err(|e| e.to_string())?;
    let account = master.accounts().get(&parsed_path).unwrap();

    let selected_utxos = get_utxos_set(&account.get_addr(), amount).await?;

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

    let sighash_type = EcdsaSighashType::All;
    let mut sighasher = SighashCache::new(&mut unsigned_tx);

    let future_tasks: Vec<_> = tx
        .input
        .iter()
        .enumerate()
        .map(|(index, input)| tokio::spawn(tokio::spawn(dosth(index, input.clone()))))
        .collect();

    let mut results = Vec::new();
    for job in future_tasks {
        results.push(job.await.unwrap().unwrap().unwrap());
    }

    let mut unlocker = Unlocker::new_for_master(&master, PASSPHRASE).unwrap();
    let secp = Secp256k1::new();

    results.iter().for_each(|(index, input, tx)| {
        let vout = tx
            .vout
            .get(input.previous_output.vout as usize)
            .expect("Cannot get the vout");
        let amount = Amount::from_sat(vout.value);

        let script_pubkey =
            ScriptBuf::from_hex(&vout.scriptpubkey).expect("Invalid script public key");
        println!("Script code 1: {}", script_pubkey);

        let sighash = sighasher
            .p2wpkh_signature_hash(*index, &script_pubkey, amount, sighash_type)
            .expect("failed to create sighash");

        let priv_key = account
            .get_privkey(script_pubkey.clone(), &mut unlocker)
            .expect("Cannot get private key");
        // input.script_sig = ScriptBuf::new();
        let msg = Message::from(sighash);
        let sk = SecretKey::from_slice(&priv_key.to_bytes()).unwrap();

        let sig = secp.sign_ecdsa(&msg, &sk);

        // Update the witness stack.
        let signature = bitcoin::ecdsa::Signature {
            sig,
            hash_ty: EcdsaSighashType::All,
        };

        let pk = sk.public_key(&secp);
        *sighasher.witness_mut(*index).unwrap() = Witness::p2wpkh(&signature, &pk);
    });

    let tx_hex = consensus::encode::serialize_hex(&unsigned_tx);
    println!("hash: {:?}", tx_hex);
    println!("{:#?}", unsigned_tx);

    Ok(0)
}

async fn dosth(index: usize, input: TxIn) -> Result<(usize, TxIn, Txn), String> {
    match api::get_onchain_tx(&input.previous_output.txid.to_string()).await {
        Ok(tx) => Ok((index, input, tx)),
        Err(e) => Err(format!("Failed to get transaction for input {}", e)),
    }
}
