use anyhow::Result;
use async_trait::async_trait;

use super::TraitStatechainRepo;
use crate::{
    db::Database,
    model::entity::statechain::{
        AuthPubkey, StateCoin, StatecoinSecret, StatecoinVerificationInfo,
    },
};
use bitcoin::secp256k1::{PublicKey, SecretKey, XOnlyPublicKey};
use sqlx::Row;

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
            (token_id, authkey, server_public_key, server_private_key, amount) 
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
        let row = sqlx::query("update statechain set sec_nonce = $1 where id = $2::uuid ")
            .bind(secnonce)
            .bind(statechain_id)
            .execute(&self.pool.pool)
            .await?;

        Ok(())
    }
    async fn get_auth_key_by_statechain_id(&self, statechain_id: &str) -> Result<String> {
        let row = sqlx::query(r#"select authkey from statechain where id = $1::uuid"#)
            .bind(statechain_id)
            .fetch_one(&self.pool.pool)
            .await?;

        let authkey: String = row.try_get("authkey")?;

        Ok(authkey)
    }

    async fn get_auth_key_transfer_by_statechain_id(
        &self,
        statechain_id: &str,
    ) -> Result<AuthPubkey> {
        let row = sqlx::query_as::<_, AuthPubkey>(
            r#"select authkey from statechain_transfer where statechain_id = $1::uuid"#,
        )
        .bind(statechain_id)
        .fetch_one(&self.pool.pool)
        .await?;

        Ok(row)
    }

    async fn create_statechain_transfer(
        &self,
        statechain_id: &str,
        authkey: &str,
        random_key: &str,
    ) -> Result<()> {
        let query = sqlx::query(
            "insert into statechain_transfer (authkey,random_key, statechain_id) values ($1,$2,$3::uuid)",
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
            sqlx::query("update statechain_transfer set transfer_msg= $1 where authkey = $2")
                .bind(transfer_msg)
                .bind(authkey)
                .execute(&self.pool.pool)
                .await?;

        Ok(())
    }

    async fn get_transfer_message(&self, authkey: &str) -> Result<String> {
        let row = sqlx::query("select transfer_msg from statechain_transfer where authkey = $1")
            .bind(authkey)
            .fetch_one(&self.pool.pool)
            .await?;
        let msg: String = row.try_get("transfer_msg")?;
        Ok(msg)
    }

    async fn get_verify_statecoin(&self, statechain_id: &str) -> Result<StatecoinVerificationInfo> {
        let row = sqlx::query_as::<_, StatecoinVerificationInfo>(
            r#"select txn, server_public_key, random_point 
            from statechain s join statechain_transfer sf 
            on s.statechain_id = sf.statechain_id 
            where statechain_id  = $1::uuid"#,
        )
        .bind(statechain_id)
        .fetch_one(&self.pool.pool)
        .await?;

        Ok(row)
    }

    async fn get_seckey_and_random_by_statechain_id(
        &self,
        statechain_id: &str,
    ) -> Result<StatecoinSecret> {
        let row = sqlx::query_as::<_, StatecoinSecret>(
            r#"select server_private_key, random_key 
                from statechain s join statechain_transfer sf 
                on s.id = sf.statechain_id 
                where statechain_id  = $1::uuid"#,
        )
        .bind(statechain_id)
        .fetch_one(&self.pool.pool)
        .await?;

        Ok(row)
    }
    async fn update_new_owner(
        &self,
        statechain_id: &str,
        auth_pubkey: &str,
        server_secret_key: &str,
        server_pub_key: &str,
    ) -> Result<()> {
        let mut transaction = self.pool.pool.begin().await.unwrap();

        let query = "update statechain set authkey = $1, server_public_key = $2, server_private_key = $3, txn = txn + 1 where id = $4::uuid;";

        let _ = sqlx::query(query)
            .bind(auth_pubkey)
            .bind(server_pub_key)
            .bind(server_secret_key)
            .bind(statechain_id)
            .execute(&mut *transaction)
            .await
            .unwrap();

        let query = "delete from statechain_transfer where authkey = $1;";

        let _ = sqlx::query(query)
            .bind(auth_pubkey)
            .execute(&mut *transaction)
            .await
            .unwrap();
        // let res = sqlx::query(
        //     r#"
        //     BEGIN;

        //     update statechain
        //     set authkey = $1, server_public_key = $2, server_private_key = $3
        //     where id = $4::uuid;

        //     delete from statechain_transfer
        //     where authkey = $1;

        //     COMMIT;
        //     "#,
        // )
        // .bind(auth_pubkey)
        // .bind(server_pub_key)
        // .bind(server_secret_key)
        // .bind(statechain_id)
        // .execute(&self.pool.pool)
        // .await?;
        transaction.commit().await.unwrap();
        Ok(())
    }
    async fn delete_statecoin_transfer(&self, authkey: &str) -> Result<()> {
        let res = sqlx::query(
            r#"delete from statechain_transfer
                where authkey = $1"#,
        )
        .bind(authkey)
        .execute(&self.pool.pool)
        .await?;
        Ok(())
    }
}

// pub async fn insert_or_update_new_statechain(
//     &self,
//     statechain_id: &str,
//     amount: u32,
//     server_pubkey_share: &PublicKey,
//     aggregated_pubkey: &PublicKey,
//     p2tr_agg_address: &Address,
//     client_pubkey_share: &PublicKey,
//     signed_statechain_id: &Signature,
//     txid: &Txid,
//     vout: u32,
//     locktime: u32,
//     vec_backup_transactions: &Vec<mercury_lib::transfer::ReceiverBackupTransaction>) {

//     let mut transaction = self.pool.begin().await.unwrap();

//     let query = "\
//         DELETE FROM backup_transaction \
//         WHERE statechain_id = $1";

//     let _ = sqlx::query(query)
//         .bind(statechain_id)
//         .execute(&mut *transaction)
//         .await
//         .unwrap();

//     let query = "\
//         DELETE FROM statechain_data \
//         WHERE statechain_id = $1";

//     let _ = sqlx::query(query)
//         .bind(statechain_id)
//         .execute(&mut *transaction)
//         .await
//         .unwrap();

//     let query = "\
//         INSERT INTO statechain_data (statechain_id, amount, server_pubkey_share, aggregated_pubkey, p2tr_agg_address, funding_txid, funding_vout, client_pubkey_share, signed_statechain_id, locktime, status) \
//         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 'AVAILABLE')";

//     let _ = sqlx::query(query)
//         .bind(statechain_id)
//         .bind(amount)
//         .bind(server_pubkey_share.serialize().to_vec())
//         .bind(aggregated_pubkey.serialize().to_vec())
//         .bind(p2tr_agg_address.to_string())
//         .bind(txid.to_string())
//         .bind(vout)
//         .bind(client_pubkey_share.serialize().to_vec())
//         .bind(signed_statechain_id.to_string())
//         .bind(locktime)
//         .execute(&mut *transaction)
//         .await
//         .unwrap();

//     for backup_tx in vec_backup_transactions {

//         let query = "INSERT INTO backup_transaction \
//             (tx_n, statechain_id, client_public_nonce, server_public_nonce, client_pubkey, server_pubkey, blinding_factor, backup_tx, recipient_address) \
//             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)";

//         let tx_bytes = bitcoin::consensus::encode::serialize(&backup_tx.tx);

//         let _ = sqlx::query(query)
//             .bind(backup_tx.tx_n)
//             .bind(statechain_id)
//             .bind(backup_tx.client_public_nonce.serialize().to_vec())
//             .bind(backup_tx.server_public_nonce.serialize().to_vec())
//             .bind(backup_tx.client_public_key.serialize().to_vec())
//             .bind(backup_tx.server_public_key.serialize().to_vec())
//             .bind(backup_tx.blinding_factor.as_bytes().to_vec())
//             .bind(tx_bytes)
//             .bind(backup_tx.recipient_address.clone())
//             .execute(&mut *transaction)
//             .await
//             .unwrap();
//     }

//     transaction.commit().await.unwrap();

// }
// }
