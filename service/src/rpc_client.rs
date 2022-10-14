use std::collections::HashMap;

use fehler::throws;
use anyhow::{Error, bail};
use reqwest::StatusCode;
use serde_json::Value;
use tracing::debug;

use crate::structs::OutgoingRpcMessage;

#[derive(Clone)]
pub struct RpcClient {
    endpoint: String,
}
  
impl RpcClient {
    pub(crate) fn new<T: Into<String>>(endpoint: T) -> RpcClient {
        RpcClient {
            endpoint: endpoint.into(),
        }
    }
    #[throws(_)]
    pub(crate) async fn run(&self, method: &str, params: Option<Vec<HashMap<String, String>>>) -> Value {
        let request = OutgoingRpcMessage {
            method,
            params,
        };
        let client = reqwest::Client::new();
        let response = client.post(&self.endpoint)
            .json(&request)
            .send()
            .await?;
        debug!("Run response {:?} {}", response, response.status());
        match response.status() {
            StatusCode::OK => response.json::<Value>().await?,
            _ => bail!("Error in RPC response"),
        }
    }
}
  
