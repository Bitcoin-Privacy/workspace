use shared::blindsign::{BlindRequest, WiredUnblindedSigData};
use shared::intf::coinjoin::{GetStatusRes, GetUnsignedTxnRes, SetOutputRes};
use shared::{
    api,
    model::{Txn, Utxo},
};

use bitcoin::{
    consensus,
    hex::{Case, DisplayHex},
    secp256k1::{Message, Secp256k1, SecretKey},
    sighash::SighashCache,
    Amount, EcdsaSighashType, ScriptBuf, Transaction, TxIn, Witness,
};
use core::panic;
use tauri::State;
use tokio::time::{sleep, Duration};
use wallet::core::{MasterAccount, Unlocker};

use crate::api::blindsign::BlindsignApis;
use crate::{
    api::coinjoin::CoinjoinApis,
    cfg::PASSPHRASE,
    db::PoolWrapper,
    model::{event, AccountActions, RoomEntity},
    store::master_account::get_master,
    svc::utxo,
};

use super::account::parse_derivation_path;

/// Register to CoinJoin Protocol
#[tauri::command]
pub async fn register(
    state: State<'_, PoolWrapper>,
    window: tauri::Window,
    deriv: &str,
    address: String,
    amount: u64,
) -> Result<(), String> {
    let source = super::account::get_account(deriv).expect("Account not found");

    let utxos = utxo::get_utxo(source.get_addr())
        .await
        .expect("Cannot get utxos");
    let utxo = utxos
        .iter()
        .find(|x: &&Utxo| x.value > amount)
        .expect("Donot have compatible utxo")
        .to_owned();

    let blind_session = BlindsignApis::get_blindsign_session()
        .await
        .expect("Cannot get blindsign session");
    let rp: [u8; 32] = hex::decode(blind_session.rp)
        .expect("Cannot parse blindsign session")
        .try_into()
        .expect("Invalid size");
    let (blinded_address, unblinder) =
        BlindRequest::new_specific_msg::<sha3::Sha3_512, &[u8]>(&rp, address.as_bytes()).unwrap();

    // NOTE: register to Server
    let register_res = CoinjoinApis::register(
        vec![utxo],
        &hex::encode(blinded_address),
        &source.get_addr(),
        amount,
    )
    .await?;
    let room_entity: RoomEntity = register_res.clone().into();

    if let Err(e) = state.add_or_update_room(deriv, &room_entity) {
        panic!("Failed to update room {:?}", e);
    }

    let signed_msg: [u8; 32] = hex::decode(&register_res.signed_blined_output)
        .expect("Invalid sig")
        .try_into()
        .expect("Invalid size");

    // Generate a random number of seconds
    let random_delay = rand::random::<u64>() % 60; // for example, 0 to 59 seconds

    // NOTE: set the output to the server
    tokio::spawn(async move {
        let unblinded_sig = unblinder
            .gen_signed_msg(&signed_msg)
            .expect("Cannot unblind the sig");
        let wired = WiredUnblindedSigData::from(unblinded_sig);
        let sig = wired.as_bytes().to_hex_string(Case::Lower);

        sleep(Duration::from_secs(random_delay)).await;
        if let Err(e) = CoinjoinApis::set_output(&register_res.room.id, &address, &sig).await {
            println!("Set output got error {}", e);
            tauri::Window::emit(
                &window,
                "coinjoin-register-complete",
                Some(event::CoinJoinRegisterCompleteEvent {
                    room_id: register_res.room.id,
                    status: 0,
                }),
            )
            .expect("Failed to emit event");
        } else {
            tauri::Window::emit(
                &window,
                "coinjoin-register-complete",
                Some(event::CoinJoinRegisterCompleteEvent {
                    room_id: register_res.room.id,
                    status: 1,
                }),
            )
            .expect("Failed to emit event");
        }
    });

    Ok(())
}

#[tauri::command]
pub async fn get_rooms(
    state: State<'_, PoolWrapper>,
    deriv: &str,
) -> Result<Vec<RoomEntity>, String> {
    state
        .get_all_rooms(deriv)
        .map_err(|e| format!("Error: {:?}", e))
}

//------------------------

async fn dosth(index: usize, input: TxIn) -> Result<(usize, TxIn, Txn), String> {
    match api::get_onchain_tx(&input.previous_output.txid.to_string()).await {
        Ok(tx) => Ok((index, input, tx)),
        Err(e) => Err(format!("Failed to get transaction for input {}", e)),
    }
}

#[tauri::command]
pub async fn sign_tx(
    state: State<'_, PoolWrapper>,
    deriv: &str,
    room_id: &str,
) -> Result<(), String> {
    let master_account: MasterAccount = get_master().expect("Master account does not exist");
    let parsed_path = parse_derivation_path(deriv).map_err(|e| e.to_string())?;
    let account = master_account.accounts().get(&parsed_path).unwrap();

    let res = CoinjoinApis::get_transaction(&room_id).await.unwrap();
    let parsed_tx =
        consensus::deserialize::<Transaction>(&hex::decode(&res.tx.clone()).unwrap()).unwrap();

    let room = state
        .get_room(deriv, &room_id)
        .map_err(|e| format!("Error: {:?}", e))
        .unwrap();

    // TODO:
    // - find inputs which is from this account
    // - sign all these inputs
    let mut unsigned_tx = parsed_tx.clone();

    let sighash_type = EcdsaSighashType::All;
    let mut sighasher = SighashCache::new(&mut unsigned_tx);

    let future_tasks: Vec<_> = parsed_tx
        .input
        .iter()
        .enumerate()
        .filter(|(_, input)| {
            room.utxos
                .iter()
                .find(|utxo| input.previous_output.txid.to_string() == utxo.txid.to_string())
                .is_some()
        })
        .map(|(index, input)| tokio::spawn(dosth(index, input.clone())))
        .collect();
    let mut results = Vec::new();
    for job in future_tasks {
        results.push(job.await.unwrap().unwrap());
    }

    let mut unlocker = Unlocker::new_for_master(&master_account, PASSPHRASE).unwrap();
    let secp = Secp256k1::new();

    let vins: Vec<u16> = results
        .iter()
        .map(|(index, input, tx)| {
            let vout = tx
                .vout
                .get(input.previous_output.vout as usize)
                .expect("Cannot get the vout");
            let amount = Amount::from_sat(vout.value);
            println!(
                "Script code 1: {}",
                ScriptBuf::from_hex(&vout.scriptpubkey).unwrap()
            );
            let script_pubkey =
                ScriptBuf::from_hex(&vout.scriptpubkey).expect("Invalid script public key");

            let sighash = sighasher
                .p2wpkh_signature_hash(*index, &script_pubkey, amount, sighash_type)
                .expect("failed to create sighash");

            let priv_key = account
                .get_privkey(script_pubkey.clone(), &mut unlocker)
                .expect("Cannot get private key");
            // input.script_sig = ScriptBuf::new();
            let msg = Message::from(sighash);
            let sk = SecretKey::from_slice(&priv_key.to_bytes()).unwrap();

            let sig = secp.sign_ecdsa(&msg, &sk);

            // Update the witness stack.
            let signature = bitcoin::ecdsa::Signature {
                sig,
                hash_ty: EcdsaSighashType::All,
            };

            let pk = sk.public_key(&secp);
            *sighasher.witness_mut(*index).unwrap() = Witness::p2wpkh(&signature, &pk);
            *index as u16
        })
        .collect();
    let tx_hex = consensus::encode::serialize_hex(&unsigned_tx);
    println!("hash: {:?}", tx_hex);
    println!("{:#?}", unsigned_tx);

    let res = CoinjoinApis::sign(room_id, vins, &tx_hex).await;
    match res {
        Ok(response) => {
            println!("RES {:#?}", response);
            Ok(())
        }
        Err(e) => Err(format!("Error: {}", e)),
    }
}

#[tauri::command]
pub async fn get_tx(room_id: &str) -> Result<GetUnsignedTxnRes, String> {
    let res = CoinjoinApis::get_transaction(room_id).await;
    match res {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error: {}", e)),
    }
}

#[tauri::command]
pub async fn get_status(room_id: &str) -> Result<GetStatusRes, String> {
    let res = CoinjoinApis::get_status(room_id).await;
    match res {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error: {}", e)),
    }
}

pub async fn set_out(
    room_id: &str,
    output_address: &str,
    sig: &str,
) -> Result<SetOutputRes, String> {
    let res = CoinjoinApis::set_output(room_id, output_address, sig).await;

    match res {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error: {}", e)),
    }
}
