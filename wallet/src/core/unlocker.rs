use bitcoin::bip32::{ChildNumber, Xpriv, Xpub};
use bitcoin::Network;
use bitcoin::PrivateKey;
use std::{collections::HashMap, sync::Arc};

use crate::error::Error;

use super::account_address_type::AccountAddressType;
use super::context::SecpContext;
use super::master_account::MasterAccount;
use super::seed::Seed;

/// calculator of private keys
pub struct Unlocker {
    master_private: Xpriv,
    network: Network,
    context: Arc<SecpContext>,
    cached: HashMap<
        AccountAddressType,
        (
            Xpriv,
            HashMap<u32, (Xpriv, HashMap<u32, (Xpriv, HashMap<u32, Xpriv>)>)>,
        ),
    >,
}

impl Unlocker {
    /// decrypt encrypted seed of a master account
    /// check result if master_public is provided
    pub fn new(
        encrypted: &[u8],
        passphrase: &str,
        network: Network,
        master_public: Option<&Xpub>,
    ) -> Result<Unlocker, Error> {
        let seed = Seed::decrypt(encrypted, passphrase)?;
        let context = Arc::new(SecpContext::new());
        let master_private = context.master_private_key(network, &seed)?;
        if let Some(master_public) = master_public {
            if network != master_public.network {
                return Err(Error::Network);
            }
            if context.extended_public_from_private(&master_private) != *master_public {
                return Err(Error::Passphrase);
            }
        }
        Ok(Unlocker {
            master_private,
            network,
            context,
            cached: HashMap::new(),
        })
    }

    pub fn new_for_master(master: &MasterAccount, passphrase: &str) -> Result<Unlocker, Error> {
        Self::new(
            master.encrypted(),
            passphrase,
            master.master_public.network,
            Some(&master.master_public),
        )
    }

    pub fn master_private(&self) -> &Xpriv {
        &self.master_private
    }

    pub fn sub_account_key(
        &mut self,
        address_type: AccountAddressType,
        account: u32,
        sub_account: u32,
    ) -> Result<Xpriv, Error> {
        let by_purpose = self.cached.entry(address_type).or_insert((
            self.context.private_child(
                &self.master_private,
                ChildNumber::Hardened {
                    index: address_type.as_u32(),
                },
            )?,
            HashMap::new(),
        ));
        let coin_type = match self.network {
            Network::Bitcoin => 0,
            Network::Testnet => 1,
            Network::Regtest => 1,
            Network::Signet => 1,
            _ => 2, // NOTE: support testnet only
        };
        let by_coin_type = by_purpose.1.entry(coin_type).or_insert((
            self.context
                .private_child(&by_purpose.0, ChildNumber::Hardened { index: coin_type })?,
            HashMap::new(),
        ));
        let by_account = by_coin_type.1.entry(account).or_insert((
            self.context
                .private_child(&by_coin_type.0, ChildNumber::Hardened { index: account })?,
            HashMap::new(),
        ));
        Ok(self
            .context
            .private_child(&by_account.0, ChildNumber::Normal { index: sub_account })?)
    }

    pub fn unlock(
        &mut self,
        address_type: AccountAddressType,
        account: u32,
        sub_account: u32,
        index: u32,
        tweak: Option<Vec<u8>>,
    ) -> Result<PrivateKey, Error> {
        let sub_account_key = self.sub_account_key(address_type, account, sub_account)?;
        let mut key = self
            .context
            .private_child(&sub_account_key, ChildNumber::Normal { index })?
            .private_key;
        if let Some(tweak) = tweak {
            let slice: &[u8] = tweak.as_slice();
            if slice.len() == 32 {
                let array_ref: &[u8; 32] = slice.try_into().expect("Slice with incorrect length");
                self.context.tweak_add(&mut key, array_ref)?;
            } else {
                // Handle error: the slice is not 32 bytes long
            }
        }
        Ok(PrivateKey {
            compressed: true,
            network: Network::Testnet,
            inner: key,
        })
    }

    pub fn context(&self) -> Arc<SecpContext> {
        self.context.clone()
    }
}
