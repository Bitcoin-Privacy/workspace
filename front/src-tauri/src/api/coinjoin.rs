use anyhow::Result;
use serde_json::{json, Map};
use shared::{
    intf::coinjoin::{
        GetStatusRes, GetUnsignedTxnRes, RegisterReq, RegisterRes, RoomDto, SetOutputReq,
        SetOutputRes, SignTxnReq, SignTxnRes,
    },
    model::Utxo,
};

use crate::{cfg::CFG, connector::NodeConnector};

pub async fn register(
    conn: &NodeConnector,
    input_coins: Vec<Utxo>,
    blinded_output_address: &str,
    change_address: &str,
    amount: u64,
) -> Result<RegisterRes> {
    let req = RegisterReq {
        utxos: input_coins,
        proofs: vec![],
        blinded_out_addr: blinded_output_address.to_string(),
        change_addr: change_address.to_string(),
        amount: amount as u32,
    };
    let body = serde_json::to_value(req).unwrap();
    let res = conn.post("coinjoin/register", &body).await?;
    Ok(serde_json::from_value::<RegisterRes>(res)?)
}

pub async fn set_output(room_id: &str, out_addr: &str, sig: &str) -> Result<SetOutputRes> {
    let conn = NodeConnector::new(CFG.service_url.clone());
    let req = SetOutputReq {
        room_id: room_id.to_string(),
        out_addr: out_addr.to_string(),
        sig: sig.to_string(),
    };
    let body = serde_json::to_value(req).unwrap();
    let res = conn.post("coinjoin/output", &body).await?;
    Ok(serde_json::from_value::<SetOutputRes>(res)?)
}

pub async fn sign(room_id: &str, vins: Vec<u16>, txn: &str) -> Result<SignTxnRes> {
    let conn = NodeConnector::new(CFG.service_url.clone());
    let req = SignTxnReq {
        room_id: room_id.to_string(),
        vins,
        txn: txn.to_string(),
    };
    let body = serde_json::to_value(req).unwrap();
    let res = conn.post("coinjoin/sign", &body).await?;
    Ok(serde_json::from_value::<SignTxnRes>(res)?)
}

pub async fn get_txn(room_id: &str) -> Result<GetUnsignedTxnRes> {
    let conn = NodeConnector::new(CFG.service_url.clone());
    let res = conn
        .get(&format!("coinjoin/room/{id}/txn", id = room_id), None)
        .await?;
    Ok(serde_json::from_value::<GetUnsignedTxnRes>(res)?)
}

pub async fn get_status(room_id: &str) -> Result<GetStatusRes> {
    let conn = NodeConnector::new(CFG.service_url.clone());
    let res = conn
        .get(&format!("coinjoin/room/{id}/status", id = room_id), None)
        .await?;
    Ok(serde_json::from_value::<GetStatusRes>(res)?)
}

pub async fn get_rooms(address: &str) -> Result<Vec<RoomDto>> {
    let conn = NodeConnector::new(CFG.service_url.clone());
    let mut params = Map::new();
    params.insert("address".to_string(), json!(address));
    let res = conn.get("coinjoin/room/list", Some(params)).await?;
    Ok(serde_json::from_value::<Vec<RoomDto>>(res)?)
}
