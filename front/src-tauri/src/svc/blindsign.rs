use shared::blindsign::BlindRequest;

use crate::api::blindsign;

pub async fn blind_message(address: &str) -> Result<([u8; 32], BlindRequest), String> {
    let blind_session = blindsign::get_blindsign_session()
        .await
        .expect("Cannot get blindsign session");
    let rp: [u8; 32] = hex::decode(blind_session.rp)
        .expect("Cannot parse blindsign session")
        .try_into()
        .expect("Invalid size");
    BlindRequest::new_specific_msg::<sha3::Sha3_512, &[u8]>(&rp, address.as_bytes())
        .map_err(|e| e.to_string())
}
