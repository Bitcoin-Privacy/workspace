use anyhow::Result;
use async_trait::async_trait;
use bitcoin::{
    secp256k1::{PublicKey, SecretKey},
    XOnlyPublicKey,
};

mod source;
pub use source::StatechainRepo;

use crate::model::entity::statechain::{AuthPubkey, StateCoin};

#[async_trait]
pub trait TraitStatechainRepo: Send + Sync + 'static {
    async fn create_deposit_tx(
        &self,
        token_id: &str,
        auth_pubkey: &XOnlyPublicKey,
        server_pubkey: &PublicKey,
        server_privkey: &SecretKey,
        amount: u32,
    ) -> Result<StateCoin>;

    async fn update_nonce(&self, secnonce: &str, statechain_id: &str) -> Result<()>;
    async fn get_auth_key_by_statechain_id(&self, statechain_id: &str) -> Result<AuthPubkey>;
    async fn create_statechain_transfer(
        &self,
        statechain_id: &str,
        authkey: &str,
        random_key: &str,
    ) -> Result<()>;
    async fn update_transfer_message(&self, authkey: &str, transfer_msg: &str) -> Result<()>;
}
