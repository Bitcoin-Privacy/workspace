use anyhow::{anyhow, Result};
use shared::blindsign::WiredUnblindedSigData;

use crate::CFG;

pub fn get_session() -> (String, String) {
    let session = CFG.blind_session;
    let keypair = CFG.blind_keypair;

    (
        hex::encode(keypair.public().compress().to_bytes()),
        hex::encode(session.get_rp()),
    )
}

pub fn blind_sign(msg: &str) -> Result<String> {
    let msg: [u8; 32] = hex::decode(msg)?
        .try_into()
        .map_err(|e: Vec<u8>| anyhow!("Invalid length: {:#?}", e))?;

    let session = CFG.blind_session;
    match session.sign_ep(&msg, CFG.blind_keypair.private()) {
        Ok(signed_blind_output) => Ok(hex::encode(signed_blind_output)),
        Err(e) => Err(anyhow!(e.to_string())), // Assuming e can be converted to String
    }
}

pub fn msg_authenticate(hex_sig: &str, msg: &str) -> Result<bool> {
    let keypair = CFG.blind_keypair;
    let sig = WiredUnblindedSigData::try_from(hex_sig)
        .map_err(|e| anyhow!(e))?
        .to_internal_format()
        .map_err(|_| anyhow!("Invalid signature type"))?;

    if !sig.msg_authenticate::<sha3::Sha3_512, &[u8]>(keypair.public(), msg.as_bytes()) {
        Ok(false)
    } else {
        Ok(true)
    }
}

pub fn authenticate(hex_sig: &str) -> Result<bool> {
    let keypair = CFG.blind_keypair;
    let sig = WiredUnblindedSigData::try_from(hex_sig)
        .map_err(|e| anyhow!(e))?
        .to_internal_format()
        .map_err(|_| anyhow!("Invalid signature type"))?;

    if !sig.authenticate(keypair.public()) {
        Ok(false)
    } else {
        Ok(true)
    }
}
