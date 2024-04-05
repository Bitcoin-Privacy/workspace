use anyhow::{anyhow, Result};
use bitcoin::secp256k1::{PublicKey, SecretKey};
use sqlx::{sqlite::SqliteQueryResult, Executor, SqlitePool};

use crate::model::RoomEntity;

mod sqlite;

pub struct PoolWrapper {
    pool: SqlitePool,
}

impl PoolWrapper {
    pub async fn new() -> Self {
        let sqlite_pool = sqlite::init_db()
            .await
            .expect("Failed to initialize SQLite database");
        sqlite_pool
            .execute(include_str!("../../db/init_database.sql"))
            .await
            .expect("Failed to run initial SQL script");

        PoolWrapper { pool: sqlite_pool }
    }

    pub async fn set_password(&self, password: &str) -> Result<()> {
        sqlite::set_cfg(&self.pool, "pw", password).await
    }

    pub async fn get_password(&self) -> Result<Option<String>> {
        sqlite::get_cfg(&self.pool, "pw").await
    }

    pub async fn set_seed(&self, seed: &str) -> Result<()> {
        sqlite::set_cfg(&self.pool, "seed", seed).await
    }

    pub async fn get_seed(&self) -> Result<Option<String>> {
        sqlite::get_cfg(&self.pool, "seed").await
    }

    pub fn add_or_update_room(&self, deriv: &str, room: &RoomEntity) -> Result<()> {
        // let rooms_tree = self.sled.open_tree("rooms-".to_owned() + derivation_path)?;
        // let room_key = room.id.as_bytes();
        // let room_value = bincode::serialize(&room)?;
        // rooms_tree.insert(room_key, room_value)?;
        Ok(())
    }

    pub fn get_all_rooms(&self, deriv: &str) -> Result<Vec<RoomEntity>> {
        // let rooms_tree = self.sled.open_tree("rooms-".to_owned() + deriv)?;
        // let mut rooms = Vec::new();
        //
        // for result in rooms_tree.iter() {
        //     let (_, value) = result?;
        //     let room: RoomEntity = bincode::deserialize(&value)?;
        //     rooms.push(room);
        // }
        //
        // Ok(rooms)
        Ok(vec![])
    }
    pub fn get_room(&self, deriv: &str, room_id: &str) -> Result<RoomEntity> {
        // let rooms_tree = self.sled.open_tree("rooms-".to_owned() + derivation_path)?;
        // if let Ok(Some(room)) = rooms_tree.get(room_id) {
        //     let room: RoomEntity = bincode::deserialize(&room)?;
        //     Ok(room)
        // } else {
        Err(anyhow!("Cannot find room"))
        // }
    }
    pub async fn insert_statecoin(
        &self,
        statechain_id: &str,
        deriv: &str,
        amount: u64,
        auth_seckey: &SecretKey,
        auth_pubkey: &PublicKey,
        aggregated_pubkey: &str,
        aggregated_address: &str,
        owner_seckey: &SecretKey,
        owner_pubkey: &PublicKey,
    ) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlite::insert_statecoin(
            &self.pool,
            &statechain_id,
            &deriv,
            amount,
            &auth_seckey,
            &auth_pubkey,
            &aggregated_pubkey,
            &aggregated_address,
            &owner_seckey,
            &owner_pubkey,
        )
        .await
    }
}
