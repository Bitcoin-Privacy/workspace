use anyhow::{anyhow, Result};
use sled::Db;
use sqlx::{Executor, SqlitePool};

use crate::{cfg::DATABASE_PATH, model::RoomEntity};

mod sqlite;

pub struct PoolWrapper {
    pub sled: Db,
    pub sqlite: SqlitePool,
}

impl PoolWrapper {
    pub async fn new() -> Self {
        let sled_pool = sled::open(DATABASE_PATH).unwrap();
        let sqlite_pool = sqlite::init_db()
            .await
            .expect("Failed to initialize SQLite database");
        sqlite_pool
            .execute(include_str!("../../db/init_database.sql"))
            .await
            .expect("Failed to run initial SQL script");

        PoolWrapper {
            sled: sled_pool,
            sqlite: sqlite_pool,
        }
    }

    pub async fn set_password(&self, password: &str) -> Result<()> {
        let _ = sqlx::query(
            r#"INSERT INTO config (KEY, VALUE)
            VALUES('pw', ?) ON CONFLICT (KEY)
            DO UPDATE SET VALUE = excluded.VALUE;"#,
        )
        .bind(password)
        .execute(&self.sqlite)
        .await?;
        Ok(())
    }

    pub fn add_or_update_room(&self, derivation_path: &str, room: &RoomEntity) -> Result<()> {
        let rooms_tree = self.sled.open_tree("rooms-".to_owned() + derivation_path)?;
        let room_key = room.id.as_bytes();
        let room_value = bincode::serialize(&room)?;
        rooms_tree.insert(room_key, room_value)?;
        Ok(())
    }

    pub fn get_all_rooms(&self, deriv: &str) -> Result<Vec<RoomEntity>> {
        let rooms_tree = self.sled.open_tree("rooms-".to_owned() + deriv)?;
        let mut rooms = Vec::new();

        for result in rooms_tree.iter() {
            let (_, value) = result?;
            let room: RoomEntity = bincode::deserialize(&value)?;
            rooms.push(room);
        }

        Ok(rooms)
    }
    pub fn get_room(&self, derivation_path: &str, room_id: &str) -> Result<RoomEntity> {
        let rooms_tree = self.sled.open_tree("rooms-".to_owned() + derivation_path)?;
        if let Ok(Some(room)) = rooms_tree.get(room_id) {
            let room: RoomEntity = bincode::deserialize(&room)?;
            Ok(room)
        } else {
            Err(anyhow!("Cannot find room"))
        }
    }
}
