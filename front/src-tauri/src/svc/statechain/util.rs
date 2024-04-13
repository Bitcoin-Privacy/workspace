use anyhow::{anyhow, Result};
use bitcoin::{
    absolute, consensus, secp256k1::Secp256k1, sighash::SighashCache, transaction::Version,
    Address, Amount, EcdsaSighashType, Network, OutPoint, ScriptBuf, Sequence, Transaction, TxIn,
    TxOut, Witness,
};

use std::{ops::ControlFlow, str::FromStr};

use super::account;
use crate::{cfg::BASE_TX_FEE, db::PoolWrapper, model::AccountActions};

pub async fn create_deposit_transaction(
    pool: &PoolWrapper,
    deriv: &str,
    amount: u64,
    aggregated_address: &str,
    statechain_id: &str,
) -> Result<String> {
    let (account, mut unlocker) = account::get_account(deriv)?;
    let selected_utxos = account::get_utxos_set(&account.get_addr(), amount).await?;

    let mut fee: u64 = 0;
    let input: Vec<TxIn> = selected_utxos
        .iter()
        .map(|utxo| {
            fee += utxo.value;
            println!("utxos set: {}", utxo.value);
            TxIn {
                previous_output: OutPoint::new(utxo.txid.parse().unwrap(), utxo.vout.into()),
                script_sig: ScriptBuf::from_bytes(vec![]),
                sequence: Sequence::MAX,
                witness: Witness::new(),
            }
        })
        .collect();

    // Setup output
    let mut output: Vec<TxOut> = Vec::new();
    {
        let (change, overflow) = fee.overflowing_sub(amount + BASE_TX_FEE);
        if overflow {
            return Err(anyhow!("Total input cannot cover amount and fee"));
        }
        // Transfer to aggregated_address
        let addr = Address::from_str(aggregated_address)?;
        let checked_addr = addr.require_network(Network::Testnet)?;
        output.push(TxOut {
            value: Amount::from_sat(amount),
            script_pubkey: checked_addr.script_pubkey(),
        });
        // Set change (if needed)
        if change > 0 {
            output.push(TxOut {
                value: Amount::from_sat(change),
                script_pubkey: account.get_checked_addr().script_pubkey(),
            });
        }
    };

    let deposit_tx = Transaction {
        version: Version::TWO,
        lock_time: absolute::LockTime::ZERO,
        input,
        output,
    };

    let mut unsigned_deposit_tx = deposit_tx.clone();

    let secp = Secp256k1::new();
    let sighash_type = EcdsaSighashType::All;
    let mut sighasher = SighashCache::new(&mut unsigned_deposit_tx);

    let future_tasks: Vec<_> = deposit_tx
        .input
        .iter()
        .enumerate()
        .map(|(index, input)| tokio::spawn(account::find_and_join_txn(index, input.clone())))
        .collect();

    let mut results = Vec::new();
    for job in future_tasks {
        results.push(job.await??);
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
    println!("deposit transaction hash: {:?}", tx_hex);
    println!("deposit transaction hash: {:#?}", unsigned_deposit_tx);
    let funding_txid = unsigned_deposit_tx.txid().to_string();
    let funding_vout = 1_u64;
    let _ = pool
        .update_deposit_tx(
            statechain_id,
            &funding_txid,
            funding_vout,
            "CONFIRM",
            &tx_hex,
        )
        .await?;

    Ok(funding_txid)
}
