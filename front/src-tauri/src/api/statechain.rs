// use crate::{cfg::CFG, connector::NodeConnector};

// pub async fn deposit(conn: &NodeConnector, amount: u64) -> Result<DepositRes> {
//     let secp = Secp256k1::new();
//     let keypair = Keypair::new(&secp, &mut rand::thread_rng());
//     let xonly_pubkey = XOnlyPublicKey::from_keypair(&keypair);

//     let req = DepositReq {
//         token_id: "abc".to_string(),
//         addr: serde_json::to_string(&xonly_pubkey).unwrap(),
//         amount: amount as u32,
//     };
//     println!("Deposit {:#?}", req);
//     let body = serde_json::to_value(req)?;
//     let res = conn.post("statechain/deposit", &body).await?;
//     let json: DepositRes = serde_json::from_value(res)?;
//     println!("Deposit {:#?}", json);
//     Ok(json)
// }
