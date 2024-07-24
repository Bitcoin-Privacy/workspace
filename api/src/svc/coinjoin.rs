use std::{ops::Not, str::FromStr};

use anyhow::{anyhow, Result};
use bitcoin::{
    absolute, consensus, transaction::Version, Address, Amount, Network, OutPoint, ScriptBuf,
    Sequence, Transaction, TxIn, TxOut, Witness,
};

use crate::{
    config::CFG,
    model::entity::coinjoin::{Input, RoomEntity},
    repo::coinjoin::CoinjoinRepo,
};
use shared::{api, model::Utxo};

pub struct CoinjoinService {
    repo: CoinjoinRepo,
}

impl CoinjoinService {
    pub fn new(repo: CoinjoinRepo) -> Self {
        Self { repo }
    }

    pub async fn register(
        &self,
        utxo: &[Utxo],
        amount: u32,
        change_addr: &str,
        output_addr: &str,
    ) -> Result<(RoomEntity, String)> {
        // Find compatible room
        let room = self.repo.get_compatible_room(amount).await?;

        let total: u64 = utxo.iter().map(|utxo| utxo.value).sum();
        let est = (amount + CFG.coinjoin_fee) as u64;
        let change = if total > est {
            total - est
        } else {
            return Err(anyhow!("Insufficient funds for CoinJoin fee"));
        };

        let des_addr = super::account::parse_addr_from_str(change_addr, Network::Testnet)
            .map_err(|e| anyhow!("Invalid address: {}", e))?;

        // Update room
        self.repo
            .add_peer(
                room.id,
                utxo.iter().map(|utxo| utxo.txid.to_string()).collect(),
                utxo.iter().map(|utxo| utxo.vout).collect(),
                utxo.iter().map(|utxo| utxo.value).collect(),
                change,
                des_addr.to_string(),
            )
            .await?;

        let sig = super::blindsign::blind_sign(output_addr)?;

        Ok((room, sig))
    }

    pub async fn set_output(&self, room_id: &str, output_addr: &str, sig: &str) -> Result<u8> {
        // Attempt to get the room and handle the error if it doesn't exist
        let room = self.repo.get_room_by_id(room_id).await?;

        // Verify signature
        self.unspent_sig(sig, Some(output_addr)).await?;

        // Attempt to add output and handle any potential error
        self.repo.set_spent_sig(sig).await?;
        self.repo
            .add_output(room_id, output_addr, room.base_amount)
            .await?;
        Ok(0)
    }

    pub async fn set_sig(
        &self,
        room_id: &str,
        address: &str,
        vins: &[u16],
        txn: &str,
    ) -> Result<u8> {
        let parsed_tx = consensus::deserialize::<Transaction>(&hex::decode(txn)?)?;

        self.update_room_status(room_id, 1, None)
            .await
            .map_err(|e| anyhow!("Update room status error: {e}"))?;
        self.repo
            .set_signed(room_id, address, 1)
            .await
            .map_err(|e| anyhow!("Set signed error: {e}"))?;

        for vin in vins.iter() {
            let signed_input = parsed_tx.input.get(*vin as usize);
            let signed_input = signed_input.ok_or(anyhow!("Cannot get signed input"))?;
            let witness = &signed_input.witness;
            witness
                .is_empty()
                .not()
                .then_some(())
                .ok_or(anyhow!("Witness is empty"))?;
            let result = self
                .repo
                .add_script(
                    room_id,
                    *vin,
                    &serde_json::to_string(signed_input).expect("Cannot encode input"),
                )
                .await;
            return match result {
                Ok(_) => continue,
                Err(e) => Err(anyhow!(
                    "Failed to update the signature to database! Detail {e:?}"
                )),
            };
        }

        let completed = self.check_tx_completed(room_id).await;
        match completed {
            Ok(tx) => {
                let tx_hex = consensus::encode::serialize_hex(&tx);
                println!("TX: {:#?}", tx);
                println!("TX completed: {}", tx_hex);

                let res = api::broadcast_tx(tx_hex)
                    .await
                    .map_err(|e| anyhow!("Broadcast txn error: {e:#?}"))?;
                Ok(1)
            }
            Err(e) => {
                println!("Check completed got error: {e}");
                Ok(0)
            }
        }
    }

    pub async fn get_txn_hex(&self, room_id: &str) -> Result<String> {
        let txn = self.get_txn(room_id).await?;
        Ok(consensus::encode::serialize_hex(&txn))
    }

    pub async fn get_signed(&self, room_id: &str, address: &str) -> Result<u8> {
        let optional_signed = self.repo.get_signed(room_id, address).await?;
        match optional_signed {
            Some(signed) => Ok(signed.status),
            None => Ok(0),
        }
    }

    async fn get_txn(&self, room_id: &str) -> Result<bitcoin::Transaction> {
        let raw_inputs = self.repo.get_inputs(room_id).await?;
        let raw_outputs = self.repo.get_outputs(room_id).await?;

        let mut fee = 0;
        let input: Vec<TxIn> = raw_inputs
            .iter()
            .map(|utxo| {
                fee += utxo.amount;
                TxIn {
                    previous_output: OutPoint::new(utxo.txid.parse().unwrap(), utxo.vout.into()),
                    script_sig: ScriptBuf::from_bytes(vec![]),
                    sequence: Sequence::MAX,
                    witness: Witness::new(),
                }
            })
            .collect();

        // Output for the receiver
        let output: Vec<TxOut> = raw_outputs
            .iter()
            .map(|output| {
                fee -= output.amount;
                let addr = Address::from_str(&output.address).unwrap();
                let checked_addr = addr.require_network(Network::Testnet).unwrap();
                TxOut {
                    value: Amount::from_sat(output.amount as u64),
                    script_pubkey: checked_addr.script_pubkey(),
                }
            })
            .collect();

        Ok(Transaction {
            version: Version::TWO,
            lock_time: absolute::LockTime::ZERO,
            input,
            output,
        })
    }

    pub async fn get_rooms_by_addr(&self, addr: &str) -> Result<Vec<RoomEntity>> {
        self.repo.get_rooms_by_addr(addr).await
    }

    pub async fn get_room_by_id(&self, id: &str) -> Result<RoomEntity> {
        self.repo.get_room_by_id(id).await
    }

    pub async fn get_room_detail_by_id(
        &self,
        id: &str,
        addr: &str,
    ) -> Result<(RoomEntity, Vec<Input>)> {
        let room = self.repo.get_room_by_id(id).await?;
        let utxo = self.repo.get_inputs_by_addr(id, addr).await?;
        print!("utxo of {}, {}: {:?}", id, addr, utxo);
        Ok((room, utxo))
    }

    pub async fn check_tx_completed(&self, room_id: &str) -> Result<bitcoin::Transaction> {
        let mut origin_tx = self.get_txn(room_id).await.unwrap();
        // TODO: check valid tx
        let proofs = self.repo.get_proofs(room_id).await.unwrap();
        if origin_tx.input.len() != proofs.len() {
            return Err(anyhow!("TX is not completed yet"));
        }
        for (id, val) in origin_tx.input.iter_mut().enumerate() {
            let proof = proofs.get(id).unwrap();
            let txin = serde_json::from_str::<TxIn>(&proof.script).unwrap();
            if val.previous_output != txin.previous_output {
                return Err(anyhow!("Invalid Proofs"));
            }
            // TODO: check valid signature
            *val = txin;
        }

        Ok(origin_tx)
    }

    pub async fn unspent_sig(&self, sig: &str, msg: Option<&str>) -> Result<()> {
        let valid = match msg {
            Some(msg) => super::blindsign::msg_authenticate(sig, msg)?,
            None => super::blindsign::authenticate(sig)?,
        };
        let is_unspent = self.repo.get_spent_sig(sig).await?;

        if valid {
            if is_unspent {
                Ok(())
            } else {
                Err(anyhow!("Signature was spent"))
            }
        } else {
            Err(anyhow!("Invalid signature"))
        }
    }

    async fn update_room_status(
        &self,
        room_id: &str,
        status: u8,
        require_status: Option<u8>,
    ) -> Result<()> {
        let room = self.repo.get_room_by_id(room_id).await?;
        if require_status.is_some_and(|r| r != room.status) {
            return Err(anyhow!("Invalid status"));
        }
        if room.status != status {
            self.repo.set_room_status(room_id, status).await?;
        };
        Ok(())
    }
}
