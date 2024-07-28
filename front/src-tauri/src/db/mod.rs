use anyhow::{anyhow, Result};
use bitcoin::{
    absolute::LockTime,
    consensus,
    hex::DisplayHex,
    secp256k1::{PublicKey, SecretKey},
    Transaction, XOnlyPublicKey,
};
use musig2::{BinaryEncoding, KeyAggContext};
use serde::de::Error;
use sqlx::{
    sqlite::{SqliteLockingMode, SqliteQueryResult},
    Executor, SqlitePool,
};

use crate::model::{RoomEntity, Statecoin, StatecoinCard, StatecoinDetail, StatecoinEntity};

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
    pub async fn get_statecoin_by_id(&self, statechain_id: &str) -> Result<StatecoinEntity> {
        sqlite::get_statecoin_by_id(&self.pool, &statechain_id).await
    }

    // pub async fn get_seckey_by_id(&self, statechain_id: &str) -> Result<Option<String>> {
    //     sqlite::get_seckey_by_id(&self.pool, &statechain_id).await
    // }

    pub async fn list_statecoins_by_account(&self, account: &str) -> Result<Vec<StatecoinCard>> {
        sqlite::get_statecoins_by_account(&self.pool, &account).await
    }

    pub async fn list_authkeys_by_account(&self, account: &str) -> Result<Vec<String>> {
        sqlite::get_authkeys_by_account(&self.pool, &account).await
    }
    pub async fn get_seckey_by_authkey(&self, authkey: &str) -> Result<Option<(String, String)>> {
        sqlite::get_seckey_by_authkey(&self.pool, &authkey).await
    }

    pub async fn delete_statecoin_by_statechain_id(&self, statechain_id: &str) -> Result<()> {
        println!("delete id : {}", statechain_id);
        sqlite::delete_statecoin_by_statechain_id(&self.pool, statechain_id).await
    }

    pub async fn create_unverified_statecoin(
        &self,
        account: &str,
        auth_pubkey: &str,
        auth_seckey: &str,
        owner_pubkey: &str,
        owner_seckey: &str,
    ) -> Result<()> {
        sqlite::create_unverified_statecoin(
            &self.pool,
            account,
            auth_pubkey,
            auth_seckey,
            owner_pubkey,
            owner_seckey,
        )
        .await
    }

    pub async fn update_unverifed_statecoin(
        &self,
        statechain_id: &str,
        statecoin: &Statecoin,
        bk_tx: &str,
        authkey: &str,
        aggregated_address: &str,
    ) -> Result<()> {
        let parsed_tx = consensus::deserialize::<Transaction>(&hex::decode(bk_tx.clone())?)?;
        let locktime = match parsed_tx.lock_time {
            LockTime::Seconds(s) => s.to_consensus_u32(),
            LockTime::Blocks(_b) => return Err(anyhow!("Internal error: invalid locktime!")),
        };
        sqlite::update_unverifed_statecoin(
            &self.pool,
            statechain_id,
            &statecoin.signed_statechain_id,
            statecoin.tx_n + 1,
            locktime as i64,
            &statecoin.key_agg_ctx,
            &statecoin.aggregated_pubkey,
            aggregated_address,
            &statecoin.funding_txid,
            statecoin.funding_vout,
            statecoin.amount,
            bk_tx,
            authkey,
            &statecoin.spend_key,
        )
        .await
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
    pub async fn create_statecoin(
        &self,
        statechain_id: &str,
        signed_statechain_id: &str,
        account: &str,
        amount: u64,
        auth_seckey: &SecretKey,
        auth_pubkey: &XOnlyPublicKey,
        aggregated_pubkey: &str,
        aggregated_address: &str,
        owner_seckey: &SecretKey,
        owner_pubkey: &PublicKey,
        key_agg_ctx: &KeyAggContext,
        funding_txid: &str,
        funding_vout: u64,
        funding_tx: &str,
        txn: u64,
        n_lock_time: u32,
        backup_tx: &str,
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
            statechain_id,
            signed_statechain_id,
            account,
            amount_i64,
            &auth_seckey_bytes,
            &auth_pubkey_bytes,
            aggregated_pubkey,
            aggregated_address,
            &owner_seckey_bytes,
            &owner_pubkey_bytes,
            &serialized_key_agg_ctx,
            funding_txid,
            funding_vout as i64,
            funding_tx,
            txn as i64,
            n_lock_time as i64,
            backup_tx,
        )
        .await
    }

    // pub async fn update_deposit_tx(
    //     &self,
    //     statechain_id: &str,
    //     funding_txid: &str,
    //     funding_vout: u64,
    //     funding_tx: &str,
    // ) -> Result<SqliteQueryResult, sqlx::Error> {
    //     sqlite::update_deposit_tx(
    //         &self.pool,
    //         statechain_id,
    //         funding_txid,
    //         funding_vout,
    //         funding_tx,
    //     )
    //     .await
    // }

    pub async fn get_statecoin_detail_by_id(&self, statechain_id: &str) -> Result<StatecoinDetail> {
        sqlite::get_statecoin_detail_by_id(&self.pool, statechain_id).await
    }
}
//     pub async fn create_bk_tx(
//         &self,
//         statechain_id: &str,
//         backup_tx: &str,
//         tx_n: u64,
//         n_lock_time: u64,
//     ) -> Result<SqliteQueryResult, sqlx::Error> {
//         sqlite::create_bk_tx(&self.pool, statechain_id, backup_tx, tx_n, n_lock_time).await
//     }

//     pub async fn get_bk_tx_by_statechain_id(
//         &self,
//         statechain_id: &str,
//     ) -> Result<Vec<String>, sqlx::Error> {
//         sqlite::get_bk_tx_by_statechain_id(&self.pool, statechain_id).await
//     }
// }
