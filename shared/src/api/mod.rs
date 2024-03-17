use crate::model::{Txn, Utxo};

pub fn uri(endpoint: &str) -> String {
    format!("https://blockstream.info/testnet/api/{}", endpoint)
}

pub async fn get_onchain_tx(txid: &str) -> Result<Txn, reqwest::Error> {
    let url = uri(&format!("tx/{}", txid));

    let response: Txn = reqwest::get(&url).await?.json().await?;

    Ok(response)
}

pub async fn get_tx_outspend(txid: &str, vout: u16) -> Result<(), reqwest::Error> {
    let url = uri(&format!("tx/{}/outspend/{}", txid, vout));

    let _ = reqwest::get(&url).await?.json().await?;

    Ok(())
}

pub async fn get_utxo(address: &str) -> Result<Vec<Utxo>, String> {
    let url = format!(
        "https://blockstream.info/testnet/api/address/{}/utxo",
        address
    );
    println!("utxo, {}", address);

    let response: Vec<Utxo> = reqwest::get(&url).await.unwrap().json().await.unwrap();

    Ok(response)
}

pub async fn get_balance(address: &str) -> Result<u64, String> {
    let utxos = get_utxo(address).await?;
    Ok(utxos.iter().map(|utxo| utxo.value).sum())
}
