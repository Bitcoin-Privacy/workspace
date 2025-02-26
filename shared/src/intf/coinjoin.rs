use serde::{Deserialize, Serialize};

use crate::model::Utxo;

use super::PaginationQuery;

#[cfg_attr(feature = "frontend", derive(Deserialize))]
#[derive(Debug, Clone, Serialize)]
pub struct RoomDto {
    pub id: String,
    pub base_amount: u32,
    pub no_peer: u8, // should limit number of peer for a room, <= 255
    pub status: u8, // WaitForNewParticipant=0, WaitForSignature=1, Submitting=2, Success=3, Failed=4
    pub due1: u32,  // 3h -> 3*24*60*1000
    pub due2: u32,  // 3h -> 3*24*60*1000 calc from due01 -> total time = due01 + due02
    pub txid: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

// Struct for the proof signature. This can be a simple string or a more complex structure
#[cfg_attr(feature = "backend", derive(Deserialize))]
#[cfg_attr(feature = "frontend", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct ProofSignature {
    pub signature: String,
}

// ---------------------------
// Register
// ---------------------------
#[cfg_attr(feature = "backend", derive(Deserialize))]
#[cfg_attr(feature = "frontend", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct CoinjoinRegisterReq {
    pub utxos: Vec<Utxo>,            // List of UTXOs the user wants to register
    pub proofs: Vec<ProofSignature>, // Proof signatures associated with the UTXOs
    pub blinded_out_addr: String,    // Blinded set of output address
    pub change_addr: String,         // Cleartext
    pub amount: u32, // Limitation for coinjoin transaction, only able to transfer 2^32 - 1 satoshis
}

#[cfg_attr(feature = "backend", derive(Serialize))]
#[cfg_attr(feature = "frontend", derive(Deserialize))]
#[derive(Debug, Clone)]
pub struct CoinjoinRegisterRes {
    pub room: RoomDto,
    pub utxos: Vec<Utxo>,
    pub signed_blined_output: String,
}

// ---------------------------
// Set output
// ---------------------------
#[cfg_attr(feature = "backend", derive(Deserialize))]
#[cfg_attr(feature = "frontend", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct SetOutputReq {
    pub room_id: String,
    pub out_addr: String,
    pub sig: String,
}

#[cfg_attr(feature = "backend", derive(Serialize))]
#[cfg_attr(feature = "frontend", derive(Deserialize))]
#[derive(Debug, Clone)]
pub struct SetOutputRes {
    pub status: u8, // Transaction in hex form
}

// ---------------------------
// Set signature
// ---------------------------
#[cfg_attr(feature = "backend", derive(Deserialize))]
#[cfg_attr(feature = "frontend", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct SignTxnReq {
    pub room_id: String,
    pub address: String,
    pub vins: Vec<u16>,
    pub txn: String, // transaction hex
}

#[cfg_attr(feature = "backend", derive(Serialize))]
#[cfg_attr(feature = "frontend", derive(Deserialize))]
#[derive(Debug, Clone)]
pub struct SignTxnRes {
    pub status: u8,
}

// ---------------------------
// Get room by id
// ---------------------------
#[cfg_attr(feature = "backend", derive(Deserialize))]
#[cfg_attr(feature = "frontend", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct RoomQueryReq {
    pub id: String,
}

#[cfg_attr(feature = "backend", derive(Serialize))]
#[cfg_attr(feature = "frontend", derive(Deserialize))]
#[derive(Debug, Clone)]
pub struct GetRoomByIdRes {
    pub room: RoomDto,
    pub utxo: Vec<Utxo>,
    pub signed: u8,
}

// ---------------------------
// Get room status
// ---------------------------
#[cfg_attr(feature = "frontend", derive(Deserialize))]
#[derive(Debug, Clone, Serialize)]
pub struct GetStatusRes {
    pub status: u8, // Transaction in hex form
}

// ---------------------------
// Get unsigned transaction
// ---------------------------
#[cfg_attr(feature = "frontend", derive(Deserialize))]
#[derive(Debug, Clone, Serialize)]
pub struct GetUnsignedTxnRes {
    pub tx: String, // Transaction in hex form
}

#[cfg_attr(feature = "backend", derive(Deserialize))]
#[cfg_attr(feature = "frontend", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct RoomListQuery {
    pub pagination: Option<PaginationQuery>,
    pub address: String,
}

#[cfg_attr(feature = "backend", derive(Deserialize))]
#[cfg_attr(feature = "frontend", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct AddressQuery {
    pub address: String,
}

// ---------------------------
// Validate signature
// ---------------------------
#[cfg_attr(feature = "backend", derive(Deserialize))]
#[cfg_attr(feature = "frontend", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct ValidateSignatureReq {
    pub signature: String,
}

#[cfg_attr(feature = "backend", derive(Serialize))]
#[cfg_attr(feature = "frontend", derive(Deserialize))]
#[derive(Debug, Clone)]
pub struct ValidateSignatureRes {
    pub status: u8,
}

#[cfg_attr(feature = "backend", derive(Serialize))]
#[cfg_attr(feature = "frontend", derive(Deserialize))]
#[derive(Debug, Clone)]
pub struct CheckSignatureRes {
    pub status: u8,
}
