use shared::intf::coinjoin::{GetRoomByIdRes, RoomDto};

// ---------------------------
// Room table
// ---------------------------
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Room {
    pub id: uuid::Uuid,
    #[sqlx(try_from = "i64")]
    pub base_amount: u32,
    #[sqlx(try_from = "i16")]
    pub no_peer: u8, // should limit number of peer for a room, <= 255
    #[sqlx(try_from = "i16")]
    pub status: u8, // WaitForNewParticipant=0, WaitForSignature=1, Submitting=2, Success=3, Failed=4
    #[sqlx(try_from = "i64")]
    pub due1: u32, // 3h -> 3*24*60*1000
    #[sqlx(try_from = "i64")]
    pub due2: u32, // 3h -> 3*24*60*1000 calc from due01 -> total time = due01 + due02
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Room> for RoomDto {
    fn from(value: Room) -> Self {
        RoomDto {
            id: value.id.to_string(),
            base_amount: value.base_amount,
            no_peer: value.no_peer,
            status: value.status,
            due1: value.due1,
            due2: value.due2,
            created_at: value.created_at.timestamp_millis() as u64,
            updated_at: value.updated_at.timestamp_millis() as u64,
        }
    }
}

impl From<Room> for GetRoomByIdRes {
    fn from(value: Room) -> Self {
        GetRoomByIdRes {
            id: value.id.to_string(),
            base_amount: value.base_amount,
            no_peer: value.no_peer,
            status: value.status,
            due1: value.due1,
            due2: value.due2,
            created_at: value.created_at.timestamp_millis() as u64,
            updated_at: value.updated_at.timestamp_millis() as u64,
        }
    }
}

// ---------------------------
// Input table
// ---------------------------
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Input {
    pub id: uuid::Uuid,
    pub room_id: uuid::Uuid,
    pub txid: String,
    #[sqlx(try_from = "i32")]
    pub vout: u16,
    #[sqlx(try_from = "i64")]
    pub amount: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// ---------------------------
// Output table
// ---------------------------
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Output {
    pub id: uuid::Uuid,
    pub room_id: uuid::Uuid,
    pub address: String,
    #[sqlx(try_from = "i64")]
    pub amount: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// ---------------------------
// Proof table
// ---------------------------
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Proof {
    pub id: uuid::Uuid,
    pub room_id: uuid::Uuid,
    #[sqlx(try_from = "i32")]
    pub vin: u16,
    pub script: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
