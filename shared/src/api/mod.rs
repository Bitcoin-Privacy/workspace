use anyhow::Result;
use serde_json::Value;

use crate::model::{Status, Txn, Utxo};

macro_rules! rpc_url {
    ($($arg:tt)*) => {{
        let res = format!("https://blockstream.info/testnet/api/{}", format_args!($($arg)*));
        res
    }}
}

pub async fn get_onchain_tx(txid: &str) -> Result<Txn> {
    let url = rpc_url!("tx/{txid}");

    let response: Txn = reqwest::get(&url).await?.json().await?;

    Ok(response)
}

pub async fn get_tx_outspend(txid: &str, vout: u16) -> Result<()> {
    let url = rpc_url!("tx/{txid}/outspend/{vout}");

    let res = reqwest::get(&url).await?.json::<Value>().await?;
    println!("Tx outspend result: {res}");

    Ok(())
}

pub async fn get_utxo(address: &str) -> Result<Vec<Utxo>> {
    let url = rpc_url!("address/{address}/utxo");
    println!("utxo, {address}");

    let response: Vec<Utxo> = reqwest::get(&url).await.unwrap().json().await.unwrap();

    Ok(response)
}
pub async fn get_status(txid: &str) -> Result<bool> {
    let url = rpc_url!("tx/{txid}/status");
    println!("txid, {txid}");

    let response = reqwest::get(&url).await?.json::<Status>().await?;
    println!("status, {:?}", response);

    Ok(response.confirmed)
}

pub async fn get_transaction_existence(txid: &str) -> Result<bool> {
    let url = rpc_url!("tx/{txid}");
    println!("txid, {}", txid);
    let response = reqwest::get(&url).await;
    match response {
        Ok(res) => {
            println!("status: {:?}", res.status());
            Ok(false)
        }
        Err(err) => {
            println!("Error: {err:?}");
            Ok(true)
        }
    }
}

pub async fn get_balance(address: &str) -> Result<u64> {
    let utxos = get_utxo(address).await?;
    Ok(utxos.iter().map(|utxo| utxo.value).sum())
}

pub async fn broadcast_tx(tx_hex: String) -> Result<String> {
    let url = rpc_url!("tx");
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .header("Content-Type", "text/plain")
        .body(tx_hex)
        .send()
        .await?;
    if res.status().is_success() {
        Ok(res.text().await?)
    } else {
        Err(anyhow::anyhow!("Broadcast error: {}", res.text().await?))
    }
}
