use std::collections::HashMap;

use fehler::throws;
use anyhow::{Error, bail};
use serde_json::Value;
use tracing::debug;
use async_process::Command;

use crate::{rpc_client::RpcClient, lib::file::read_cfg, structs::NodeConfig};

#[derive(Clone)]
pub struct Node {
    endpoint: String,
    rpc_client: RpcClient,
    config_path: String,
}
  
impl Node {
    pub(crate) fn new<E: Into<String>, C: Into<String>>(endpoint: E, config_path: C) -> Node {
        let endpoint = endpoint.into();
        let rpc_client = RpcClient::new(&endpoint);
        Node {
            endpoint,
            rpc_client,
            config_path: config_path.into(),
        }
    }

    #[throws(_)]
    pub(crate) async fn stop(&self) -> Value {
        debug!("Stop call to {}", self.endpoint);
        match self.rpc_client.run("stop", None).await {
            Ok(response) => response,
            Err(error) => bail!(error)
        }
    }

    #[throws(_)]
    pub(crate) async fn start(&self) -> Value {
        debug!("Start command");
        let output = Command::new("screen").args(&["-d", "-m", "/Users/javiromero/Documents/dev/xrpl/rippled/my_build/rippled"]).spawn()?.output().await?;
        // TODO: command
        if output.status.success() {
            let r = String::from_utf8_lossy(&output.stdout);
            debug!("Start command result {:?}", r);
            self.info().await.unwrap()
        } else {
            debug!("Start command err {:?}", &output);
            let err = String::from_utf8_lossy(&output.stderr).to_string();
            bail!(err)
        }
    }
    
    #[throws(_)]
    pub(crate) async fn features(&self) -> Value {
        debug!("Features call to {}", self.endpoint);
        match self.rpc_client.run("feature", None).await {
            Ok(response) => response.get("result").unwrap().to_owned(),
            Err(error) => bail!(error)
        }
    }

    #[throws(_)]
    pub(crate) async fn config(&self) -> Value {
        debug!("Config request");
        let result: NodeConfig = NodeConfig::from(read_cfg(&self.config_path).await?);
        serde_json::to_value(result).unwrap()
    }

    #[throws(_)]
    pub(crate) async fn stats(&self) -> Value {
        debug!("Stats call to {}", self.endpoint);
        let mut params = HashMap::new();
        params.insert(String::from("min_count"), String::from("100"));
        match self.rpc_client.run("get_counts", Some(vec!(params))).await {
            Ok(response) => response.get("result").unwrap().to_owned(),
            Err(error) => bail!(error)
        }
    }

    #[throws(_)]
    pub(crate) async fn restart(&self) {
        debug!("Restart command");
        self.stop().await?;
        self.start().await?;
    }

    #[throws(_)]
    pub(crate) async fn info(&self) -> Value {
        debug!("Server info call to {}", self.endpoint);
        match self.rpc_client.run("server_info", None).await {
            Ok(response) => response.get("result").unwrap().to_owned(),
            Err(error) => bail!(error)
        }
    }
}
