use anyhow::Result;

use crate::model::{Status, Txn, Utxo};

pub fn uri(endpoint: &str) -> String {
    format!("https://blockstream.info/testnet/api/{}", endpoint)
}

pub async fn get_onchain_tx(txid: &str) -> Result<Txn> {
    let url = uri(&format!("tx/{}", txid));

    let response: Txn = reqwest::get(&url).await?.json().await?;

    Ok(response)
}

pub async fn get_tx_outspend(txid: &str, vout: u16) -> Result<()> {
    let url = uri(&format!("tx/{}/outspend/{}", txid, vout));

    let _ = reqwest::get(&url).await?.json().await?;

    Ok(())
}

pub async fn get_utxo(address: &str) -> Result<Vec<Utxo>> {
    let url = format!(
        "https://blockstream.info/testnet/api/address/{}/utxo",
        address
    );
    println!("utxo, {}", address);

    let response: Vec<Utxo> = reqwest::get(&url).await.unwrap().json().await.unwrap();

    Ok(response)
}
pub async fn get_status(txid: &str) -> Result<bool> {
    let url = format!("https://blockstream.info/testnet/api/tx/{}/status", txid);
    println!("txid, {}", txid);

    let response = reqwest::get(&url).await?.json::<Status>().await?;
    println!("status, {:?}", response);

    Ok(response.confirmed)
}

pub async fn get_transaction_existence(txid: &str) -> Result<bool> {
    let url = format!("https://blockstream.info/testnet/api/tx/{}", txid);
    println!("txid, {}", txid);
    let response = reqwest::get(&url).await;
    match response {
        Ok(res) => {
            println!("status: {:?}", res.status());
            Ok(false)
        }
        Err(err) => {
            println!("Error: {:?}", err);
            Ok(true)
        }
    }
}

pub async fn get_balance(address: &str) -> Result<u64> {
    let utxos = get_utxo(address).await?;
    Ok(utxos.iter().map(|utxo| utxo.value).sum())
}
