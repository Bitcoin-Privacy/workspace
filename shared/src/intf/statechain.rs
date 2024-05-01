use std::{str::SplitTerminator, string};

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "backend", derive(Deserialize))]
#[cfg_attr(feature = "frontend", derive(Serialize))]
#[derive(Debug, Clone,)]
pub struct KeyRegisterReq {
    pub statechain_id: String,
    pub signed_id: String,
    pub auth_pubkey_2: String,
}

#[cfg_attr(feature = "backend", derive(Serialize))]
#[cfg_attr(feature = "frontend", derive(Deserialize))]
#[derive(Debug, Clone,)]
pub struct KeyRegisterRes {
    pub random_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositInfo {
    pub aggregated_address: String,
    pub deposit_tx_hex: String,
}

// ---------------------------
// create token
// ---------------------------
#[cfg_attr(feature = "backend", derive(Deserialize))]
#[cfg_attr(feature = "frontend", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct CreateTokenReq {
    pub user_name: String,
}

// ---------------------------
// Deposit
// ---------------------------
#[cfg_attr(feature = "backend", derive(Deserialize))]
#[cfg_attr(feature = "frontend", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct DepositReq {
    pub token_id: String,
    pub addr: String,
    pub amount: u32, // Limitation for coinjoin transaction, only able to transfer 2^32 - 1 satoshis
}

// ---------------------------
// Sign first request
// ---------------------------
#[cfg_attr(feature = "backend", derive(Deserialize))]
#[cfg_attr(feature = "frontend", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct SignFirstReq {
    pub statechain_id: String,
    pub r2_commitment: String,
    pub blind_commitment: String,
    pub signed_statechain_id: String,
}

// ---------------------------
// Sign first response
// ---------------------------
#[cfg_attr(feature = "backend", derive(Deserialize))]
//#[cfg_attr(feature = "frontend", derive(Serialize))]
#[derive(Debug, Clone, Serialize)]
pub struct SignFirstRes {
    pub server_pub_nonce: String,
}

// ---------------------------
// Sign second
// ---------------------------
#[cfg_attr(feature = "backend", derive(Deserialize))]
#[cfg_attr(feature = "frontend", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct SignSecondReq {
    pub statechain_id: String,
    pub negate_seckey: u8,
    pub session: String,
    pub signed_statechain_id: String,
    pub server_pub_nonce: String,
}

// #[cfg_attr(feature = "backend", derive(Serialize))]
#[cfg_attr(feature = "frontend", derive(Deserialize))]
#[derive(Debug, Clone, Serialize)]
pub struct DepositRes {
    pub se_pubkey_1: String,
    pub statechain_id: String,
}

// ---------------------------
// Create Backup Transaction output
// ---------------------------
#[cfg_attr(feature = "backend", derive(Deserialize))]
#[cfg_attr(feature = "frontend", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct CreateBkTxnReq {
    pub statechain_id: String,
    pub scriptpubkey: String,
    pub txn_bk: String, // hex
}

// #[cfg_attr(feature = "backend", derive(Serialize))]
// #[cfg_attr(feature = "frontend", derive(Deserialize))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBkTxnRes {
    pub sig: String,
    pub rand_key: String,
}

// ---------------------------
// Transfer
// ---------------------------
#[cfg_attr(feature = "backend", derive(Deserialize))]
#[cfg_attr(feature = "frontend", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct TransferReq {
    pub encrypted_msg: String,
    pub addr: String,
}

#[cfg_attr(feature = "backend", derive(Serialize))]
#[cfg_attr(feature = "frontend", derive(Deserialize))]
#[derive(Debug, Clone)]
pub struct TransferRes {
    pub status: u8,
}

// ---------------------------
// Get list statecoin
// ---------------------------
#[cfg_attr(feature = "backend", derive(Deserialize))]
#[cfg_attr(feature = "frontend", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct ListStatecoinsReq {
    pub authkey: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatecoinInfo {
    pub statechain_id: String,
    pub amount: u64,
    pub sequence: u64,
    pub txid: String,
    pub vout: String,
}

#[cfg_attr(feature = "backend", derive(Serialize))]
#[cfg_attr(feature = "frontend", derive(Deserialize))]
#[derive(Debug, Clone)]
pub struct ListStatecoinsRes {
    pub statecoins: Vec<StatecoinInfo>,
}

// ---------------------------
// Update key
// ---------------------------
#[cfg_attr(feature = "backend", derive(Deserialize))]
#[cfg_attr(feature = "frontend", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct UpdateKeyReq {
    pub t: String,
}

#[cfg_attr(feature = "backend", derive(Serialize))]
#[cfg_attr(feature = "frontend", derive(Deserialize))]
#[derive(Debug, Clone)]
pub struct UpdateKeyRes {
    pub status: u8,
}

#[cfg_attr(feature = "backend", derive(Deserialize))]
#[cfg_attr(feature = "frontend", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct GetNonceReq {
    pub signed_statechain_id: String,
}

#[cfg_attr(feature = "backend", derive(Serialize))]
#[cfg_attr(feature = "frontend", derive(Deserialize))]
#[derive(Debug, Clone)]
pub struct GetNonceRes {
    pub server_nonce: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2trUtxo<'a> {
    pub txid: &'a str,
    pub vout: u32,
    pub script_pubkey: &'a str,
    pub pubkey: &'a str,
    pub master_fingerprint: &'a str,
    pub amount_in_sats: u64,
    pub pubderivation_path: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DepositTx {
    pub txid: String,
    pub vout: String,
}

#[cfg_attr(feature = "backend", derive(Serialize))]
#[cfg_attr(feature = "frontend", derive(Deserialize))]
#[derive(Debug, Clone)]
pub struct BkTxSignRes {
    pub sign: String,
    pub rand: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPartialSignatureReq {
    pub serialized_key_agg_ctx: String,
    pub signed_statechain_id: String,
    pub parsed_tx: String,
    pub agg_pubnonce: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPartialSignatureRes {
    pub partial_signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferMessageReq {
    pub transfer_msg: String,
    pub authkey: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferMessage {
    pub total_owner: i64,
    pub backup_txs: Vec<String>,
    pub t1: String,
    pub statechain_id: String,
    pub agg_pubkey: String,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct GetStatecoinsReq {
//     pub authkey: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct GetStatecoinsRes {
//     pub statecoins: Vec<StatecoinInfo>,
// }
