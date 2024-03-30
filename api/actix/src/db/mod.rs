use async_trait::async_trait;
use sqlx::{postgres::PgPoolOptions, Executor, PgPool};

use crate::config::CONFIG;

#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

pub type DatabaseError = String;
pub type DatabaseResult<T> = Result<T, DatabaseError>;

#[async_trait]
pub trait TraitDatabase: Send + Sync + 'static {
    async fn init_database(&mut self) -> Result<(), String>;
}

#[async_trait]
impl TraitDatabase for Database {
    async fn init_database(&mut self) -> Result<(), String> {
        let result = self
            .pool
            .execute(include_str!("../../db/init_database.sql"))
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl Database {
    pub async fn new() -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&CONFIG.postgres_uri)
            .await
            .unwrap();
        Database { pool }
    }
}
