use secp256k1::{PublicKey, SecretKey};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{self};
#[derive(sqlx::FromRow, Debug, Clone, Deserialize, Serialize)]
pub struct Statecoin {
    pub tx_n: i64,
    // pub owner_seckey: String,
    pub signed_statechain_id: String,
    pub aggregated_pubkey: String,
    //pub aggregated_address: String,
    pub funding_txid: String,
    pub funding_vout: i64,
    pub key_agg_ctx: String,
    pub amount: i64,
    // pub account: String,
    pub spend_key: String,
}

#[derive(sqlx::FromRow, Debug, Clone, Deserialize, Serialize)]
pub struct StatecoinEntity {
    pub tx_n: i64,
    pub owner_seckey: String,
    pub signed_statechain_id: String,
    pub aggregated_pubkey: String,
    pub aggregated_address: String,
    pub funding_txid: String,
    pub funding_vout: i64,
    pub key_agg_ctx: String,
    pub amount: i64,
    pub account: String,
    pub spend_key: String,
    pub bk_tx: String,
}

impl From<StatecoinEntity> for Statecoin {
    fn from(entity: StatecoinEntity) -> Self {
        Statecoin {
            tx_n: entity.tx_n,
            signed_statechain_id: entity.signed_statechain_id,
            aggregated_pubkey: entity.aggregated_pubkey,
            funding_txid: entity.funding_txid,
            funding_vout: entity.funding_vout,
            key_agg_ctx: entity.key_agg_ctx,
            amount: entity.amount,
            spend_key: entity.spend_key,
        }
    }
}

#[derive(sqlx::FromRow, Debug, Clone, Deserialize, Serialize)]
pub struct StatecoinCard {
    pub statechain_id: String,
    pub amount: i64,
    pub n_lock_time: i64,
}

#[derive(sqlx::FromRow, Debug, Clone, Deserialize, Serialize)]
pub struct StatecoinDetail {
    pub statechain_id: String,
    pub aggregated_address: String,
    pub amount: i64,
    pub tx_n: i64,
    pub n_lock_time: i64,
    pub bk_tx: String,
    pub funding_txid: String,

    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug, Clone, Deserialize, Serialize)]
pub struct TransferStateCoinInfo {
    pub auth_key: String,
    pub transfer_message: String,
}

#[derive(sqlx::FromRow, Debug, Clone, Deserialize, Serialize)]
pub struct StatechainKeypairs {
    pub owner_seckey: SecretKey,
    pub owner_pubkey: PublicKey,
    pub auth_seckey: SecretKey,
    pub auth_pubkey: PublicKey,
}
