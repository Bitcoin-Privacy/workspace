use anyhow::Result;
use async_trait::async_trait;

use crate::{db::Database, model::entity::statechain::StateCoin};
use bitcoin::secp256k1::{PublicKey, XOnlyPublicKey, SecretKey};

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
    }}

//     async fn get_auth_key_by_statechain_id(&self, statechain_id: &str) -> StatechainResult<String> {
//         let row = sqlx::query(
//             r#"select auth_xonly_public_key from statechain_data where statechain_id = $1"#,
//         )
//         .bind(statechain_id)
//         .fetch_one(&self.pool.pool)
//         .await
//         .map_err(|e| e.to_string());
//         println!(
//             "auth xonly {:?}",
//             row.unwrap().column("auth_xonly_public_key")
//         );

//         Ok("asdf".to_string())
//     }

//     async fn insert_signature_data(
//         &self,
//         r2_commitment: &str,
//         blind_commitment: &str,
//         statechain_id: &str,
//         server_pubnonce: &PublicKey,
//         server_secnonce: &SecretKey,
//     ) -> StatechainResult<()> {
//         let mut transaction = self.pool.pool.begin().await.unwrap();

//         let max_tx_k_query = "\
//             SELECT COALESCE(MAX(tx_n), 0) \
//             FROM (\
//                 SELECT * \
//                 FROM statechain_signature_data \
//                 WHERE statechain_id = $1 FOR UPDATE) AS result";

//         let row = sqlx::query(max_tx_k_query)
//             .bind(statechain_id)
//             .fetch_one(&mut *transaction)
//             .await
//             .unwrap();

//         let mut new_tx_n = row.get::<i32, _>(0);
//         new_tx_n = new_tx_n + 1;

//         let query = "\
//             INSERT INTO statechain_signature_data \
//             (r2_commitment, blind_commitment,  server_pubnonce, server_secnonce, tx_n, statechain_id ) \
//             VALUES ($1, $2, $3, $4, $5, $6)";

//         let server_secnonce_bytes = server_secnonce.secret_bytes();
//         let server_pubnonce_bytes = server_pubnonce.serialize();

//         let _ = sqlx::query(query)
//             .bind(r2_commitment)
//             .bind(blind_commitment)
//             .bind(server_pubnonce_bytes)
//             .bind(server_secnonce_bytes)
//             .bind(new_tx_n)
//             .bind(statechain_id)
//             .execute(&mut *transaction)
//             .await
//             .unwrap();

//         let res = transaction.commit().await;

//         match res {
//             Ok(_) => Ok(()),
//             Err(e) => Err(e.to_string()),
//         }
//     }

//     async fn get_auth_key_by_statechain_id(&self, statechain_id: &str) -> StatechainResult<String> {
//         let row = sqlx::query(
//             r#"select auth_xonly_public_key from statechain_data where statechain_id = $1"#,
//         )
//         .bind(statechain_id)
//         .fetch_one(&self.pool.pool)
//         .await
//         .map_err(|e| e.to_string());
//         println!(
//             "auth xonly {:?}",
//             row.unwrap().column("auth_xonly_public_key")
//         );

//         Ok("asdf".to_string())
//     }

//     async fn insert_signature_data(
//         &self,
//         r2_commitment: &str,
//         blind_commitment: &str,
//         statechain_id: &str,
//         server_pubnonce: &PublicKey,
//         server_secnonce: &SecretKey,
//     ) -> StatechainResult<()> {
//         let mut transaction = self.pool.pool.begin().await.unwrap();

//         let max_tx_k_query = "\
//             SELECT COALESCE(MAX(tx_n), 0) \
//             FROM (\
//                 SELECT * \
//                 FROM statechain_signature_data \
//                 WHERE statechain_id = $1 FOR UPDATE) AS result";

//         let row = sqlx::query(max_tx_k_query)
//             .bind(statechain_id)
//             .fetch_one(&mut *transaction)
//             .await
//             .unwrap();

//         let mut new_tx_n = row.get::<i32, _>(0);
//         new_tx_n = new_tx_n + 1;

//         let query = "\
//             INSERT INTO statechain_signature_data \
//             (r2_commitment, blind_commitment,  server_pubnonce, server_secnonce, tx_n, statechain_id ) \
//             VALUES ($1, $2, $3, $4, $5, $6)";

//         let server_secnonce_bytes = server_secnonce.secret_bytes();
//         let server_pubnonce_bytes = server_pubnonce.serialize();

//         let _ = sqlx::query(query)
//             .bind(r2_commitment)
//             .bind(blind_commitment)
//             .bind(server_pubnonce_bytes)
//             .bind(server_secnonce_bytes)
//             .bind(new_tx_n)
//             .bind(statechain_id)
//             .execute(&mut *transaction)
//             .await
//             .unwrap();

//         let res = transaction.commit().await;

//         match res {
//             Ok(_) => Ok(()),
//             Err(e) => Err(e.to_string()),
//         }
//     }
// }

// #[derive(FromRow)]
// struct AuthXonly {
//     auth_xonly_public_key: String,
// }

// #[derive(FromRow)]
// struct AuthXonly {
//     auth_xonly_public_key: String,
// }
