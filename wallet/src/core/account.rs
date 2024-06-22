use anyhow::{anyhow, Result};
use bitcoin::bip32::{ChildNumber, Xpub};
use bitcoin::hashes::{hash160, Hash};
use bitcoin::script::PushBytesBuf;
use bitcoin::sighash::SighashCache;
use bitcoin::{
    blockdata::script::Builder,
    blockdata::{opcodes::all, transaction::TxOut},
    secp256k1::PublicKey,
    OutPoint, Transaction,
};
use bitcoin::{EcdsaSighashType, Network, PrivateKey, ScriptBuf};
use shared::{api, model::Utxo};
use std::sync::Arc;

use crate::error::Error;

use super::{AddrType, InstantiatedKey, SecpContext, Unlocker};

#[derive(Clone, Debug)]
pub struct Account {
    pub address_type: AddrType,
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
        address_type: AddrType,
        account_number: u32,
        sub_account_number: u32,
        look_ahead: u32,
    ) -> Result<Account, Error> {
        let context = Arc::new(SecpContext::default());
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
        address_type: AddrType,
        account_number: u32,
        sub_account_number: u32,
        master_public: Xpub,
        instantiated: Vec<InstantiatedKey>,
        next: u32,
        look_ahead: u32,
        network: Network,
    ) -> Account {
        let context = Arc::new(SecpContext::default());
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

    fn get_addr(&self) -> String {
        return self.get_key(0).unwrap().address.to_string();
    }

    pub fn address_type(&self) -> AddrType {
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

    /// Look ahead from last seen
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
            AddrType::P2WPKH => Builder::new()
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
        self.instantiate_more()?;
        let key = &self.instantiated[self.next as usize];
        self.next += 1;
        Ok(key)
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

    pub fn used(&self) -> usize {
        self.next as usize
    }

    // Get all pubkey scripts of this account
    pub fn get_scripts(
        &self,
    ) -> impl Iterator<Item = (u32, ScriptBuf, Option<Vec<u8>>, Option<u16>)> + '_ {
        self.instantiated.iter().enumerate().map(|(kix, i)| {
            (
                kix as u32,
                i.address.script_pubkey().clone(),
                i.tweak.clone(),
                i.csv,
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

    pub async fn get_utxo(&self, amount: u64) -> Result<Vec<Utxo>> {
        let utxos = api::get_utxo(&self.get_addr()).await?;
        let mut utxos: Vec<&Utxo> = utxos.iter().filter(|utxo| utxo.status.confirmed).collect();

        // Sort UTXOs in descending order by value
        utxos.sort_by(|a, b| b.value.cmp(&a.value));

        let mut selected_utxos: Vec<Utxo> = Vec::new();
        let mut total: u64 = 0;

        for utxo in utxos {
            if total >= amount {
                break;
            }
            selected_utxos.push(utxo.clone());
            total += utxo.value;
        }

        if total >= amount {
            Ok(selected_utxos)
        } else {
            Err(anyhow!("Do not have compatible UTXOs")) // Not enough funds
        }
    }

    /// Sign a transaction with keys in this account works for types except P2WSH
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
        // TODO: try to prevent this clone here
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
                        AddrType::P2PKH => {
                            let sighash = bip143hasher
                                .legacy_signature_hash(
                                    ix,
                                    &instantiated.address.script_pubkey(),
                                    hash_type.to_u32(),
                                )
                                .map_err(Error::SigHash)?;
                            let slice: &[u8] = &sighash[..];
                            let array_ref: &[u8; 32] =
                                slice.try_into().expect("Slice has incorrect length");
                            let signature =
                                self.context.sign(array_ref, &priv_key)?.serialize_der();
                            let mut with_hashtype = PushBytesBuf::new();
                            with_hashtype
                                .extend_from_slice(&signature)
                                .map_err(Error::PushBytesError)?;
                            with_hashtype
                                .push(hash_type.to_u32() as u8)
                                .map_err(Error::PushBytesError)?;
                            input.script_sig = Builder::new()
                                .push_slice(with_hashtype)
                                .push_slice(instantiated.public.serialize())
                                .into_script();
                            input.witness.clear();
                            signed += 1;
                        }
                        AddrType::P2WPKH => {
                            if hash_type.to_u32() & EcdsaSighashType::All.to_u32() == 0 {
                                return Err(Error::Unsupported("can only sign all inputs for now"));
                            }
                            input.script_sig = ScriptBuf::new();
                            println!("Script code 2: {}", instantiated.script_code);

                            let sighash = bip143hasher
                                .p2wpkh_signature_hash(
                                    ix,
                                    // &instantiated.script_code,
                                    &spend.script_pubkey,
                                    spend.value,
                                    hash_type,
                                )
                                .map_err(Error::SigHash)?;
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
                        } // _ => {
                          //     panic!("NOT SUPPORT YET")
                          // }
                    }
                }
            }
        }
        Ok(signed)
    }
}
