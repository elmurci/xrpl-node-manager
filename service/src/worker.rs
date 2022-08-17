use crate::{TOPICS, CLIENTS};
use crate::node::Node;
use crate::enums::Topic;
use crate::handler::update;
use serde_json::json;
use tokio;
use tokio::time::Duration;
use tracing::{debug, error};

pub async fn main_worker(refresh: u64, endpoint: &str) {
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
        let node = Node::new(endpoint);
        let msg = match node.info().await {
            Ok(result) => result,
            Err(err) => json!({ "error": err.to_string()}) // TODO
        };
        match update(Topic::Status, msg).await {
            Ok(_) => debug!("Clients have been sent info"),
            Err(err) => {
                error!("Error updating {:?}", err);
            }
        }
    }
}