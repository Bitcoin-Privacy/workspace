use anyhow::Result;
use shared::intf::blindsign::GetBlindSessionRes;

use crate::connector::NodeConnector;

pub async fn get_blindsign_session() -> Result<GetBlindSessionRes> {
    let conn = NodeConnector::new();
    let res = conn.get("blindsign/session".to_string(), None).await?;
    Ok(serde_json::from_value::<GetBlindSessionRes>(res)?)
}
