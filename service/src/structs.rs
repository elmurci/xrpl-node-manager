use std::{collections::HashMap, str::FromStr};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{enums::{Topic, ResponseStatus, Protocol, DatabaseType, EventType}, lib::util::{get_bool_from_binary, get_vec_from_hashmap}};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct IncomingMessage {
    pub topic: Topic
}

#[derive(Debug, Serialize)]
pub struct OutgoingMessage {
    pub event: EventType,
    pub topic: Topic,
    pub status: ResponseStatus,
    pub message: Value
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OutgoingRpcMessage<'a> {
    pub method: &'a str,
    pub params: Option<Vec<HashMap<String, String>>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeConfig {
    pub validators_file: String,
    pub port_peer: Port,
    pub rpc_startup: String,
    pub server: Vec<String>,
    pub debug_logfile: String,
    pub sntp_servers: Vec<String>,
    pub port_rpc_admin_local: Port,
    pub node_db: Database,
    pub database_path: String,
    pub port_ws_admin_local: Port,
}

impl From<HashMap<String,HashMap<String,String>>> for NodeConfig {
    fn from(item: HashMap<String,HashMap<String,String>>) -> Self {
        // TODO: error handling
        NodeConfig {
            validators_file: item.get("validators_file").unwrap().get("0").unwrap().to_string(),
            port_peer: Port {
                port: item.get("port_peer").unwrap().get("port").unwrap().to_string().parse::<u16>().unwrap(),
                ip: item.get("port_peer").unwrap().get("ip").unwrap().to_string(),
                protocol: Protocol::from_str(&item.get("port_peer").unwrap().get("protocol").unwrap().to_string()).unwrap(),
                admin: None
            },
            rpc_startup: item.get("rpc_startup").unwrap().get("0").unwrap().to_string(),
            server: get_vec_from_hashmap(item.get("server").unwrap()),
            debug_logfile: item.get("debug_logfile").unwrap().get("0").unwrap().to_string(),
            sntp_servers: get_vec_from_hashmap(item.get("sntp_servers").unwrap()),
            port_rpc_admin_local: Port {
                port: item.get("port_rpc_admin_local").unwrap().get("port").unwrap().to_string().parse::<u16>().unwrap(),
                ip: item.get("port_rpc_admin_local").unwrap().get("ip").unwrap().to_string(),
                protocol: Protocol::from_str(&item.get("port_rpc_admin_local").unwrap().get("protocol").unwrap().to_string()).unwrap(),
                admin: Some(item.get("port_rpc_admin_local").unwrap().get("admin").unwrap().to_string())
            },
            node_db: Database {
                r#type: DatabaseType::from_str(&item.get("node_db").unwrap().get("type").unwrap().to_string()).unwrap(),
                path: item.get("node_db").unwrap().get("path").unwrap().to_string(),
                online_delete: item.get("node_db").unwrap().get("online_delete").unwrap().parse::<u16>().unwrap(),
                advisory_delete: get_bool_from_binary(&item.get("node_db").unwrap().get("advisory_delete").unwrap().to_string())
            },
            database_path: item.get("database_path").unwrap().get("0").unwrap().to_string(),
            port_ws_admin_local: Port {
                port: item.get("port_ws_admin_local").unwrap().get("port").unwrap().to_string().parse::<u16>().unwrap(),
                ip: item.get("port_ws_admin_local").unwrap().get("ip").unwrap().to_string(),
                protocol: Protocol::from_str(&item.get("port_ws_admin_local").unwrap().get("protocol").unwrap().to_string()).unwrap(),
                admin: Some(item.get("port_ws_admin_local").unwrap().get("admin").unwrap().to_string())
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Port {
    pub port: u16,
    pub ip: String,
    pub protocol: Protocol,
    pub admin: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub r#type: DatabaseType,
    pub path: String,
    pub online_delete: u16,
    pub advisory_delete: bool
}
