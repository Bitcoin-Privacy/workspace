use anyhow::Result;
use bitcoin::hex::Case;
use bitcoin::hex::DisplayHex;
use bitcoin::TxIn;
use shared::model::Txn;
use tokio::time::{sleep, Duration};

use shared::api;
use shared::blindsign::{BlindRequest, WiredUnblindedSigData};
use shared::model::Utxo;

use crate::api::{blindsign, coinjoin};
use crate::db::PoolWrapper;
use crate::model::{AccountActions, RoomEntity};
use crate::svc::account;

pub async fn register(
    pool: &PoolWrapper,
    deriv: &str,
    amount: u64,
    dest: &str,
) -> Result<(String, String)> {
    let acct = account::get_internal_account(deriv).expect("Account not found");
    let utxos = api::get_utxo(&acct.get_addr())
        .await
        .expect("Cannot get utxos");
    let utxo = utxos
        .iter()
        .find(|x: &&Utxo| x.value > amount)
        .expect(&format!(
            "Donot have compatible utxo {}, {:?}",
            amount, utxos
        ))
        .to_owned();

    let blind_session = blindsign::get_blindsign_session()
        .await
        .expect("Cannot get blindsign session");

    let rp: [u8; 32] = hex::decode(blind_session.rp)
        .expect("Cannot parse blindsign session")
        .try_into()
        .expect("Invalid size");
    let (blinded_address, unblinder) =
        BlindRequest::new_specific_msg::<sha3::Sha3_512, &[u8]>(&rp, dest.as_bytes()).unwrap();

    let register_res = coinjoin::register(
        vec![utxo],
        &hex::encode(blinded_address),
        &acct.get_addr(),
        amount,
    )
    .await?;

    let signed_msg: [u8; 32] = hex::decode(&register_res.signed_blined_output)
        .expect("Invalid sig")
        .try_into()
        .expect("Invalid size");

    let unblinded_sig = unblinder
        .gen_signed_msg(&signed_msg)
        .expect("Cannot unblind the sig");
    let wired = WiredUnblindedSigData::from(unblinded_sig);
    let sig = wired.as_bytes().to_hex_string(Case::Lower);

    let room_entity: RoomEntity = register_res.clone().into();

    if let Err(e) = pool.add_or_update_room(deriv, &room_entity) {
        panic!("Failed to update room {:?}", e);
    }

    let (room_id, address, sig_cloned) =
        (register_res.room.id.clone(), dest.to_string(), sig.clone());

    tokio::spawn(async move {
        // Generate a random number of seconds
        let random_delay = rand::random::<u64>() % 60; // for example, 0 to 59 seconds
        sleep(Duration::from_secs(random_delay)).await;

        if let Err(e) = coinjoin::set_output(&room_id, &address, &sig_cloned).await {
            println!("Set output got error {}", e);
            // tauri::Window::emit(
            //     &window,
            //     "coinjoin-register-complete",
            //     Some(event::CoinJoinRegisterCompleteEvent { room_id, status: 0 }),
            // )
            // .expect("Failed to emit event");
        } else {
            // tauri::Window::emit(
            //     &window,
            //     "coinjoin-register-complete",
            //     Some(event::CoinJoinRegisterCompleteEvent { room_id, status: 1 }),
            // )
            // .expect("Failed to emit event");
        }
    });

    Ok((register_res.room.id, sig))
}

pub async fn find_and_join_txn(index: usize, input: TxIn) -> Result<(usize, TxIn, Txn), String> {
    match api::get_onchain_tx(&input.previous_output.txid.to_string()).await {
        Ok(tx) => Ok((index, input, tx)),
        Err(e) => Err(format!("Failed to get transaction for input {}", e)),
    }
}
