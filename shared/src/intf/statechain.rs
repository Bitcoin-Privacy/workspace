use serde::{Deserialize, Serialize};

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

#[cfg_attr(feature = "backend", derive(Serialize))]
#[cfg_attr(feature = "frontend", derive(Deserialize))]
#[derive(Debug, Clone)]
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
    pub txn_bk: String, // hex
}

#[cfg_attr(feature = "backend", derive(Serialize))]
#[cfg_attr(feature = "frontend", derive(Deserialize))]
#[derive(Debug, Clone)]
pub struct CreateBkTxnRes {
    pub signed_txn_bk: String,
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
    pub addr: String,
}

#[cfg_attr(feature = "backend", derive(Serialize))]
#[cfg_attr(feature = "frontend", derive(Deserialize))]
#[derive(Debug, Clone)]
pub struct ListStatecoinsRes {
    // pub id: String,
    // pub base_amount: u32,
    // pub no_peer: u8, // should limit number of peer for a room, <= 255
    // pub status: u8, // WaitForNewParticipant=0, WaitForSignature=1, Submitting=2, Success=3, Failed=4
    // pub due1: u32,  // 3h -> 3*24*60*1000
    // pub due2: u32,  // 3h -> 3*24*60*1000 calc from due01 -> total time = due01 + due02
    // pub created_at: u64,
    // pub updated_at: u64,
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
