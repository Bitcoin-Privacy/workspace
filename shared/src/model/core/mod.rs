use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub block_hash: String,
    pub block_height: u64,
    pub block_time: u64,
    pub confirmed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Prevout {
    pub scriptpubkey: String,
    pub scriptpubkey_address: String,
    pub scriptpubkey_asm: String,
    pub scriptpubkey_type: String,
    pub value: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vin {
    pub is_coinbase: bool,
    pub prevout: Prevout,
    pub scriptsig: String,
    pub scriptsig_asm: String,
    pub sequence: u64,
    pub txid: String,
    pub vout: u64,
    pub witness: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vout {
    pub scriptpubkey: String,
    pub scriptpubkey_address: String,
    pub scriptpubkey_asm: String,
    pub scriptpubkey_type: String,
    pub value: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Txn {
    pub fee: u64,
    pub locktime: u64,
    pub size: u64,
    pub status: Status,
    pub txid: String,
    pub version: u64,
    pub vin: Vec<Vin>,
    pub vout: Vec<Vout>,
    pub weight: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Utxo {
    pub txid: String,
    pub vout: u16,
    pub value: u64,
}
