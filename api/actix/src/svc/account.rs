use shared::{api, intf::coinjoin::ProofSignature, model::Utxo};

use std::str::FromStr;

use bitcoin::{Address, Network};

pub fn parse_addr_from_str(raw_addr: &str, network: Network) -> Result<Address, String> {
    match Address::from_str(&raw_addr)
        .map_err(|e| e.to_string())
        .and_then(|addr| addr.require_network(network).map_err(|e| e.to_string()))
    {
        Ok(a) => Ok(a),
        Err(e) => Err(e),
    }
}

/// UTXO Validator
///
/// Check whether UTXO is valid or not
///
/// * `utxo`: UTXO
pub async fn utxo_validator(utxo: Utxo) -> bool {
    let _ = api::get_tx_outspend(&utxo.txid, utxo.vout).await;
    true
}

pub async fn validate_utxos(utxos: &Vec<Utxo>) -> bool {
    let tasks = utxos
        .iter()
        .map(|utxo| tokio::spawn(utxo_validator(utxo.clone())))
        .collect::<Vec<_>>();

    let mut results = Vec::new();
    for job in tasks {
        results.push(job.await.unwrap());
    }

    results.into_iter().all(|is_valid| is_valid)
}

pub fn proof_validator(utxo: &Utxo, proof: &ProofSignature) -> bool {
    true
}
