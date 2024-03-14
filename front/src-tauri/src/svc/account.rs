use bitcoin::bip32::{ChildNumber, Xpub};
use bitcoin::hashes::{hash160, Hash};
use bitcoin::script::PushBytesBuf;
use bitcoin::sighash::SighashCache;
use bitcoin::{
    blockdata::script::Builder,
    blockdata::{opcodes::all, transaction::TxOut},
    secp256k1::PublicKey,
    Address, OutPoint, Transaction,
};
use bitcoin::{EcdsaSighashType, Network, PrivateKey, ScriptBuf};
use core::panic;
use secp256k1::rand::{thread_rng, RngCore};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::error::Error;
use crate::model::{AccountAddressType, KeyDerivation};

use super::context::SecpContext;
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
        let context = SecpContext::new();
        let encrypted = seed.encrypt(passphrase)?;
        let master_key = context.master_private_key(network, &seed)?;
        let public_master_key = context.extended_public_from_private(&master_key);
        Ok(MasterAccount {
            master_public: public_master_key,
            encrypted,
            accounts: HashMap::new(),
            birth,
        })
    }

    pub fn seed(&self, network: Network, passphrase: &str) -> Result<Seed, Error> {
        let context = SecpContext::new();
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

    pub fn get_scripts<'a>(&'a self) -> impl Iterator<Item = (ScriptBuf, KeyDerivation)> + 'a {
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

#[derive(Clone, Debug)]
pub struct Account {
    pub address_type: AccountAddressType,
    pub account_number: u32,
    pub sub_account_number: u32,
    pub context: Arc<SecpContext>,
    pub master_public: Xpub,
    pub instantiated: Vec<InstantiatedKey>,
    pub next: u32,
    pub look_ahead: u32,
    pub network: Network,
}

impl Account {
    pub fn new(
        unlocker: &mut Unlocker,
        address_type: AccountAddressType,
        account_number: u32,
        sub_account_number: u32,
        look_ahead: u32,
    ) -> Result<Account, Error> {
        let context = Arc::new(SecpContext::new());
        let master_private =
            unlocker.sub_account_key(address_type, account_number, sub_account_number)?;
        let pubic_key = context.extended_public_from_private(&master_private);
        let mut sub = Account {
            address_type,
            account_number,
            sub_account_number,
            context,
            master_public: pubic_key,
            instantiated: Vec::new(),
            next: 0,
            look_ahead,
            network: pubic_key.network,
        };
        sub.do_look_ahead(None)?;
        Ok(sub)
    }

    pub fn new_from_storage(
        address_type: AccountAddressType,
        account_number: u32,
        sub_account_number: u32,
        master_public: Xpub,
        instantiated: Vec<InstantiatedKey>,
        next: u32,
        look_ahead: u32,
        network: Network,
    ) -> Account {
        let context = Arc::new(SecpContext::new());
        Account {
            address_type,
            account_number,
            sub_account_number,
            context,
            master_public,
            instantiated,
            next,
            look_ahead,
            network,
        }
    }

    pub fn address_type(&self) -> AccountAddressType {
        self.address_type
    }

    pub fn account_number(&self) -> u32 {
        self.account_number
    }

    pub fn sub_account_number(&self) -> u32 {
        self.sub_account_number
    }

    pub fn master_public(&self) -> &Xpub {
        &self.master_public
    }

    pub fn next(&self) -> u32 {
        self.next
    }

    pub fn look_ahead(&self) -> u32 {
        self.look_ahead
    }

    pub fn network(&self) -> Network {
        self.network
    }

    pub fn instantiated(&self) -> &Vec<InstantiatedKey> {
        &self.instantiated
    }

    /// look ahead from last seen
    pub fn do_look_ahead(&mut self, seen: Option<u32>) -> Result<Vec<(u32, ScriptBuf)>, Error> {
        use std::cmp::max;

        if let Some(seen) = seen {
            self.next = max(self.next, seen + 1);
        }

        let seen = seen.unwrap_or(0);
        let have = self.instantiated.len() as u32;
        let need = max(seen + self.look_ahead, have) - have;
        let mut new = Vec::new();
        for i in 0..need {
            new.push((
                have + i,
                self.instantiate_more()?.address.script_pubkey().clone(),
            ));
        }
        Ok(new)
    }

    fn instantiate_more(&mut self) -> Result<&InstantiatedKey, Error> {
        let kix = self.instantiated.len() as u32;

        let scripter = |public: &PublicKey, _| match self.address_type {
            AccountAddressType::P2SHWPKH => Builder::new()
                .push_opcode(all::OP_DUP)
                .push_opcode(all::OP_HASH160)
                .push_slice(hash160::Hash::hash(&public.serialize()).to_byte_array())
                .push_opcode(all::OP_EQUALVERIFY)
                .push_opcode(all::OP_CHECKSIG)
                .into_script(),
            AccountAddressType::P2WPKH => Builder::new()
                .push_opcode(all::OP_DUP)
                .push_opcode(all::OP_HASH160)
                .push_slice(hash160::Hash::hash(&public.serialize()).to_byte_array())
                .push_opcode(all::OP_EQUALVERIFY)
                .push_opcode(all::OP_CHECKSIG)
                .into_script(),
            _ => ScriptBuf::new(),
        };
        let instantiated = InstantiatedKey::new(
            self.address_type,
            self.network,
            &self.master_public,
            None,
            kix,
            scripter,
            None,
            self.context.clone(),
        )?;

        let len = self.instantiated.len();
        self.instantiated.push(instantiated);
        Ok(&self.instantiated[len])
    }

    /// create a new key
    pub fn next_key(&mut self) -> Result<&InstantiatedKey, Error> {
        match self.address_type {
            AccountAddressType::P2WSH(_) => {
                return Err(Error::Unsupported(
                    "next_key can not be used for P2WSH accounts",
                ))
            }
            _ => {}
        }
        self.instantiate_more()?;
        let key = &self.instantiated[self.next as usize];
        self.next += 1;
        Ok(&key)
    }

    pub fn compute_base_public_key(&self, kix: u32) -> Result<PublicKey, Error> {
        Ok(self
            .context
            .public_child(&self.master_public, ChildNumber::Normal { index: kix })?
            .public_key)
    }

    /// get a previously instantiated key
    pub fn get_key(&self, kix: u32) -> Option<&InstantiatedKey> {
        self.instantiated.get(kix as usize)
    }

    pub fn add_script_key<W>(
        &mut self,
        scripter: W,
        tweak: Option<&[u8]>,
        csv: Option<u16>,
    ) -> Result<u32, Error>
    where
        W: FnOnce(&PublicKey, Option<u16>) -> ScriptBuf,
    {
        match self.address_type {
            AccountAddressType::P2WSH(_) => {}
            _ => {
                return Err(Error::Unsupported(
                    "add_script_key can only be used for P2WSH accounts",
                ))
            }
        }
        let kix = self.instantiated.len() as u32;
        let instantiated = InstantiatedKey::new(
            self.address_type,
            self.network,
            &self.master_public,
            tweak,
            kix,
            scripter,
            csv,
            self.context.clone(),
        )?;
        self.instantiated.push(instantiated);
        Ok(kix)
    }

    pub fn used(&self) -> usize {
        self.next as usize
    }

    // get all pubkey scripts of this account
    pub fn get_scripts<'a>(
        &'a self,
    ) -> impl Iterator<Item = (u32, ScriptBuf, Option<Vec<u8>>, Option<u16>)> + 'a {
        self.instantiated.iter().enumerate().map(|(kix, i)| {
            (
                kix as u32,
                i.address.script_pubkey().clone(),
                i.tweak.clone(),
                i.csv.clone(),
            )
        })
    }

    pub fn get_privkey(
        &self,
        script_pubkey: ScriptBuf,
        unlocker: &mut Unlocker,
    ) -> Result<PrivateKey, Error> {
        if let Some((kix, instantiated)) = self
            .instantiated
            .iter()
            .enumerate()
            .find(|(_, i)| i.address.script_pubkey() == script_pubkey)
        {
            let priv_key = unlocker.unlock(
                self.address_type,
                self.account_number,
                self.sub_account_number,
                kix as u32,
                instantiated.tweak.clone(),
            )?;
            Ok(priv_key)
        } else {
            Err(Error::Unsupported("Cannot find suitable subaccount"))
        }
    }

    /// sign a transaction with keys in this account works for types except P2WSH
    pub fn sign<R>(
        &self,
        transaction: &mut Transaction,
        hash_type: EcdsaSighashType,
        resolver: R,
        unlocker: &mut Unlocker,
    ) -> Result<usize, Error>
    where
        R: Fn(&OutPoint) -> Option<TxOut>,
    {
        let mut signed = 0;
        //TODO(stevenroose) try to prevent this clone here
        let mut txclone = transaction.clone();
        let mut bip143hasher = SighashCache::new(&mut txclone);
        for (ix, input) in transaction.input.iter_mut().enumerate() {
            if let Some(spend) = resolver(&input.previous_output) {
                if let Some((kix, instantiated)) = self
                    .instantiated
                    .iter()
                    .enumerate()
                    .find(|(_, i)| i.address.script_pubkey() == spend.script_pubkey)
                {
                    let priv_key = unlocker.unlock(
                        self.address_type,
                        self.account_number,
                        self.sub_account_number,
                        kix as u32,
                        instantiated.tweak.clone(),
                    )?;
                    match self.address_type {
                        AccountAddressType::P2PKH => {
                            let sighash = bip143hasher
                                .legacy_signature_hash(
                                    ix,
                                    &instantiated.address.script_pubkey(),
                                    hash_type.to_u32(),
                                )
                                .unwrap();
                            let slice: &[u8] = &sighash[..];
                            let array_ref: &[u8; 32] =
                                slice.try_into().expect("Slice has incorrect length");
                            let signature =
                                self.context.sign(array_ref, &priv_key)?.serialize_der();
                            let mut with_hashtype = PushBytesBuf::new();
                            // let mut with_hashtype = signature.to_vec();
                            let _ = with_hashtype.push(hash_type.to_u32() as u8);
                            input.script_sig = Builder::new()
                                .push_slice(with_hashtype.as_push_bytes())
                                .push_slice(instantiated.public.serialize())
                                .into_script();
                            input.witness.clear();
                            signed += 1;
                        }
                        AccountAddressType::P2WPKH => {
                            if hash_type.to_u32() & EcdsaSighashType::All.to_u32() == 0 {
                                return Err(Error::Unsupported("can only sign all inputs for now"));
                            }
                            input.script_sig = ScriptBuf::new();
                            println!("Script code 2: {}", instantiated.script_code.to_string());

                            let sighash = bip143hasher
                                .p2wpkh_signature_hash(
                                    ix,
                                    // &instantiated.script_code,
                                    &spend.script_pubkey,
                                    spend.value,
                                    hash_type,
                                )
                                .unwrap();
                            let slice: &[u8] = &sighash[..];
                            let array_ref: &[u8; 32] =
                                slice.try_into().expect("Slice has incorrect length");
                            let signature =
                                self.context.sign(array_ref, &priv_key)?.serialize_der();
                            let mut with_hashtype = signature.to_vec();
                            with_hashtype.push(hash_type.to_u32() as u8);
                            input.witness.clear();
                            input.witness.push(with_hashtype);
                            input.witness.push(instantiated.public.serialize());
                            signed += 1;
                        }
                        _ => {
                            panic!("NOT SUPPORT YET")
                        } // AccountAddressType::P2SHWPKH => {
                          //     if hash_type.to_u32() & EcdsaSighashType::All.to_u32() == 0 {
                          //         return Err(Error::Unsupported("can only sign all inputs for now"));
                          //     }
                          //     input.script_sig = Builder::new()
                          //         .push_slice(
                          //             &Builder::new()
                          //                 .push_int(0)
                          //                 .push_slice(
                          //                     &hash160::Hash::hash(
                          //                         instantiated.public.to_bytes().as_slice(),
                          //                     )[..],
                          //                 )
                          //                 .into_script()[..],
                          //         )
                          //         .into_script();
                          //     let sighash = bip143hasher.signature_hash(
                          //         ix,
                          //         &instantiated.script_code,
                          //         spend.value,
                          //         hash_type,
                          //     );
                          //     let signature =
                          //         self.context.sign(&sighash[..], &priv_key)?.serialize_der();
                          //     let mut with_hashtype = signature.to_vec();
                          //     with_hashtype.push(hash_type.as_u32() as u8);
                          //     input.witness.clear();
                          //     input.witness.push(with_hashtype);
                          //     input.witness.push(instantiated.public.to_bytes());
                          //     signed += 1;
                          // }
                          // AccountAddressType::P2WSH(_) => {
                          //     if hash_type.to_u32() & EcdsaSighashType::All.to_u32() == 0 {
                          //         return Err(Error::Unsupported("can only sign all inputs for now"));
                          //     }
                          //     input.script_sig = ScriptBuf::new();
                          //     let sighash = bip143hasher.signature_hash(
                          //         ix,
                          //         &instantiated.script_code,
                          //         spend.value,
                          //         hash_type,
                          //     );
                          //     let signature =
                          //         self.context.sign(&sighash[..], &priv_key)?.serialize_der();
                          //     let mut with_hashtype = signature.to_vec();
                          //     with_hashtype.push(hash_type.to_u32() as u8);
                          //     input.witness.clear();
                          //     input.witness.push(with_hashtype);
                          //     input.witness.push(instantiated.script_code.to_bytes());
                          //     signed += 1;
                          // }
                    }
                }
            }
        }
        Ok(signed)
    }
}

/// instantiated key of an account
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
        address_type: AccountAddressType,
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
            AccountAddressType::P2PKH => Address::p2pkh(&btc_pub_key, network),
            AccountAddressType::P2SHWPKH => {
                Address::p2shwpkh(&btc_pub_key, network).expect("compressed pubkey")
            }
            AccountAddressType::P2WPKH => {
                Address::p2wpkh(&btc_pub_key, network).expect("compressed pubkey")
            }
            AccountAddressType::P2WSH(_) => Address::p2wsh(&script_code, network),
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
