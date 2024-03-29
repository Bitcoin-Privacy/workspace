use anyhow::{anyhow, Result};
use sqlx::{Executor, Row, SqlitePool};

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
        let _ = sqlx::query(
            r#"insert into Config (key, value)
            values('pw', ?) on conflict (key)
            do update set value = excluded.value;"#,
        )
        .bind(password)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_password(&self) -> Result<Option<String>> {
        let row = sqlx::query(r#"select value from Config where key = "pw";"#)
            .fetch_optional(&self.pool)
            .await?;
        let val = match row {
            Some(r) => Some(r.try_get::<String, _>("value")?),
            None => None,
        };
        Ok(val)
    }

    pub async fn get_seed(&self) -> Result<Option<String>> {
        let row = sqlx::query(r#"select value from Config where key = "seed";"#)
            .fetch_optional(&self.pool)
            .await?;
        let val = match row {
            Some(r) => Some(r.try_get::<String, _>("value")?),
            None => None,
        };
        Ok(val)
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
}
