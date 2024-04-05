use crate::CFG;

pub fn get_session() -> (String, String) {
    let session = CFG.blind_session;
    let keypair = CFG.blind_keypair;

    (
        hex::encode(keypair.public().compress().to_bytes()),
        hex::encode(session.get_rp()),
    )
}

pub fn blind_sign(msg: &str) -> Result<String, String> {
    let msg: [u8; 32] = hex::decode(msg)
        .map_err(|e: hex::FromHexError| e.to_string())?
        .try_into()
        .map_err(|e: Vec<u8>| format!("Invalid length: {:#?}", e))?;

    let session = CFG.blind_session;
    match session.sign_ep(&msg, CFG.blind_keypair.private()) {
        Ok(signed_blind_output) => Ok(hex::encode(signed_blind_output)),
        Err(e) => Err(e.to_string()), // Assuming e can be converted to String
    }
}
