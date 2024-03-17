use bonsaidb::core::keyvalue::Value;
use shared::{
    intf::coinjoin::{
        GetStatusReq, GetStatusRes, GetUnsignedTxnReq, GetUnsignedTxnRes, RegisterReq, RegisterRes,
        SetOutputReq, SetOutputRes, SignTxnReq, SignTxnRes,
    },
    model::Utxo,
};

use crate::connector::NodeConnector;

pub struct CoinjoinApis {}

impl CoinjoinApis {
    pub async fn register(
        input_coins: Vec<Utxo>,
        blinded_output_address: &str,
        change_address: &str,
        amount: u64,
    ) -> Result<RegisterRes, String> {
        let conn = NodeConnector::new();
        let req = RegisterReq {
            utxos: input_coins,
            proofs: vec![],
            blinded_output_address: blinded_output_address.to_string(),
            change_address: change_address.to_string(),
            amount: amount as u32,
        };
        let body = serde_json::to_value(req).unwrap();
        let res = conn.post("coinjoin/register", &body).await;
        match res {
            Ok(value) => serde_json::from_value::<RegisterRes>(value).map_err(|e| e.to_string()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn set_output(
        room_id: &str,
        output_address: &str,
        signed_blinded_output_address: &str,
    ) -> Result<SetOutputRes, String> {
        let conn = NodeConnector::new();
        let req = SetOutputReq {
            room_id: room_id.to_string(),
            output_address: output_address.to_string(),
            sig: signed_blinded_output_address.to_string(),
        };
        let body = serde_json::to_value(req).unwrap();
        let res = conn.post("coinjoin/output", &body).await;
        match res {
            Ok(value) => serde_json::from_value::<SetOutputRes>(value).map_err(|e| e.to_string()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn sign(room_id: &str, vins: Vec<u16>, txn: &str) -> Result<SignTxnRes, String> {
        let conn = NodeConnector::new();
        let req = SignTxnReq {
            room_id: room_id.to_string(),
            vins,
            txn: txn.to_string(),
        };
        let body = serde_json::to_value(req).unwrap();
        let res = conn.post("coinjoin/sign", &body).await;
        match res {
            Ok(value) => serde_json::from_value::<SignTxnRes>(value).map_err(|e| e.to_string()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn get_transaction(room_id: &str) -> Result<GetUnsignedTxnRes, String> {
        let conn = NodeConnector::new();
        let res = conn
            .get(format!("coinjoin/room/{id}/tx", id = room_id))
            .await;
        match res {
            Ok(value) => {
                serde_json::from_value::<GetUnsignedTxnRes>(value).map_err(|e| e.to_string())
            }
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn get_status(room_id: &str) -> Result<GetStatusRes, String> {
        let conn = NodeConnector::new();
        let res = conn
            .get(format!("coinjoin/room/{id}/status", id = room_id))
            .await;
        match res {
            Ok(value) => serde_json::from_value::<GetStatusRes>(value).map_err(|e| e.to_string()),
            Err(e) => Err(e.to_string()),
        }
    }
}
