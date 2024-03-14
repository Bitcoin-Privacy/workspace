use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoinJoinRegisterCompleteEvent {
    pub room_id: String,
    pub status: u8,
}
