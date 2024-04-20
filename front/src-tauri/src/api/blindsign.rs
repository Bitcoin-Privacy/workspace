use anyhow::Result;
use shared::intf::blindsign::GetBlindSessionRes;

use crate::connector::NodeConnector;

pub async fn get_blindsign_session(conn: &NodeConnector) -> Result<GetBlindSessionRes> {
    let res = conn.get("blindsign/session", None).await?;
    Ok(serde_json::from_value::<GetBlindSessionRes>(res)?)
}
