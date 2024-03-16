use crate::api::blindsign::BlindsignApis;
use crate::api::coinjoin::CoinjoinApis;
use crate::db::PoolWrapper;
use crate::model::{AccountActions, RoomEntity};
use crate::svc::account;
use bitcoin::hex::Case;
use bitcoin::hex::DisplayHex;
use shared::api;
use shared::blindsign::{BlindRequest, WiredUnblindedSigData};
use shared::model::Utxo;
use tauri::State;

pub async fn register(
    state: State<'_, PoolWrapper>,
    deriv: &str,
    amount: u64,
    dest: &str,
) -> Result<(String, String), String> {
    let acct = account::get_internal_account(deriv).expect("Account not found");
    let utxos = api::get_utxo(&acct.get_addr())
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
        BlindRequest::new_specific_msg::<sha3::Sha3_512, &[u8]>(&rp, dest.as_bytes()).unwrap();

    let register_res = CoinjoinApis::register(
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

    if let Err(e) = state.add_or_update_room(deriv, &room_entity) {
        panic!("Failed to update room {:?}", e);
    }

    Ok((register_res.room.id, sig))
}
