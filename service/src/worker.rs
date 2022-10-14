use std::sync::Arc;

use crate::handler::Handler;
use crate::structs::OutgoingMessage;
use crate::{TOPICS, CLIENTS};
use crate::enums::{Topic, EventType, ResponseStatus};
use tokio;
use tokio::time::Duration;
use tracing::{debug, error};

pub struct Worker {
    refresh: u64,
    handler: Arc<Handler>,
  }
  
impl Worker {
    pub(crate) fn new(handler: Arc<Handler>, refresh: u64) -> Worker {
        Worker {
            refresh,
            handler
        }
    }

    pub async fn start(&self, refresh: u64) {
        loop {
            tokio::time::sleep(Duration::from_millis(refresh)).await;
            let connected_client_count = CLIENTS.lock().await.len();
            if connected_client_count == 0 {
                debug!("No clients connected, skip sending data");
                continue;
            }
            debug!("{} connected client(s)",connected_client_count);
            // For each topic
            //     send to subscribed clients
            debug!("topics {:?}", TOPICS.lock().await.len());
            // TODO: implement loop over topics
            let msg = match self.handler.node.info().await {
                Ok(result) => OutgoingMessage {
                    event: EventType::FEED,
                    topic: Topic::Status,
                    status: ResponseStatus::Success,
                    message: result
                },
                Err(err) => OutgoingMessage {
                    event: EventType::FEED,
                    topic: Topic::Status,
                    status: ResponseStatus::Error,
                    message: serde_json::Value::String(err.to_string())
                }
            };
            match self.handler.update(Topic::Status, msg).await {
                Ok(_) => debug!("Clients have been sent info"),
                Err(err) => {
                    error!("Error updating {:?}", err);
                }
            }
            let msg = match self.handler.node.features().await {
                Ok(result) => OutgoingMessage {
                    event: EventType::FEED,
                    topic: Topic::Features,
                    status: ResponseStatus::Success,
                    message: result
                },
                Err(err) => OutgoingMessage {
                    event: EventType::FEED,
                    topic: Topic::Features,
                    status: ResponseStatus::Error,
                    message: serde_json::Value::String(err.to_string())
                }
            };
            match self.handler.update(Topic::Status, msg).await {
                Ok(_) => debug!("Clients have been sent info"),
                Err(err) => {
                    error!("Error updating {:?}", err);
                }
            }
        }
    }
}