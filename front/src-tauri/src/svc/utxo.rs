use shared::model::Utxo;

pub async fn get_utxo(address: String) -> Result<Vec<Utxo>, String> {
    let url = format!(
        "https://blockstream.info/testnet/api/address/{}/utxo",
        address
    );

    let response: Vec<Utxo> = reqwest::get(&url).await.unwrap().json().await.unwrap();

    Ok(response)
}

pub async fn get_balance(address: String) -> Result<u64, String> {
    let utxos = get_utxo(address).await?;
    Ok(utxos.iter().map(|utxo| utxo.value).sum())
}
