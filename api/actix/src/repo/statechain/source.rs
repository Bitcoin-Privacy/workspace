use async_trait::async_trait;
use sqlx::Executor;

use crate::db::Database;
use secp256k1::{PublicKey, SecretKey};

use super::{StatechainResult, TraitStatechainRepo};

#[derive(Clone)]
pub struct StatechainRepo {
    pool: Database,
}

impl StatechainRepo {
    pub fn new(pool: Database) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TraitStatechainRepo for StatechainRepo {
    async fn create_deposit_tx(
        &self,
        token_id: &str,
        auth_pubkey: &PublicKey,
        server_pubkey: &PublicKey,
        server_privkey: &SecretKey,
        statechain_id: &str,
        amount: u32,
    ) -> StatechainResult<()> {
        let server_privkey_bytes = server_privkey.secret_bytes();
        let server_pubkey_bytes = server_pubkey.serialize();
        let auth_pubkey_bytes = auth_pubkey.serialize();
        let query = sqlx::query(r#"INSERT INTO statechain_data (token_id, auth_xonly_public_key, server_public_key, server_private_key, statechain_id,amount) VALUES ($1, $2, $3, $4, $5, $6)"#)
            .bind(token_id)
            .bind(auth_pubkey_bytes)
            .bind(server_pubkey_bytes)
            .bind(server_privkey_bytes)
            .bind(statechain_id)
            .bind(amount as i64);
        let res = self.pool.pool.execute(query).await;
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}
