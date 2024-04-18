use anyhow::Result;
use async_trait::async_trait;
use bitcoin::{
    secp256k1::{PublicKey, SecretKey},
    XOnlyPublicKey,
};

mod source;
use musig2::{PubNonce, SecNonce};
pub use source::StatechainRepo;

use crate::model::entity::statechain::{AuthPubkey, Pubnonce, StateCoin};

#[async_trait]
pub trait TraitStatechainRepo: Send + Sync + 'static {
    async fn create_deposit_tx(
        &self,
        token_id: &str,
        auth_pubkey: &XOnlyPublicKey,
        server_pubkey: &PublicKey,
        server_privkey: &SecretKey,
        amount: u32,
        secnonce: &SecNonce,
        pubnonce: &PubNonce,
    ) -> Result<StateCoin>;

    async fn get_nonce(&self, statechain_id: &str) -> Result<Pubnonce>;
    async fn get_auth_key_by_statechain_id(&self, statechain_id: &str) -> Result<AuthPubkey>;
}
