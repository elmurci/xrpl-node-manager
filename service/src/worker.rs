use std::sync::Arc;

use crate::enums::{EventType, ResponseStatus, Topic};
use crate::handler::update;
use crate::structs::OutgoingMessage;
use crate::{handler, Node, CLIENTS, TOPICS};
use tokio;
use tokio::time::Duration;
use tracing::{debug, error};

pub struct Worker {
  refresh: u64,
  node: Arc<Node>,
}

impl Worker {
  pub(crate) fn new(node: Arc<Node>, refresh: u64) -> Worker {
    Worker { refresh, node }
  }

  pub async fn start(&self, refresh: u64) {
    loop {
      tokio::time::sleep(Duration::from_millis(refresh)).await;
      let connected_client_count = CLIENTS.lock().await.len();
      if connected_client_count == 0 {
        debug!("No clients connected, skip sending data");
        continue;
      }
      debug!("{} connected client(s)", connected_client_count);
      // For each topic
      //     send to subscribed clients
      debug!("topics {:?}", TOPICS.lock().await.len());
      // TODO: implement loop over topics
      let msg = match self.node.info().await {
        Ok(result) => OutgoingMessage {
          event: EventType::FEED,
          topic: Topic::Status,
          status: ResponseStatus::Success,
          message: result,
        },
        Err(err) => OutgoingMessage {
          event: EventType::FEED,
          topic: Topic::Status,
          status: ResponseStatus::Error,
          message: serde_json::Value::String(err.to_string()),
        },
      };
      match handler::update(Topic::Status, msg).await {
        Ok(_) => debug!("Clients have been sent info"),
        Err(err) => {
          error!("Error updating {:?}", err);
        }
      }
      let msg = match self.node.features().await {
        Ok(result) => OutgoingMessage {
          event: EventType::FEED,
          topic: Topic::Features,
          status: ResponseStatus::Success,
          message: result,
        },
        Err(err) => OutgoingMessage {
          event: EventType::FEED,
          topic: Topic::Features,
          status: ResponseStatus::Error,
          message: serde_json::Value::String(err.to_string()),
        },
      };
      match handler::update(Topic::Status, msg).await {
        Ok(_) => debug!("Clients have been sent info"),
        Err(err) => {
          error!("Error updating {:?}", err);
        }
      }
    }
  }
}
