use bitcoin::bip32::{ChildNumber, Xpub};
use bitcoin::{secp256k1::PublicKey, Address};
use bitcoin::{Network, ScriptBuf};
use std::sync::Arc;

use crate::error::Error;

use super::{AddrType, SecpContext};

/// Instantiated key of an account
#[derive(Clone, Debug)]
pub struct InstantiatedKey {
    pub public: PublicKey,
    pub script_code: ScriptBuf,
    pub address: Address,
    pub tweak: Option<Vec<u8>>,
    pub csv: Option<u16>,
}

impl InstantiatedKey {
    pub fn new<W>(
        address_type: AddrType,
        network: Network,
        master: &Xpub,
        tweak: Option<&[u8]>,
        kix: u32,
        scripter: W,
        csv: Option<u16>,
        context: Arc<SecpContext>,
    ) -> Result<InstantiatedKey, Error>
    where
        W: FnOnce(&PublicKey, Option<u16>) -> ScriptBuf,
    {
        let mut public = context
            .public_child(master, ChildNumber::Normal { index: kix })?
            .public_key;
        if let Some(tweak) = tweak {
            let array_ref: &[u8; 32] = tweak.try_into().expect("Slice has incorrect length");
            context.tweak_exp_add(&mut public, array_ref)?;
        }
        let script_code = scripter(&public, csv);
        // assert!(public.compressed);
        let btc_pub_key = bitcoin::PublicKey {
            compressed: true,
            inner: public,
        };

        let address = match address_type {
            AddrType::P2PKH => Address::p2pkh(&btc_pub_key, network),
            AddrType::P2WPKH => Address::p2wpkh(&btc_pub_key, network).expect("compressed pubkey"),
        };
        Ok(InstantiatedKey {
            public,
            script_code,
            address,
            tweak: tweak.map(|t| t.to_vec()),
            csv,
        })
    }
}
