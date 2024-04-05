use anyhow::Result;
use shared::intf::statechain::{DepositReq, DepositRes};

use crate::{connector::NodeConnector, store::master_account::get_master};

use super::account;

pub async fn deposit(conn: &NodeConnector, deriv: &str, amount: u64) -> Result<()> {
    let acct = account::get_internal_account(deriv)?;
    let req = DepositReq {
        token_id: "abc".to_string(),
        addr: acct.master_public.to_string(),
        amount: amount as u32,
    };
    let body = serde_json::to_value(req)?;
    let res = conn.post("statechain/deposit", &body).await?;
    let json: DepositRes = serde_json::from_value(res)?;
    println!("Deposit {:#?}", json);
    Ok(())
}
