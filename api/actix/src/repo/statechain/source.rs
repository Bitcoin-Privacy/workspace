use anyhow::Result;
use async_trait::async_trait;

use crate::{db::Database, model::entity::statechain::StateCoin};
use bitcoin::secp256k1::{PublicKey, SecretKey};

use super::TraitStatechainRepo;

#[derive(Clone)]
pub struct StatechainRepo {
    pool: Database,
}

impl StatechainRepo {
    pub fn new(pool: Database) -> Self {
        Self { pool }
    }

    pub async fn get_statecoin(&self, id: &str) -> Result<StateCoin> {
        let statecoin =
            sqlx::query_as::<_, StateCoin>("select * from statechain_data where id = $1::uuid")
                .bind(id)
                .fetch_one(&self.pool.pool)
                .await?;
        Ok(statecoin)
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
        amount: u32,
    ) -> Result<StateCoin> {
        let server_privkey_bytes = server_privkey.display_secret().to_string();
        let server_pubkey_bytes = server_pubkey.serialize();
        let auth_pubkey_bytes = auth_pubkey.serialize();
        let statecoin = sqlx::query_as::<_, StateCoin>(
            r#"
            insert into statechain_data 
            (token_id, auth_xonly_public_key, server_public_key, server_private_key, amount) 
            values ($1, $2, $3, $4, $5)
            returning *
            "#,
        )
        .bind(token_id)
        .bind(auth_pubkey_bytes)
        .bind(server_pubkey_bytes)
        .bind(server_privkey_bytes)
        .bind(amount as i64)
        .fetch_one(&self.pool.pool)
        .await?;
        Ok(statecoin)
    }
}
