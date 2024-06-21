use anyhow::Result;
use async_trait::async_trait;
use sqlx::{postgres::PgPoolOptions, Executor, PgPool};

use crate::CFG;

#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

#[async_trait]
pub trait TraitDatabase: Send + Sync + 'static {
    async fn init_database(&mut self) -> Result<()>;
}

#[async_trait]
impl TraitDatabase for Database {
    async fn init_database(&mut self) -> Result<()> {
        let _ = self
            .pool
            .execute(include_str!("../../db/init_database.sql"))
            .await?;
        Ok(())
    }
}

impl Database {
    pub async fn new() -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&CFG.postgres_uri)
            .await
            .unwrap();
        Database { pool }
    }
}
