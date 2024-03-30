use async_trait::async_trait;
use secp256k1::{PublicKey, SecretKey, XOnlyPublicKey};

mod source;
pub use source::StatechainRepo;
pub type StatechainError = String;
pub type StatechainResult<T> = Result<T, StatechainError>;

#[async_trait]
pub trait TraitStatechainRepo: Send + Sync + 'static {
    async fn create_deposit_tx(
        &self,
        token_id: &str,
        auth_pubkey: &XOnlyPublicKey,
        server_pubkey: &PublicKey,
        server_privkey: &SecretKey,
        statechain_id: &String,
        amount: u32,
    ) -> StatechainResult<()>;
    async fn get_auth_key_by_statechain_id(&self, statechain_id: &str) -> StatechainResult<String>;
    async fn insert_signature_data(
        &self,
        r2_commitment: &str,
        blind_commitment: &str,
        statechain_id: &str,
        server_pubnonce: &PublicKey,
        server_secnonce: &SecretKey,
    ) -> StatechainResult<()>;
}
