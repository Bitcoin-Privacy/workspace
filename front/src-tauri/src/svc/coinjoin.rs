use std::ops::ControlFlow;

use anyhow::{anyhow, Result};
use bitcoin::hex::{Case, DisplayHex};
use bitcoin::{
    consensus, secp256k1::Secp256k1, sighash::SighashCache, EcdsaSighashType, Transaction,
};
use shared::intf::coinjoin::{CheckSignatureRes, GetStatusRes, RoomDto};
use tokio::time::{sleep, Duration};

use shared::blindsign::WiredUnblindedSigData;

use crate::cfg::CFG;
use crate::connector::NodeConnector;
use crate::db::PoolWrapper;
use crate::model::{AccountActions, RoomEntity};
use crate::svc::account;
use crate::svc::blindsign;
use crate::{api::coinjoin, model::event};

pub async fn register(
    pool: &PoolWrapper,
    conn: &NodeConnector,
    window: tauri::Window,
    deriv: &str,
    amount: u64,
    dest: &str,
) -> Result<(String, String)> {
    let acct = account::get_internal_account(deriv)?;
    let utxos = acct.get_utxo(amount + CFG.coinjoin_fee).await?;

    let (blinded_address, unblinder) = blindsign::blind_message(conn, dest).await?;

    let register_res = coinjoin::register(
        conn,
        utxos,
        &hex::encode(blinded_address),
        &acct.get_addr(),
        amount,
    )
    .await?;

    let signed_msg: [u8; 32] = hex::decode(&register_res.signed_blined_output)?
        .try_into()
        .map_err(|e| anyhow!("Internal error: {e:?}"))?;

    let unblinded_sig = unblinder
        .gen_signed_msg(&signed_msg)
        .map_err(|e| anyhow!("Cannot unblind the sig: {e:?}"))?;
    let wired = WiredUnblindedSigData::from(unblinded_sig);
    let sig = wired.as_bytes().to_hex_string(Case::Lower);

    let room_entity: RoomEntity = register_res.clone().into();

    pool.add_or_update_room(deriv, &room_entity)
        .map_err(|e| anyhow!("Failed to update room {e:?}"))?;

    let (room_id, address, sig_cloned) =
        (register_res.room.id.clone(), dest.to_string(), sig.clone());

    tokio::spawn(async move {
        // Generate a random number of seconds
        let random_delay = rand::random::<u64>() % 60;
        sleep(Duration::from_secs(random_delay)).await;

        if let Err(e) = coinjoin::set_output(&room_id, &address, &sig_cloned).await {
            println!("Set output got error {e}");
            tauri::Window::emit(
                &window,
                "coinjoin-setoutput",
                Some(event::CoinJoinRegisterCompleteEvent { room_id, status: 0 }),
            )
            .expect("Failed to emit event");
        } else {
            tauri::Window::emit(
                &window,
                "coinjoin-setoutput",
                Some(event::CoinJoinRegisterCompleteEvent { room_id, status: 1 }),
            )
            .expect("Failed to emit event");
        }
    });

    Ok((register_res.room.id, sig))
}

pub async fn sign_txn(deriv: &str, room_id: &str) -> Result<()> {
    let (account, mut unlocker) = account::get_account(deriv)?;

    let res = coinjoin::get_txn(room_id).await?;
    let parsed_tx = consensus::deserialize::<Transaction>(&hex::decode(res.tx.clone())?)?;
    let mut unsigned_tx = parsed_tx.clone();

    let room_detail = coinjoin::get_room(&account.get_addr(), room_id).await?;

    let secp = Secp256k1::new();
    let sighash_type = EcdsaSighashType::All;
    let mut sighasher = SighashCache::new(&mut unsigned_tx);

    let future_tasks: Vec<_> = parsed_tx
        .input
        .iter()
        .enumerate()
        .filter(|(_, input)| {
            room_detail
                .utxo
                .iter()
                .any(|utxo| input.previous_output.txid.to_string() == utxo.txid)
        })
        .map(|(index, input)| tokio::spawn(account::find_and_join_txn(index, input.clone())))
        .collect();

    let mut results = Vec::new();
    for job in future_tasks {
        results.push(job.await??);
    }

    let mut vins: Vec<u16> = Vec::new();
    let res = results.iter().try_for_each(|(index, input, tx)| {
        vins.push(*index as u16);
        match account::sign(
            &secp,
            &mut sighasher,
            sighash_type,
            &account,
            &mut unlocker,
            index,
            input,
            tx,
        ) {
            Ok(_) => ControlFlow::Continue(()),
            Err(e) => ControlFlow::Break(e),
        }
    });
    if let ControlFlow::Break(e) = res {
        return Err(e);
    }

    let tx_hex = consensus::encode::serialize_hex(&unsigned_tx);

    let res = coinjoin::sign(room_id, &account.get_addr(), vins, &tx_hex).await;
    match res {
        Ok(response) => {
            println!("CoinJoin Sign transaction response: {response:#?}");
            Ok(())
        }
        Err(e) => Err(anyhow!("CoinJoin Sign transaction error: {e}")),
    }
}

pub async fn get_status(room_id: &str) -> Result<GetStatusRes> {
    crate::api::coinjoin::get_status(room_id).await
}

pub async fn get_signed(deriv: &str, room_id: &str) -> Result<CheckSignatureRes> {
    let acct = account::get_internal_account(deriv)?;
    crate::api::coinjoin::get_signed(&acct.get_addr(), room_id).await
}

pub async fn get_rooms(deriv: &str) -> Result<Vec<RoomDto>> {
    let acct = account::get_internal_account(deriv)?;
    crate::api::coinjoin::get_room_list(&acct.get_addr()).await
}
