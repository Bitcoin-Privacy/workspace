use crate::model::entity::coinjoin::{Input, Output, Proof, Room};
use anyhow::Result;
use async_trait::async_trait;

mod source;
pub use source::CoinJoinRepo;

#[async_trait]
pub trait TraitCoinJoinRepo: Send + Sync + 'static {
    async fn get_rooms(&self) -> Result<Vec<Room>>;
    async fn get_compatible_room(&self, base_amount: u32) -> Result<Room>;
    async fn create_room(&self, base_amount: u32, due1: u32, due2: u32) -> Result<Room>;
    async fn add_peer(
        &self,
        room_id: uuid::Uuid,
        txids: Vec<String>,
        vouts: Vec<u16>,
        amounts: Vec<u64>,
        change: u64,
        script: String,
    ) -> Result<()>;
    async fn get_room_by_id(&self, room_id: &str) -> Result<Room>;
    async fn get_inputs(&self, room_id: &str) -> Result<Vec<Input>>;
    async fn get_outputs(&self, room_id: &str) -> Result<Vec<Output>>;
    async fn add_output(&self, room_id: &str, address: &str, amount: u32) -> Result<()>;
    async fn get_proofs(&self, room_id: &str) -> Result<Vec<Proof>>;
    async fn add_script(&self, room_id: &str, vin: u16, script: &str) -> Result<()>;
}
