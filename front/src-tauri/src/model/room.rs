use serde::{Deserialize, Serialize};

use shared::{intf::coinjoin::CoinjoinRegisterRes, model::Utxo};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoomEntity {
    pub id: String,
    pub base_amount: u32,
    pub no_peer: u8, // should limit number of peer for a room, <= 255
    pub status: u8, // WaitForNewParticipant=0, WaitForSignature=1, Submitting=2, Success=3, Failed=4
    pub due1: u32,  // 3h -> 3*24*60*1000
    pub due2: u32,  // 3h -> 3*24*60*1000 calc from due01 -> total time = due01 + due02
    pub created_at: u64,
    pub updated_at: u64,
    pub utxos: Vec<Utxo>,
}

impl From<CoinjoinRegisterRes> for RoomEntity {
    fn from(value: CoinjoinRegisterRes) -> Self {
        RoomEntity {
            id: value.room.id.clone(),
            base_amount: value.room.base_amount,
            no_peer: value.room.no_peer,
            status: value.room.status,
            due1: value.room.due1,
            due2: value.room.due2,
            created_at: value.room.created_at,
            updated_at: value.room.updated_at,
            utxos: value.utxos.clone(),
        }
    }
}
