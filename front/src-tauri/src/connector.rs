use failure_derive::Fail;
use serde_json::Value;
use shared::model::resp::{ApiResponse, Status};

use crate::cfg::NODE_SERVICE_BASE_URL;

pub struct NodeConnector {
    base_url: String,
}

#[derive(Fail, Debug)]
pub enum NodeConnectorError {
    #[fail(display = "failed to request to node")]
    RequestFailed,
    #[fail(display = "failed to parse response to JSON")]
    ParseResponseFailed,
}

pub type NodeConnectorResult<T> = ::std::result::Result<T, NodeConnectorError>;

impl NodeConnector {
    pub fn new() -> Self {
        Self {
            base_url: NODE_SERVICE_BASE_URL.to_string(),
        }
    }
    pub fn get_url(&self, endpoint: &str) -> String {
        format!("{}/{}", self.base_url, endpoint)
    }
    pub async fn get(&self, endpoint: String, body: &Value) -> NodeConnectorResult<Value> {
        let client = reqwest::Client::new();
        let builder = client
            .get(self.get_url(&endpoint))
            .json(&body)
            .header("content-type", "application/json");
        let res = builder.send().await;
        match res {
            Ok(response) => {
                let parsed_response = response.json::<ApiResponse<Value>>().await;
                match parsed_response {
                    Ok(body_content) => {
                        println!("[Node] ({}): {:#?}", endpoint, body_content);
                        match body_content.status {
                            Status::Success => Ok(body_content.data.unwrap_or(Value::Null)),
                            Status::BadRequest => {
                                println!(
                                    "[Node] Get Error Message ({}): {:#?}",
                                    endpoint, body_content.message
                                );
                                Err(NodeConnectorError::RequestFailed)
                            }
                        }
                    }
                    Err(e) => {
                        println!("Failed to read response body: {}", e);
                        Err(NodeConnectorError::ParseResponseFailed)
                    }
                }
            }
            Err(e) => {
                println!("Failed to send request: {}", e);
                Err(NodeConnectorError::RequestFailed)
            }
        }
    }

    pub async fn post(&self, endpoint: &str, body: &Value) -> NodeConnectorResult<Value> {
        let client = reqwest::Client::new();
        let builder = client
            .post(self.get_url(&endpoint))
            .json(&body)
            .header("content-type", "application/json");
        let res = builder.send().await;
        match res {
            Ok(response) => {
                let parsed_response = response.json::<ApiResponse<Value>>().await;
                match parsed_response {
                    Ok(body_content) => {
                        println!("[Node] ({}): {:#?}", endpoint, body_content);
                        match body_content.status {
                            Status::Success => Ok(body_content.data.unwrap_or(Value::Null)),
                            Status::BadRequest => {
                                println!(
                                    "[Node] Get Error Message ({}): {:#?}",
                                    endpoint, body_content.message
                                );
                                Err(NodeConnectorError::RequestFailed)
                            }
                        }
                    }
                    Err(e) => {
                        println!("Failed to read response body: {}", e);
                        Err(NodeConnectorError::ParseResponseFailed)
                    }
                }
            }
            Err(e) => {
                println!("Failed to send request: {}", e);
                Err(NodeConnectorError::RequestFailed)
            }
        }
    }
}
