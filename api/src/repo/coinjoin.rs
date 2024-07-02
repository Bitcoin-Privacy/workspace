use anyhow::{anyhow, Result};
use sqlx::Executor;

use crate::{
    db::Database,
    model::entity::coinjoin::{Input, Output, Proof, RoomEntity, SpentSig},
    CFG,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct CoinjoinRepo {
    pool: Database,
}

impl CoinjoinRepo {
    pub fn new(pool: Database) -> Self {
        Self { pool }
    }

    pub async fn get_rooms_by_addr(&self, addr: &str) -> Result<Vec<RoomEntity>> {
        let res = sqlx::query_as::<_, RoomEntity>(
            r#"select r.*
            from room r
            inner join (
                select distinct i.room_id as id
                from txin i
                where i.address = $1
            ) as distinct_rooms on r.id = distinct_rooms.id"#,
        )
        .bind(addr)
        .fetch_all(&self.pool.pool)
        .await?;
        Ok(res)
    }

    pub async fn get_compatible_room(&self, base_amount: u32) -> Result<RoomEntity> {
        let rooms = sqlx::query_as::<_, RoomEntity>(
            r#"
            select * 
            from room
            where base_amount = 10
              and status = 0
              and created_at + interval '1 second' * (room.due1 / 1000) > now();
            "#,
        )
        .bind(base_amount as i64)
        .fetch_all(&self.pool.pool)
        .await?;

        if rooms.is_empty() {
            let res = self
                .create_room(base_amount, CFG.due_time_1, CFG.due_time_2)
                .await?;
            Ok(res)
        } else {
            Ok(rooms.first().unwrap().clone())
        }
    }

    pub async fn create_room(&self, base_amount: u32, due1: u32, due2: u32) -> Result<RoomEntity> {
        let res = sqlx::query_as::<_, RoomEntity>(
            r#"insert into room (base_amount, due1, due2) values ($1, $2, $3) returning *"#,
        )
        .bind(base_amount as i64)
        .bind(due1 as i64)
        .bind(due2 as i64)
        .fetch_one(&self.pool.pool)
        .await?;
        Ok(res)
    }

    pub async fn add_peer(
        &self,
        room_id: uuid::Uuid,
        txids: Vec<String>,
        vouts: Vec<u16>,
        amounts: Vec<u64>,
        change: u64,
        address: String,
    ) -> Result<()> {
        // Convert the vectors to arrays of the proper type for PostgreSQL
        let txids_array: Vec<&str> = txids.iter().map(String::as_str).collect();
        let vouts_array: Vec<i32> = vouts.into_iter().map(|vout| vout as i32).collect();
        let amounts_array: Vec<i64> = amounts.into_iter().map(|amount| amount as i64).collect();

        let query = sqlx::query(
            r#"select add_new_peer($1, $2::varchar[], $3::int[], $4::bigint[], $5, $6);"#,
        )
        .bind(room_id)
        .bind(&txids_array)
        .bind(&vouts_array)
        .bind(&amounts_array)
        .bind(change as i64)
        .bind(address);

        let _ = self.pool.pool.execute(query).await?;
        Ok(())
    }

    pub async fn get_room_by_id(&self, room_id: &str) -> Result<RoomEntity> {
        let res = sqlx::query_as::<_, RoomEntity>(r#"select * from room where id = $1::uuid"#)
            .bind(room_id)
            .fetch_one(&self.pool.pool)
            .await?;
        Ok(res)
    }

    pub async fn get_inputs(&self, room_id: &str) -> Result<Vec<Input>> {
        let res = sqlx::query_as::<_, Input>(
            r#"select * from txin where room_id = $1::uuid order by txid"#,
        )
        .bind(room_id)
        .fetch_all(&self.pool.pool)
        .await?;
        Ok(res)
    }

    pub async fn get_inputs_by_addr(&self, room_id: &str, address: &str) -> Result<Vec<Input>> {
        let res = sqlx::query_as::<_, Input>(
            r#"select * from txin where room_id = $1::uuid and address = $2 order by txid"#,
        )
        .bind(room_id)
        .bind(address)
        .fetch_all(&self.pool.pool)
        .await?;
        Ok(res)
    }

    pub async fn get_outputs(&self, room_id: &str) -> Result<Vec<Output>> {
        let res = sqlx::query_as::<_, Output>(
            r#"select * from txout where room_id = $1::uuid order by id"#,
        )
        .bind(room_id)
        .fetch_all(&self.pool.pool)
        .await?;
        Ok(res)
    }

    pub async fn add_output(&self, room_id: &str, address: &str, amount: u32) -> Result<()> {
        let parsed_room_id = match Uuid::parse_str(room_id) {
            Ok(uuid) => uuid,
            Err(_) => return Err(anyhow!("Invalid UUID format for room_id")),
        };
        let query = sqlx::query_as::<_, Output>(
            r#"insert into txout (room_id, address, amount) values ($1::uuid, $2, $3)"#,
        )
        .bind(parsed_room_id)
        .bind(address)
        .bind(amount as i64);
        let _ = self.pool.pool.execute(query).await?;
        Ok(())
    }

    pub async fn set_room_status(&self, room_id: &str, status: u8) -> Result<()> {
        let query =
            sqlx::query_as::<_, RoomEntity>(r#"update room set status=$2 where id = $1::uuid"#)
                .bind(room_id)
                .bind(status as i8);
        let _ = self.pool.pool.execute(query).await?;
        Ok(())
    }

    pub async fn get_proofs(&self, room_id: &str) -> Result<Vec<Proof>> {
        let res = sqlx::query_as::<_, Proof>(
            r#"select * from proof where room_id = $1::uuid order by vin"#,
        )
        .bind(room_id)
        .fetch_all(&self.pool.pool)
        .await?;
        Ok(res)
    }

    pub async fn add_script(&self, room_id: &str, vin: u16, script: &str) -> Result<()> {
        let query = sqlx::query_as::<_, Output>(
            r#"insert into proof (room_id, vin, script) values ($1::uuid, $2, $3)"#,
        )
        .bind(room_id)
        .bind(vin as i32)
        .bind(script);
        let _ = self.pool.pool.execute(query).await;
        Ok(())
    }

    pub async fn set_spent_sig(&self, sig: &str) -> Result<()> {
        let query =
            sqlx::query_as::<_, SpentSig>(r#"insert into spent_sig (signature) values ($1)"#)
                .bind(sig);
        let _ = self.pool.pool.execute(query).await?;
        Ok(())
    }

    pub async fn get_spent_sig(&self, sig: &str) -> Result<bool> {
        let res = sqlx::query_as::<_, SpentSig>(r#"select * from spent_sig where signature = $1"#)
            .bind(sig)
            .fetch_all(&self.pool.pool)
            .await?;
        Ok(res.is_empty())
    }
}
