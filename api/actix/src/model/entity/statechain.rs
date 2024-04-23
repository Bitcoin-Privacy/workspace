use shared::intf::statechain::StatecoinDto;

// ---------------------------
// Statecoin table
// ---------------------------
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct StateCoin {
    pub id: uuid::Uuid,
    pub token_id: String,
    pub auth_xonly_public_key: String,
    pub server_public_key: String,
    pub server_private_key: String,
    #[sqlx(try_from = "i64")]
    pub amount: u32,
    pub sec_nonce: Option<String>,
    pub pub_nonce: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Pubnonce {
    pub pub_nonce: String,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct AuthPubkey {
    pub auth_xonly_public_key: String,
}

impl From<StateCoin> for StatecoinDto {
    fn from(value: StateCoin) -> Self {
        StatecoinDto {
            id: value.id.to_string(),
            token_id: value.token_id,
            auth_xonly_public_key: value.auth_xonly_public_key,
            server_public_key: value.server_public_key,
            server_private_key: value.server_private_key,
            amount: value.amount,
            sec_nonce: value.sec_nonce,
            pub_nonce: value.pub_nonce,
            created_at: value.created_at.timestamp_millis() as u64,
            updated_at: value.updated_at.timestamp_millis() as u64,
        }
    }
}

impl From<&StateCoin> for StatecoinDto {
    fn from(value: &StateCoin) -> Self {
        StatecoinDto {
            id: value.id.to_string(),
            token_id: value.token_id.clone(),
            auth_xonly_public_key: value.auth_xonly_public_key.clone(),
            server_public_key: value.server_public_key.clone(),
            server_private_key: value.server_private_key.clone(),
            amount: value.amount,
            sec_nonce: value.sec_nonce.clone(),
            pub_nonce: value.pub_nonce.clone(),
            created_at: value.created_at.timestamp_millis() as u64,
            updated_at: value.updated_at.timestamp_millis() as u64,
        }
    }
}
