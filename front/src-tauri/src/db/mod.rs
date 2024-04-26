use anyhow::{anyhow, Result};
use bitcoin::{
    hex::DisplayHex,
    secp256k1::{PublicKey, SecretKey},
    XOnlyPublicKey,
};
use musig2::{BinaryEncoding, KeyAggContext};
use sqlx::{
    sqlite::{SqliteLockingMode, SqliteQueryResult},
    Executor, SqlitePool,
};

use crate::model::{RoomEntity, StateCoin, StateCoinInfo};

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
    pub async fn get_statecoin_by_id(&self, statechain_id: &str) -> Result<StateCoin> {
        sqlite::get_statecoin_by_id(&self.pool, &statechain_id).await
    }

    pub async fn get_seckey_by_id(&self, statechain_id: &str) -> Result<Option<String>> {
        sqlite::get_seckey_by_id(&self.pool, &statechain_id).await
    }

    pub async fn list_statecoins_by_account(&self, account: &str) -> Result<Vec<StateCoinInfo>> {
        sqlite::get_statecoins_by_account(&self.pool, &account).await
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
    pub async fn create_statecoin(
        &self,
        statechain_id: &str,
        signed_statechain_id: &str,
        deriv: &str,
        amount: u64,
        auth_seckey: &SecretKey,
        auth_pubkey: &XOnlyPublicKey,
        aggregated_pubkey: &str,
        aggregated_address: &str,
        owner_seckey: &SecretKey,
        owner_pubkey: &PublicKey,
        key_agg_ctx: &KeyAggContext,
    ) -> Result<SqliteQueryResult, sqlx::Error> {
        let amount_i64: i64 = amount as i64;
        let owner_seckey_bytes = owner_seckey.secret_bytes().to_lower_hex_string();
        let owner_pubkey_bytes = owner_pubkey.to_string();
        let auth_seckey_bytes = auth_seckey.secret_bytes().to_lower_hex_string();
        let auth_pubkey_bytes = auth_pubkey.to_string();

        let serialized_key_agg_ctx = key_agg_ctx
            .to_bytes()
            .to_hex_string(bitcoin::hex::Case::Lower);

        sqlite::create_statecoin(
            &self.pool,
            &statechain_id,
            &signed_statechain_id,
            &deriv,
            amount_i64,
            &auth_seckey_bytes,
            &auth_pubkey_bytes,
            &aggregated_pubkey,
            &aggregated_address,
            &owner_seckey_bytes,
            &owner_pubkey_bytes,
            &serialized_key_agg_ctx,
        )
        .await
    }

    pub async fn update_deposit_tx(
        &self,
        statechain_id: &str,
        funding_txid: &str,
        funding_vout: u64,
        funding_tx: &str,
    ) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlite::update_deposit_tx(
            &self.pool,
            statechain_id,
            funding_txid,
            funding_vout,
            funding_tx,
        )
        .await
    }

    pub async fn update_bk_tx(
        &self,
        statechain_id: &str,
        backup_tx: &str,
        agg_pubnonce: &str,
    ) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlite::update_bk_tx(&self.pool, statechain_id, backup_tx, agg_pubnonce).await
    }
}
