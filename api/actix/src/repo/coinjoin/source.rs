use async_trait::async_trait;
use sqlx::Executor;

use crate::{
    config::CONFIG,
    db::Database,
    model::entity::coinjoin::{Input, Output, Proof, Room},
};
use uuid::Uuid;

use super::{CoinjoinResult, TraitCoinJoinRepo};

#[derive(Clone)]
pub struct CoinJoinRepo {
    pool: Database,
}

impl CoinJoinRepo {
    pub fn new(pool: Database) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TraitCoinJoinRepo for CoinJoinRepo {
    async fn get_rooms(&self) -> CoinjoinResult<Vec<Room>> {
        sqlx::query_as::<_, Room>(r#"select * from room"#)
            .fetch_all(&self.pool.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn get_compatible_room(&self, base_amount: u32) -> CoinjoinResult<Room> {
        let rooms = sqlx::query_as::<_, Room>(r#"select * from room where base_amount = $1"#)
            .bind(base_amount as i64)
            .fetch_all(&self.pool.pool)
            .await
            .map_err(|e| e.to_string())?;

        if rooms.len() == 0 {
            self.create_room(base_amount, CONFIG.due_time_1, CONFIG.due_time_2)
                .await
        } else {
            Ok(rooms.first().unwrap().clone())
        }
    }

    async fn create_room(&self, base_amount: u32, due1: u32, due2: u32) -> CoinjoinResult<Room> {
        sqlx::query_as::<_, Room>(
            r#"insert into room (base_amount, due1, due2) values ($1, $2, $3) returning *"#,
        )
        .bind(base_amount as i64)
        .bind(due1 as i64)
        .bind(due2 as i64)
        .fetch_one(&self.pool.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn add_peer(
        &self,
        room_id: uuid::Uuid,
        txid: String,
        vout: u16,
        amount: u64,
        change: u64,
        script: String,
    ) -> CoinjoinResult<()> {
        let query = sqlx::query_as::<_, Room>(r#"select add_new_peer($1, $2, $3, $4, $5, $6);"#)
            .bind(room_id)
            .bind(txid)
            .bind(vout as i32)
            .bind(amount as i64)
            .bind(change as i64)
            .bind(script);

        let result = self.pool.pool.execute(query).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn get_room_by_id(&self, room_id: &str) -> CoinjoinResult<Room> {
        sqlx::query_as::<_, Room>(r#"select * from room where id = $1::uuid"#)
            .bind(room_id)
            .fetch_one(&self.pool.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn get_inputs(&self, room_id: &str) -> CoinjoinResult<Vec<Input>> {
        sqlx::query_as::<_, Input>(r#"select * from txin where room_id = $1::uuid order by txid"#)
            .bind(room_id)
            .fetch_all(&self.pool.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn get_outputs(&self, room_id: &str) -> CoinjoinResult<Vec<Output>> {
        sqlx::query_as::<_, Output>(r#"select * from txout where room_id = $1::uuid order by id"#)
            .bind(room_id)
            .fetch_all(&self.pool.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn add_output(&self, room_id: &str, address: &str, amount: u32) -> CoinjoinResult<()> {
        let parsed_room_id = match Uuid::parse_str(room_id) {
            Ok(uuid) => uuid,
            Err(_) => return Err("Invalid UUID format for room_id".to_string()),
        };
        let query = sqlx::query_as::<_, Output>(
            r#"insert into txout (room_id, address, amount) values ($1::uuid, $2, $3)"#,
        )
        .bind(parsed_room_id)
        .bind(address)
        .bind(amount as i64);
        let result = self.pool.pool.execute(query).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn get_proofs(&self, room_id: &str) -> CoinjoinResult<Vec<Proof>> {
        sqlx::query_as::<_, Proof>(r#"select * from proof where room_id = $1::uuid order by vin"#)
            .bind(room_id)
            .fetch_all(&self.pool.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn add_script(&self, room_id: &str, vin: u16, script: &str) -> CoinjoinResult<()> {
        let query = sqlx::query_as::<_, Output>(
            r#"insert into proof (room_id, vin, script) values ($1::uuid, $2, $3)"#,
        )
        .bind(room_id)
        .bind(vin as i32)
        .bind(script);
        let result = self.pool.pool.execute(query).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}
