use std::collections::HashMap;

use fehler::throws;
use anyhow::{Error, bail};
use reqwest::StatusCode;
use serde_json::Value;
use tracing::{info, debug};

use crate::structs::OutgoingRpcMessage;

pub struct RpcClient<'a> {
    endpoint: &'a str,
}
  
impl<'a> RpcClient<'a> {
    pub(crate) fn new(endpoint: &'a str) -> RpcClient {
        RpcClient {
            endpoint
        }
    }
    #[throws(_)]
    pub(crate) async fn run(&self, method: &str, params: Option<Vec<HashMap<String, String>>>) -> Value {
        let request = OutgoingRpcMessage {
            method,
            params,
        };
        let client = reqwest::Client::new();
        let response = client.post(self.endpoint)
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
  
