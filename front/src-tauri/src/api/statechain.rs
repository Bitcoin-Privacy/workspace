use anyhow::Result;
use bitcoin::script;
use reqwest::{Client, Response};
use shared::intf::statechain::{
    self, CreateBkTxnReq, CreateBkTxnRes, GetNonceReq, GetNonceRes, GetPartialSignatureReq,
    GetPartialSignatureRes, ListStatecoinsReq,
};
use tauri::http::Uri;

extern crate reqwest;

use crate::connector::NodeConnector;

pub async fn get_nonce(
    conn: &NodeConnector,
    statechain_id: &str,
    signed_statechain_id: &str,
) -> Result<GetNonceRes> {
    let req = GetNonceReq {
        signed_statechain_id: signed_statechain_id.to_string(),
    };

    println!("get nonce : {:#?}", req);

    let body = serde_json::to_value(req)?;
    let res = conn
        .post(&format!("statechain/{}/nonce", statechain_id), &body)
        .await?;
    let json: GetNonceRes = serde_json::from_value(res)?;
    println!("Deposit {:#?}", json);
    Ok(json)
}

pub async fn request_sign_bk_tx(
    conn: &NodeConnector,
    statechain_id: &str,
    txn_bk: &str,
    scriptpubkey: &str,
) -> Result<CreateBkTxnRes> {
    let req = CreateBkTxnReq {
        statechain_id: statechain_id.to_string(),
        scriptpubkey: scriptpubkey.to_string(),
        txn_bk: txn_bk.to_string(),
    };

    let body = serde_json::to_value(req)?;
    let res = conn.post("statechain/create-bk-txn", &body).await?;
    let json: CreateBkTxnRes = serde_json::from_value(res)?;
    println!("Sign backup transaction {:#?}", json);
    Ok(json)
}

pub async fn get_partial_signature(
    conn: &NodeConnector,
    serialized_key_agg_ctx: &str,
    statechain_id: &str,
    signed_statechain_id: &str,
    parsed_tx: &str,
    agg_pubnonce: &str,
) -> Result<GetPartialSignatureRes> {
    let req = GetPartialSignatureReq {
        serialized_key_agg_ctx: serialized_key_agg_ctx.to_string(),
        signed_statechain_id: signed_statechain_id.to_string(),
        parsed_tx: parsed_tx.to_string(),
        agg_pubnonce: agg_pubnonce.to_string(),
    };

    let body = serde_json::to_value(req)?;
    let res = conn
        .post(&format!("statechain/{}/get-sig", statechain_id), &body)
        .await?;
    let json: GetPartialSignatureRes = serde_json::from_value(res)?;
    println!("Sign partial signature {:#?}", json);
    Ok(json)
}

pub async fn broadcast_tx(tx_hex: String) -> Result<String, String> {
    let url = "https://blockstream.info/testnet/api/tx";
    let client = reqwest::Client::new();
    let res = client.post(url).body(tx_hex).send().await;

    match res {
        Ok(res) => Ok(res.json().await.unwrap()),
        Err(err) => Err(err.to_string()),
    }
}
