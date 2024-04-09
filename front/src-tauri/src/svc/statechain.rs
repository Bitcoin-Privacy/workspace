use anyhow::{anyhow, Result};
use bitcoin::{
    absolute, consensus,
    secp256k1::{rand, Keypair, PublicKey, Secp256k1, SecretKey},
    sighash::SighashCache,
    transaction::Version,
    Address, Amount, EcdsaSighashType, Network, OutPoint, ScriptBuf, Sequence, Transaction, TxIn,
    TxOut, Witness,
};
use std::{ops::ControlFlow, str::FromStr};

use crate::{
    cfg::BASE_TX_FEE,
    db::PoolWrapper,
    model::{AccountActions, AccountDTO, InitState},
    store::master_account::{get_master, initialize_master_account},
    svc::app::create_txn,
};
use shared::intf::statechain::{AggregatedPublicKey, DepositReq, DepositRes};
use wallet::core::Account;

use crate::connector::NodeConnector;

use super::account;

pub async fn deposit(
    pool: &PoolWrapper,
    conn: &NodeConnector,
    deriv: &str,
    amount: u64,
) -> Result<AggregatedPublicKey> {
    let secp = Secp256k1::new();
    // let keypair = Keypair::new(&secp, &mut rand::thread_rng());
    // let xonly_pubkey = XOnlyPublicKey::from_keypair(&keypair).0;

    let auth_keypair = Keypair::new(&secp, &mut rand::thread_rng());
    let auth_seckey = SecretKey::from_keypair(&auth_keypair);
    let auth_pubkey = PublicKey::from_keypair(&auth_keypair);

    let (account, mut unlocker) = account::get_account(deriv).unwrap();
    let account_address = account.get_addr();
    let req = DepositReq {
        token_id: "abc".to_string(),
        addr: auth_pubkey.to_string(),
        amount: amount as u32,
    };
    println!("Deposit request {:#?}", req);
    let body = serde_json::to_value(req)?;
    let res = conn.post("statechain/deposit", &body).await?;

    let json: DepositRes = serde_json::from_value(res)?;
    println!("Deposit response {:#?}", json);
    // response
    let se_pubkey = json.se_pubkey_1;
    let statechain_id = json.statechain_id;

    //gen o1
    let owner_keypair = Keypair::new(&secp, &mut rand::thread_rng());
    let owner_seckey = SecretKey::from_keypair(&owner_keypair);
    let owner_pubkey = PublicKey::from_keypair(&owner_keypair);

    //gen auth_key

    // combine 2 address
    let key =
        create_aggregated_address(owner_pubkey.to_string(), se_pubkey, Network::Testnet).unwrap();
    if let Err(e) = pool
        .insert_statecoin(
            &statechain_id,
            &account_address,
            amount,
            &auth_seckey,
            &auth_pubkey,
            &key.aggregated_pubkey,
            &key.aggregated_address,
            &owner_seckey,
            &owner_pubkey,
        )
        .await
    {
        panic!("Failed to insert statecoin data {:?}", e);
    }

    //let tx = create_deposit_transaction(&pool, &deriv, amount,  &key2.aggregated_address).await?;
    //let tx = create_txn(deriv, &key2.aggregated_address, amount).await?;

    Ok(key)
}

pub fn create_aggregated_address(
    k1: String,
    k2: String,
    network: Network,
) -> Result<AggregatedPublicKey> {
    let secp = Secp256k1::new();
    let pub_k1 = PublicKey::from_str(&k1)?;
    let pub_k2 = PublicKey::from_str(&k2)?;

    let aggregated_pubkey = pub_k1.combine(&pub_k2)?;

    let aggregated_address = Address::p2tr(
        &secp,
        aggregated_pubkey.x_only_public_key().0,
        None,
        network,
    );

    Ok(AggregatedPublicKey {
        aggregated_pubkey: aggregated_pubkey.to_string(),
        aggregated_address: aggregated_address.to_string(),
    })
}

pub async fn create_deposit_transaction(
    pool: &PoolWrapper,
    deriv: &str,
    amount: u64,
    aggregated_address: &str,
) -> Result<String> {
    let (account, mut unlocker) = account::get_account(deriv).unwrap();
    let selected_utxos = account::get_utxos_set(&account.get_addr(), amount).await?;

    let mut fee: u64 = 0;
    let input: Vec<TxIn> = selected_utxos
        .iter()
        .map(|utxo| {
            fee += utxo.value;
            println!("utxos set :{}", utxo.value);
            TxIn {
                previous_output: OutPoint::new(utxo.txid.parse().unwrap(), utxo.vout.into()),
                script_sig: ScriptBuf::from_bytes(vec![]),
                sequence: Sequence::MAX,
                witness: Witness::new(),
            }
        })
        .collect();

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

    let addr = Address::from_str(aggregated_address).unwrap();
    let checked_addr = addr.require_network(Network::Testnet).unwrap();

    output.push(TxOut {
        value: Amount::from_sat(amount),
        script_pubkey: checked_addr.script_pubkey(),
    });

    let deposit_tx = Transaction {
        version: Version::TWO,
        lock_time: absolute::LockTime::ZERO,
        input: input,
        output: output,
    };

    let mut unsigned_deposit_tx = deposit_tx.clone();

    let secp = Secp256k1::new();
    let sighash_type = EcdsaSighashType::All;
    let mut sighasher = SighashCache::new(&mut unsigned_deposit_tx);

    let future_tasks: Vec<_> = deposit_tx
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

    let tx_hex = consensus::encode::serialize_hex(&unsigned_deposit_tx);
    println!("hash: {:?}", tx_hex);
    println!("{:#?}", unsigned_deposit_tx);
    Ok(tx_hex)
}

pub fn create_tx(receiver_address: &Address, prev_outpoint: OutPoint, amount: u64) -> Transaction {
    let input = TxIn {
        previous_output: prev_outpoint,
        script_sig: ScriptBuf::default(),
        sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
        witness: Witness::default(),
    };

    let spend = TxOut {
        value: Amount::from_sat(amount),
        script_pubkey: receiver_address.script_pubkey(),
    };

    let unsigned_tx = Transaction {
        version: Version::TWO,               // Post BIP-68.
        lock_time: absolute::LockTime::ZERO, // Ignore the locktime.
        input: vec![input],                  // Input goes into index 0.
        output: vec![spend],                 // Outputs, order does not matter.
    };

    unsigned_tx
}
