use serde::{Deserialize, Serialize};
#[derive(sqlx::FromRow, Debug, Clone, Deserialize, Serialize)]

pub struct StateCoin {
    pub statechain_id: String,
    pub deriv: String,
    pub aggregated_address: String,
    pub amount: i64,
    pub funding_tx: String,
    pub backup_tx: String,
    pub tx_n: i64,
    pub n_lock_time: i64,
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
