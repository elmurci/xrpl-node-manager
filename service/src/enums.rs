use std::str::FromStr;

use serde::{Serialize, Deserialize};

#[derive(PartialEq, Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Topic {
    Stop,
    Start,
    Ledger,
    Peers,
    Status,
    Stats,
    Config,
    Features,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ResponseStatus {
    Error,
    Success,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum EventType {
    HELLO,
    SUBSCRIBE,
    UNSUBSCRIBE,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Http,
    Https,
    Peer,
    Ws,
    Wss,
}

impl FromStr for Protocol {

    type Err = ();

    fn from_str(input: &str) -> Result<Protocol, Self::Err> {
        match input {
            "http"  => Ok(Protocol::Http),
            "https"  => Ok(Protocol::Https),
            "peer"  => Ok(Protocol::Peer),
            "ws"  => Ok(Protocol::Ws),
            "wss"  => Ok(Protocol::Wss),
            _      => Err(()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseType {
    NuDB,
    RocksDB,
}

impl FromStr for DatabaseType {

    type Err = ();

    fn from_str(input: &str) -> Result<DatabaseType, Self::Err> {
        match input {
            "NuDB"  => Ok(DatabaseType::NuDB),
            "RocksDB"  => Ok(DatabaseType::RocksDB),
            _      => Err(()),
        }
    }
}