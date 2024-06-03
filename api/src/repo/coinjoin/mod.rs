use crate::model::entity::coinjoin::{Input, Output, Proof, RoomEntity};
use async_trait::async_trait;

mod source;
pub use source::CoinjoinRepo;

pub type CoinjoinError = String;
pub type CoinjoinResult<T> = Result<T, CoinjoinError>;

#[async_trait]
pub trait TraitCoinJoinRepo: Send + Sync + 'static {
    async fn get_rooms(&self) -> CoinjoinResult<Vec<RoomEntity>>;
    async fn get_compatible_room(&self, base_amount: u32) -> CoinjoinResult<RoomEntity>;
    async fn create_room(
        &self,
        base_amount: u32,
        due1: u32,
        due2: u32,
    ) -> CoinjoinResult<RoomEntity>;
    async fn add_peer(
        &self,
        room_id: uuid::Uuid,
        txids: Vec<String>,
        vouts: Vec<u16>,
        amounts: Vec<u64>,
        change: u64,
        script: String,
    ) -> CoinjoinResult<()>;
    async fn get_room_by_id(&self, room_id: &str) -> CoinjoinResult<RoomEntity>;
    async fn get_inputs(&self, room_id: &str) -> CoinjoinResult<Vec<Input>>;
    async fn get_outputs(&self, room_id: &str) -> CoinjoinResult<Vec<Output>>;
    async fn add_output(&self, room_id: &str, address: &str, amount: u32) -> CoinjoinResult<()>;
    async fn get_proofs(&self, room_id: &str) -> CoinjoinResult<Vec<Proof>>;
    async fn add_script(&self, room_id: &str, vin: u16, script: &str) -> CoinjoinResult<()>;
}
