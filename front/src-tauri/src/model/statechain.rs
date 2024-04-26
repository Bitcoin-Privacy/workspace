use bitcoin::Address;
use serde::{Deserialize, Serialize};
#[derive(sqlx::FromRow, Debug, Clone, Deserialize, Serialize)]

pub struct StateCoin {
    pub signed_statechain_id: String,
    pub aggregated_pubkey: String,
    pub aggregated_address: String,
    pub funding_txid: String,
    pub funding_vout: i64,
    pub key_agg_ctx: String,
    pub amount: i64,
    pub deriv: String,
}

#[derive(sqlx::FromRow, Debug, Clone, Deserialize, Serialize)]
pub struct StateCoinInfo {
    pub statechain_id: String,
    pub aggregated_address: String,
    pub amount: i64,
    pub funding_txid: String,
    pub funding_vout: i64,
    pub n_lock_time: i64,
}
