use anyhow::Result;
use shared::blindsign::BlindRequest;

use crate::{api::blindsign, connector::NodeConnector};

pub async fn blind_message(
    conn: &NodeConnector,
    address: &str,
) -> Result<([u8; 32], BlindRequest)> {
    let blind_session = blindsign::get_blindsign_session(conn).await?;
    let rp: [u8; 32] = hex::decode(blind_session.rp)?
        .try_into()
        .expect("Invalid size");
    Ok(BlindRequest::new_specific_msg::<sha3::Sha3_512, &[u8]>(
        &rp,
        address.as_bytes(),
    )?)
}
