use crate::model::entity::coinjoin::{Input, Output, Proof, Room};
use async_trait::async_trait;

mod source;
pub use source::CoinJoinRepo;

pub type CoinjoinError = String;
pub type CoinjoinResult<T> = Result<T, CoinjoinError>;

#[async_trait]
pub trait TraitCoinJoinRepo: Send + Sync + 'static {
    async fn get_rooms(&self) -> CoinjoinResult<Vec<Room>>;
    async fn get_compatible_room(&self, base_amount: u32) -> CoinjoinResult<Room>;
    async fn create_room(&self, base_amount: u32, due1: u32, due2: u32) -> CoinjoinResult<Room>;
    async fn add_peer(
        &self,
        room_id: uuid::Uuid,
        txid: String,
        vout: u16,
        amount: u64,
        change: u64,
        script: String,
    ) -> CoinjoinResult<()>;
    async fn get_room_by_id(&self, room_id: &str) -> CoinjoinResult<Room>;
    async fn get_inputs(&self, room_id: &str) -> CoinjoinResult<Vec<Input>>;
    async fn get_outputs(&self, room_id: &str) -> CoinjoinResult<Vec<Output>>;
    async fn add_output(&self, room_id: &str, address: &str, amount: u32) -> CoinjoinResult<()>;
    async fn get_proofs(&self, room_id: &str) -> CoinjoinResult<Vec<Proof>>;
    async fn add_script(&self, room_id: &str, vin: u16, script: &str) -> CoinjoinResult<()>;
}
