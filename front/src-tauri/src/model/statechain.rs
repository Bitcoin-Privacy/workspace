use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Clone, Deserialize, Serialize)]
pub struct StateCoin {
    pub statechain_id: String,
    pub deriv: String,
    pub amount: i64,
    pub owner_pubkey: String,
    pub owner_seckey: String,
    pub funding_txid: String,
    pub funding_vout: i64,
    pub status: String,
    pub funding_tx: String,
}
