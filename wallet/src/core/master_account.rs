use bitcoin::bip32::Xpub;
use bitcoin::{blockdata::transaction::TxOut, OutPoint, Transaction};
use bitcoin::{EcdsaSighashType, Network, ScriptBuf};
use secp256k1::rand::{thread_rng, RngCore};
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::error::Error;

use super::account::Account;
use super::context::SecpContext;
use super::key_derivation::KeyDerivation;
use super::mnemonic::Mnemonic;
use super::seed::Seed;
use super::unlocker::Unlocker;

/// chose your security level
#[derive(Copy, Clone)]
pub enum MasterKeyEntropy {
    Sufficient = 16,
    Double = 32,
    Paranoid = 64,
}

/// A masterAccount is the root of an account hierarchy
#[derive(Clone, Debug)]
pub struct MasterAccount {
    pub master_public: Xpub,
    encrypted: Vec<u8>,
    accounts: HashMap<(u32, u32), Account>,
    birth: u64,
}

impl MasterAccount {
    /// create a new random master account
    /// the information that leads to private key is stored encrypted with passphrase
    pub fn new(
        entropy: MasterKeyEntropy,
        network: Network,
        passphrase: &str,
    ) -> Result<MasterAccount, Error> {
        let mut random = vec![0u8; entropy as usize];
        thread_rng().fill_bytes(random.as_mut_slice());
        let seed = Seed(random);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Self::from_seed(&seed, now, network, passphrase)
    }

    /// Restore from encrypted store
    pub fn from_encrypted(encrypted: &[u8], public_master_key: Xpub, birth: u64) -> MasterAccount {
        let encrypted = encrypted.to_vec();
        MasterAccount {
            master_public: public_master_key,
            encrypted,
            accounts: HashMap::new(),
            birth,
        }
    }

    /// A watch only master. You will not be able to sign with this.
    pub fn watch_only(public_master_key: Xpub, birth: u64) -> MasterAccount {
        MasterAccount {
            master_public: public_master_key,
            encrypted: Vec::new(),
            accounts: HashMap::new(),
            birth,
        }
    }

    /// Restore from BIP39 mnemonic
    pub fn from_mnemonic(
        mnemonic: &Mnemonic,
        birth: u64,
        network: Network,
        passphrase: &str,
        pd_passphrase: Option<&str>,
    ) -> Result<MasterAccount, Error> {
        let seed = mnemonic.to_seed(pd_passphrase);
        Self::from_seed(&seed, birth, network, passphrase)
    }

    // /// Restore from Shamir's Secret Shares (SLIP-0039)
    // pub fn from_shares(
    //     shares: &[Share],
    //     birth: u64,
    //     network: Network,
    //     passphrase: &str,
    //     pd_passphrase: Option<&str>,
    // ) -> Result<MasterAccount, Error> {
    //     let seed = ShamirSecretSharing::combine(shares, pd_passphrase)?;
    //     Self::from_seed(&seed, birth, network, passphrase)
    // }

    pub fn from_seed(
        seed: &Seed,
        birth: u64,
        network: Network,
        passphrase: &str,
    ) -> Result<MasterAccount, Error> {
        let context = SecpContext::default();
        let encrypted = seed.encrypt(passphrase)?;
        let master_key = context.master_private_key(network, seed)?;
        let public_master_key = context.extended_public_from_private(&master_key);
        Ok(MasterAccount {
            master_public: public_master_key,
            encrypted,
            accounts: HashMap::new(),
            birth,
        })
    }

    pub fn seed(&self, network: Network, passphrase: &str) -> Result<Seed, Error> {
        let context = SecpContext::default();
        let seed = Seed::decrypt(self.encrypted.as_slice(), passphrase)?;
        let master_key = context.master_private_key(network, &seed)?;
        if self.master_public != context.extended_public_from_private(&master_key) {
            return Err(Error::Passphrase);
        }
        Ok(seed)
    }

    pub fn master_public(&self) -> &Xpub {
        &self.master_public
    }

    pub fn encrypted(&self) -> &Vec<u8> {
        &self.encrypted
    }

    pub fn birth(&self) -> u64 {
        self.birth
    }

    pub fn get(&self, account: (u32, u32)) -> Option<&Account> {
        self.accounts.get(&account)
    }

    pub fn get_mut(&mut self, account: (u32, u32)) -> Option<&mut Account> {
        self.accounts.get_mut(&account)
    }

    pub fn accounts(&self) -> &HashMap<(u32, u32), Account> {
        &self.accounts
    }

    pub fn get_scripts(&self) -> impl Iterator<Item = (ScriptBuf, KeyDerivation)> + '_ {
        self.accounts.iter().flat_map(|((an, sub), a)| {
            a.get_scripts().map(move |(kix, s, tweak, csv)| {
                (
                    s,
                    KeyDerivation {
                        account: *an,
                        sub: *sub,
                        kix,
                        tweak,
                        csv,
                    },
                )
            })
        })
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.insert(
            (account.account_number, account.sub_account_number),
            account,
        );
    }

    pub fn sign<R>(
        &self,
        transaction: &mut Transaction,
        hash_type: EcdsaSighashType,
        resolver: &R,
        unlocker: &mut Unlocker,
    ) -> Result<usize, Error>
    where
        R: Fn(&OutPoint) -> Option<TxOut>,
    {
        let mut n_signatures = 0;
        for (_, a) in self.accounts.iter() {
            n_signatures += a.sign(transaction, hash_type, resolver, unlocker)?;
        }
        Ok(n_signatures)
    }
}
