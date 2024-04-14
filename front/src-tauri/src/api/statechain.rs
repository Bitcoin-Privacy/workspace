use anyhow::Result;
use bitcoin::script;
use shared::intf::statechain::{
    self, CreateBkTxnReq, CreateBkTxnRes, GetNonceReq, GetNonceRes, GetPartialSignatureReq,
    GetPartialSignatureRes,
};

use crate::connector::NodeConnector;

pub async fn get_nonce(
    conn: &NodeConnector,
    statechain_id: &str,
    signed_statechain_id: &str,
) -> Result<GetNonceRes> {
    let req = GetNonceReq {
        statechain_id: statechain_id.to_string(),
        signed_statechain_id: signed_statechain_id.to_string(),
    };

    println!("get nonce : {:#?}", req);

    let body = serde_json::to_value(req)?;
    let res = conn.post("statechain/nonce", &body).await?;
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
        serialized_key_agg_ctx : serialized_key_agg_ctx.to_string(),
        statechain_id: statechain_id.to_string(),
        signed_statechain_id: signed_statechain_id.to_string(),
        parsed_tx: parsed_tx.to_string(),
        agg_pubnonce: agg_pubnonce.to_string(),
    };

    let body = serde_json::to_value(req)?;
    let res = conn.post("statechain/get-sig", &body).await?;
    let json: GetPartialSignatureRes = serde_json::from_value(res)?;
    println!("Sign partial signature {:#?}", json);
    Ok(json)
}
