use anyhow::Result;
use bitcoin::script;
use reqwest::{Client, Response};
use serde::Serialize;
use shared::intf::statechain::{
    self, CreateBkTxnReq, CreateBkTxnRes, GetNonceReq, GetNonceRes, GetPartialSignatureReq,
    GetPartialSignatureRes, GetTransferMessageReq, GetTransferMessageRes, KeyRegisterReq,
    KeyRegisterRes, ListStatecoinsReq, TransferMessage, TransferMessageReq, UpdateKeyReq,
    UpdateKeyRes, VerifyStatecoinReq, VerifyStatecoinRes,
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
        .post(&format!("statechain/{}/sig", statechain_id), &body)
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

pub async fn register_new_owner(
    conn: &NodeConnector,
    statechain_id: &str,
    signed_statechain_id: &str,
    auth_pubkey_2: &str,
) -> Result<KeyRegisterRes> {
    let req = KeyRegisterReq {
        statechain_id: statechain_id.to_string(),
        signed_id: signed_statechain_id.to_string(),
        auth_pubkey_2: auth_pubkey_2.to_string(),
    };

    let body = serde_json::to_value(req)?;
    let res = conn.post("statechain/transfer/key-register", &body).await?;
    let json: KeyRegisterRes = serde_json::from_value(res)?;
    println!("Register key response {:#?}", json);
    Ok(json)
}

pub async fn create_transfer_msg(
    conn: &NodeConnector,
    encrypted_msg: &str,
    auth_pubkey_2: &str,
) -> Result<()> {
    let req = TransferMessageReq {
        transfer_msg: encrypted_msg.to_string(),
        authkey: auth_pubkey_2.to_string(),
    };

    let body = serde_json::to_value(req)?;
    let res = conn
        .post("statechain/transfer/transfer-message", &body)
        .await?;
    let json: KeyRegisterRes = serde_json::from_value(res)?;
    println!("send transfer message {:#?}", json);
    Ok(())
}

pub async fn get_transfer_msg(
    conn: &NodeConnector,
    auth_pubkey: &str,
) -> Result<Option<GetTransferMessageRes>> {
    let res = conn
        .get(
            format!(
                "statechain/transfer/transfer-message/{auth_key}",
                auth_key = auth_pubkey
            ),
            None,
        )
        .await?;

    if res.is_null() {
        println!("Transfer message is null for authkey: {}", auth_pubkey);
        return Ok(None);
    }
    let json: GetTransferMessageRes = serde_json::from_value(res)?;

    println!("Received transfer message: {:#?}", json);
    Ok(Some(json))
}

pub async fn get_verification_statecoin(
    conn: &NodeConnector,
    authkey: &str,
    statechain_id: &str,
    signed_msg: &str,
) -> Result<Option<VerifyStatecoinRes>> {
    let req = VerifyStatecoinReq {
        statechain_id: statechain_id.to_string(),
        signed_msg: signed_msg.to_string(),
        authkey: authkey.to_string(),
    };
    let body = serde_json::to_value(req)?;
    let res = conn
        .post("statechain/transfer/transfer-message/verify", &body)
        .await?;

    if res.is_null() {
        println!(
            "Null value for statecoin transfer verification: {}",
            authkey
        );
        return Ok(None);
    }
    let json: VerifyStatecoinRes = serde_json::from_value(res)?;

    Ok(Some(json))
}

pub async fn update_new_key(
    conn: &NodeConnector,
    t2: &str,
    signed_msg: &str,
    statechain_id: &str,
    authkey: &str,
) -> Result<Option<UpdateKeyRes>> {
    let req = UpdateKeyReq {
        authkey: authkey.to_string(),
        t2: t2.to_string(),
        statechain_id: statechain_id.to_string(),
        signed_msg: signed_msg.to_string(),
    };
    let body = serde_json::to_value(req)?;
    let res = conn
        .post("statechain/transfer/transfer-message/update-key", &body)
        .await?;

    if res.is_null() {
        return Ok(None);
    }
    let json: UpdateKeyRes = serde_json::from_value(res)?;

    Ok(Some(json))
}
