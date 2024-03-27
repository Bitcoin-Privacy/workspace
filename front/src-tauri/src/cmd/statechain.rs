use crate::connector::NodeConnector;
use shared::intf::statechain::{DepositReq, DepositRes};
use tauri::State;

#[tauri::command]
pub async fn deposit(
    conn: State<'_, NodeConnector>,
    deriv: &str,
    amount: u64,
) -> Result<(), String> {
    let req = DepositReq {
        token_id: "abc".to_string(),
        addr: "hello".to_string(),
        amount: amount as u32,
    };
    let body = serde_json::to_value(req).unwrap();
    let res = conn.post("statechain/deposit", &body).await;
    match res {
        Ok(value) => {
            let json: DepositRes = serde_json::from_value(value).unwrap();
            println!("Deposit {:#?}", json);
        }
        Err(e) => {
            println!("Deposit Error {:#?}", e);
        }
    }
    Ok(())
}
