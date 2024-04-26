use anyhow::Result;
use async_trait::async_trait;
use musig2::{BinaryEncoding, PubNonce, SecNonce};

use crate::{
    db::Database,
    model::entity::statechain::{AuthPubkey, StateCoin},
};
use bitcoin::{
    hex::DisplayHex,
    secp256k1::{PublicKey, SecretKey, XOnlyPublicKey},
};

use super::TraitStatechainRepo;

#[derive(Clone)]
pub struct StatechainRepo {
    pool: Database,
}

impl StatechainRepo {
    pub fn new(pool: Database) -> Self {
        Self { pool }
    }

    pub async fn get_by_id(&self, id: &str) -> Result<StateCoin> {
        let statecoin =
            sqlx::query_as::<_, StateCoin>("select * from statechain where id = $1::uuid")
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
        auth_pubkey: &XOnlyPublicKey,
        server_pubkey: &PublicKey,
        server_privkey: &SecretKey,
        amount: u32,
    ) -> Result<StateCoin> {
        let server_privkey_bytes = server_privkey.display_secret().to_string();
        let server_pubkey_bytes = server_pubkey.to_string();
        let auth_pubkey_bytes = auth_pubkey.to_string();
        let statecoin = sqlx::query_as::<_, StateCoin>(
            r#"
            insert into statechain 
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
    async fn update_nonce(&self, secnonce: &str, statechain_id: &str) -> Result<()> {
        let row = sqlx::query(" update statechain set sec_nonce = $1 where id = $2::uuid ")
            .bind(secnonce)
            .bind(statechain_id)
            .execute(&self.pool.pool)
            .await?;

        Ok(())
    }
    async fn get_auth_key_by_statechain_id(&self, statechain_id: &str) -> Result<AuthPubkey> {
        let row = sqlx::query_as::<_, AuthPubkey>(
            r#"select auth_xonly_public_key from statechain where id = $1::uuid"#,
        )
        .bind(statechain_id)
        .fetch_one(&self.pool.pool)
        .await?;

        Ok(row)
    }

    async fn update_auth_pubkey(
        &self,
        statechain_id: &str,
        authkey: &str,
        random_key: &str,
    ) -> Result<()> {
        let query = sqlx::query(
            "insert into statechain_transfer (authkey,random_key, statechain_id) values ($1,$2,$3)",
        )
        .bind(authkey)
        .bind(random_key)
        .bind(statechain_id)
        .execute(&self.pool.pool)
        .await?;

        Ok(())
    }

    async fn update_transfer_message(&self, authkey: &str, transfer_msg: &str) -> Result<()> {
        let query =
            sqlx::query("update statechain_transfer set tranfer_msg= $1 where authkey = $2::uuid")
                .bind(transfer_msg)
                .bind(authkey)
                .execute(&self.pool.pool)
                .await?;

        Ok(())
    }
}
