use bitcoin::secp256k1::ecdsa::Signature;
use bitcoin::secp256k1::{All, Message, PublicKey, Scalar, Secp256k1, SecretKey};
use bitcoin::{
    bip32::{ChildNumber, Xpriv, Xpub},
    Network, PrivateKey,
};

use super::seed::Seed;
use crate::error::Error;

#[derive(Debug)]
pub struct SecpContext {
    secp: Secp256k1<All>,
}

impl Default for SecpContext {
    fn default() -> Self {
        SecpContext {
            secp: Secp256k1::new(),
        }
    }
}

impl SecpContext {
    /// create a master private key from seed
    pub fn master_private_key(&self, network: Network, seed: &Seed) -> Result<Xpriv, Error> {
        Ok(Xpriv::new_master(network, &seed.0)?)
    }

    /// get extended public key for a known private key
    pub fn extended_public_from_private(&self, extended_private_key: &Xpriv) -> Xpub {
        Xpub::from_priv(&self.secp, extended_private_key)
    }

    pub fn private_child(
        &self,
        extended_private_key: &Xpriv,
        child: ChildNumber,
    ) -> Result<Xpriv, Error> {
        Ok(extended_private_key.derive_priv(&self.secp, &child)?)
    }

    pub fn public_child(
        &self,
        extended_public_key: &Xpub,
        child: ChildNumber,
    ) -> Result<Xpub, Error> {
        Ok(extended_public_key.ckd_pub(&self.secp, child)?)
    }

    pub fn public_from_private(&self, private: &PrivateKey) -> bitcoin::PublicKey {
        bitcoin::PublicKey::from_private_key(&self.secp, private)
    }

    pub fn sign(&self, digest: &[u8; 32], key: &PrivateKey) -> Result<Signature, Error> {
        Ok(self
            .secp
            .sign_ecdsa(&Message::from_digest(*digest), &key.inner))
    }

    pub fn tweak_add(&self, key: &mut SecretKey, tweak: &[u8; 32]) -> Result<(), Error> {
        key.add_tweak(&Scalar::from_be_bytes(*tweak).unwrap())?;
        Ok(())
    }

    pub fn tweak_exp_add(&self, key: &mut PublicKey, tweak: &[u8; 32]) -> Result<(), Error> {
        key.add_exp_tweak(&self.secp, &Scalar::from_be_bytes(*tweak).unwrap())?;
        Ok(())
    }
}
