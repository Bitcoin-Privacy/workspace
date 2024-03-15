use crate::api::blindsign::BlindsignApis;
use crate::api::coinjoin::CoinjoinApis;
use crate::model::{AccountActions, AccountDTO};
use shared::blindsign::BlindRequest;
use shared::model::Utxo;

use super::utxo;

pub async fn register(
    // state: State<'_, PoolWrapper>,
    acct: AccountDTO,
    amount: u64,
    dest: String,
) -> Result<(BlindRequest, [u8; 32]), String> {
    let utxos = utxo::get_utxo(acct.get_addr())
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
    // let room_entity: RoomEntity = register_res.clone().into();

    // if let Err(e) = state.add_or_update_room(deriv, &room_entity) {
    //     panic!("Failed to update room {:?}", e);
    // }

    let signed_msg: [u8; 32] = hex::decode(&register_res.signed_blined_output)
        .expect("Invalid sig")
        .try_into()
        .expect("Invalid size");

    Ok((unblinder, signed_msg))
}
