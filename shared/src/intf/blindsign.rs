use serde::{Deserialize, Serialize};

// ---------------------------
// Get blind session
// ---------------------------
#[cfg_attr(feature = "backend", derive(Serialize))]
#[cfg_attr(feature = "frontend", derive(Deserialize))]
#[derive(Debug, Clone)]
pub struct GetBlindSessionRes {
    pub publickey: String, // Hex
    pub rp: String,        //Hex
}
