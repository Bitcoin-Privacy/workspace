use std::str::FromStr;

use actix_web::web::Data;
use anyhow::{anyhow, Result};
use bitcoin::{
    absolute, consensus, transaction::Version, Address, Amount, Network, OutPoint, ScriptBuf,
    Sequence, Transaction, TxIn, TxOut, Witness,
};
use curve25519_dalek::RistrettoPoint;

use crate::{
    constance::COINJOIN_FEE,
    model::entity::coinjoin::Room,
    repo::coinjoin::{CoinJoinRepo, TraitCoinJoinRepo},
    CFG,
};
use shared::model::Utxo;

pub async fn register(
    repo: &Data<CoinJoinRepo>,
    utxo: &[Utxo],
    amount: u32,
    change_addr: &str,
    output_addr: &str,
) -> Result<(Room, String)> {
    // Find compatible room
    let room = repo.get_compatible_room(amount).await?;

    // Update room
    let first_utxo = utxo.first().unwrap();

    let total: u64 = utxo.iter().map(|utxo| utxo.value).sum();
    let est = (amount + COINJOIN_FEE) as u64;
    let change = if total > est {
        total - est
    } else {
        return Err(anyhow!("Insufficient funds for CoinJoin fee"));
    };

    let des_addr = super::account::parse_addr_from_str(change_addr, Network::Testnet)?;
    let add_peer_res = repo
        .add_peer(
            room.id,
            vec![first_utxo.txid.to_string()],
            vec![first_utxo.vout],
            vec![first_utxo.value],
            change,
            des_addr.to_string(),
        )
        .await;

    println!("ADD PEER RES: {:#?}", add_peer_res);

    let sig = super::blindsign::blind_sign(output_addr)?;

    Ok((room, sig))
}

pub async fn set_output(
    repo: Data<CoinJoinRepo>,
    room_id: &str,
    output_addr: &str,
    sig: &str,
) -> Result<u8> {
    // Attempt to get the room and handle the error if it doesn't exist
    let room = repo.get_room_by_id(room_id).await?;

    // Attempt to add output and handle any potential error
    repo.add_output(room_id, output_addr, room.base_amount)
        .await
        .map_err(|e| anyhow!("Failed to add output: {}", e))?;

    let keypair = CFG.blind_keypair;

    // Process signature errors in one go
    let valid = validate_signature(sig, keypair.public(), output_addr)?;

    if valid {
        Ok(0)
    } else {
        Err(anyhow!("Invalid signature"))
    }
}

pub async fn set_sig(
    repo: Data<CoinJoinRepo>,
    room_id: &str,
    vins: &[u16],
    txn: &str,
) -> Result<bool> {
    let parsed_tx = consensus::deserialize::<Transaction>(&hex::decode(txn)?)?;

    for vin in vins.iter() {
        let signed_input = parsed_tx.input.get(*vin as usize);
        if let Some(signed_input) = signed_input {
            let witness = &signed_input.witness;
            if witness.is_empty() {
                return Err(anyhow!("Empty witness!"));
            };
            let result = repo
                .add_script(
                    room_id,
                    *vin,
                    &serde_json::to_string(signed_input).expect("Cannot encode input"),
                )
                .await?;
        } else {
            return Err(anyhow!("Cannot get signed input"));
        }
    }

    let completed = check_tx_completed(Data::clone(&repo), room_id).await;
    match completed {
        Ok(tx) => {
            let tx_hex = consensus::encode::serialize_hex(&tx);
            println!("TX: {:#?}", tx);
            println!("TX completed: {}", tx_hex);
            Ok(true)
        }
        Err(e) => {
            println!("Check completed got error: {}", e);
            Ok(false)
        }
    }
}

pub async fn get_txn_hex(repo: Data<CoinJoinRepo>, room_id: &str) -> Result<String> {
    let txn = get_txn(repo, room_id).await?;
    Ok(consensus::encode::serialize_hex(&txn))
}

async fn get_txn(repo: Data<CoinJoinRepo>, room_id: &str) -> Result<bitcoin::Transaction> {
    let raw_inputs = repo.get_inputs(room_id).await?;
    let raw_outputs = repo.get_outputs(room_id).await?;

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

pub async fn get_room_by_addr(repo: Data<CoinJoinRepo>, addr: &str) -> Result<Vec<Room>> {
    let rooms = repo.get_room_by_addr(addr).await?;

    Ok(rooms)
}

pub async fn check_tx_completed(
    repo: Data<CoinJoinRepo>,
    room_id: &str,
) -> Result<bitcoin::Transaction, String> {
    let mut origin_tx = get_txn(Data::clone(&repo), room_id).await.unwrap();
    // TODO: check valid tx
    let proofs = repo.get_proofs(room_id).await.unwrap();
    if origin_tx.input.len() != proofs.len() {
        return Err("TX is not completed yet".to_string());
    }
    for (id, val) in origin_tx.input.iter_mut().enumerate() {
        let proof = proofs.get(id).unwrap();
        let txin = serde_json::from_str::<TxIn>(&proof.script).unwrap();
        if val.previous_output != txin.previous_output {
            return Err("Invalid Proofs".to_string());
        }
        // TODO: check valid signature
        *val = txin;
    }

    Ok(origin_tx)
}

// Function to encapsulate signature processing and error handling
fn validate_signature(
    _hex_sig: &str,
    _public_key: RistrettoPoint, // Assuming you have some PublicKey type
    _output_address: &str,
) -> Result<bool> {
    // let sig = WiredUnblindedSigData::try_from(hex_sig)?
    //     .to_internal_format()
    //     .map_err(|_| "Invalid signature type".to_string())?;
    //
    // if !sig.msg_authenticate::<sha3::Sha3_512, &[u8]>(public_key, output_address.as_bytes()) {
    //     return Ok(false);
    // }

    Ok(true)
}
