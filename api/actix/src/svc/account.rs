use shared::{api, intf::coinjoin::ProofSignature, model::Utxo};

use anyhow::Result;
use std::str::FromStr;

use bitcoin::{Address, Network};

pub fn parse_addr_from_str(raw_addr: &str, network: Network) -> Result<Address> {
    Ok(Address::from_str(raw_addr)?.require_network(network)?)
}

/// UTXO Validator
///
/// Check whether UTXO is valid or not
///
/// * `utxo`: UTXO
pub async fn utxo_validator(utxo: Utxo) -> Result<bool, String> {
    let _ = api::get_tx_outspend(&utxo.txid, utxo.vout).await;
    Ok(true)
}

pub async fn validate_utxos(utxos: &[Utxo]) -> Result<(), String> {
    let tasks = utxos
        .iter()
        .map(|utxo| tokio::spawn(utxo_validator(utxo.clone())))
        .collect::<Vec<_>>();

    let mut results = Vec::new();
    for job in tasks {
        results.push(job.await.map_err(|e| e.to_string())??);
    }

    if results.into_iter().all(|is_valid| is_valid) {
        Ok(())
    } else {
        Err("Invalid utxos".to_string())
    }
}

pub fn proof_validator(utxo: &Utxo, proof: &ProofSignature) -> bool {
    true
}
