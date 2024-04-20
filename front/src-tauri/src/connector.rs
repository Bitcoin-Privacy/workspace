use serde_json::{Map, Value};
use shared::model::resp::{ApiResponse, Status};
use thiserror::Error;

pub struct NodeConnector {
    base_url: String,
}

#[derive(Error, Debug)]
pub enum NodeConnErr {
    #[error("failed to request to node")]
    RequestFailed(String),
    #[error("failed to parse response to JSON")]
    ParseResponseFailed(String),
}

impl From<reqwest::Error> for NodeConnErr {
    fn from(err: reqwest::Error) -> Self {
        println!("Failed to send request: {}", err);
        NodeConnErr::RequestFailed(err.to_string())
    }
}

pub type NodeConnectorResult<T> = ::std::result::Result<T, NodeConnErr>;

impl NodeConnector {
    pub fn new(url: String) -> Self {
        Self { base_url: url }
    }
    pub fn get_url(&self, endpoint: &str) -> String {
        format!("{}/{}", self.base_url, endpoint)
    }
    pub async fn get(
        &self,
        endpoint: &str,
        params: Option<Map<String, Value>>,
    ) -> NodeConnectorResult<Value> {
        let client = reqwest::Client::new();
        let url = self.get_url(endpoint);

        let res = client
            .get(&url)
            .query(&params.unwrap_or_default())
            .send()
            .await?;

        match res.json::<ApiResponse<Value>>().await {
            Ok(parsed_res) => {
                println!("[Node] ({}): {:#?}", endpoint, parsed_res);
                match parsed_res.status {
                    Status::Success => Ok(parsed_res.data.unwrap_or(Value::Null)),
                    Status::BadRequest => {
                        println!(
                            "[Node] Get Error Message ({}): {:#?}",
                            endpoint, parsed_res.message
                        );
                        Err(NodeConnErr::RequestFailed(format!(
                            "{} ({:?}): {:?}",
                            endpoint, parsed_res.status, parsed_res.message
                        )))
                    }
                }
            }
            Err(e) => {
                println!("Failed to read response body: {}", e);
                Err(NodeConnErr::ParseResponseFailed(e.to_string()))
            }
        }
    }

    pub async fn post(&self, endpoint: &str, body: &Value) -> NodeConnectorResult<Value> {
        let client = reqwest::Client::new();
        let res = client
            .post(self.get_url(endpoint))
            .json(&body)
            .header("content-type", "application/json")
            .send()
            .await?;
        match res.json::<ApiResponse<Value>>().await {
            Ok(parsed_res) => {
                println!("[Node] ({}): {:#?}", endpoint, parsed_res);
                match parsed_res.status {
                    Status::Success => Ok(parsed_res.data.unwrap_or(Value::Null)),
                    Status::BadRequest => {
                        println!(
                            "[Node] Get Error Message ({}): {:#?}",
                            endpoint, parsed_res.message
                        );
                        Err(NodeConnErr::RequestFailed(format!(
                            "{} ({:?}): {:?}",
                            endpoint, parsed_res.status, parsed_res.message
                        )))
                    }
                }
            }
            Err(e) => {
                println!("Failed to read response body: {}", e);
                Err(NodeConnErr::ParseResponseFailed(e.to_string()))
            }
        }
    }
}
