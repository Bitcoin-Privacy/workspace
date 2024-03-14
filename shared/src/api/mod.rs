use crate::model::Txn;

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
