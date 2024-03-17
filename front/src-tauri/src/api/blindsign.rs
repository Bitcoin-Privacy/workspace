use shared::intf::blindsign::GetBlindSessionRes;

use crate::connector::NodeConnector;

pub struct BlindsignApis {}

impl BlindsignApis {
    pub async fn get_blindsign_session() -> Result<GetBlindSessionRes, String> {
        let conn = NodeConnector::new();

        let res = conn.get("blindsign/session".to_string(), None).await;
        match res {
            Ok(value) => {
                serde_json::from_value::<GetBlindSessionRes>(value).map_err(|e| e.to_string())
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
