// pub async fn get_tx(txid: &str) -> Result<models::Transaction, Error> {
//     let url = format!("https://blockstream.info/testnet/api/tx/{}", txid);
//
//     let response: models::Transaction = reqwest::get(&url).await?.json().await?;
//
//     println!("{:?}", response);
//     Ok(response)
// }
