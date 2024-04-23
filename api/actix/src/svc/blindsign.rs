use crate::CFG;
use anyhow::{anyhow, Result};

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
    Ok(hex::encode(
        session.sign_ep(&msg, CFG.blind_keypair.private())?,
    ))
}
