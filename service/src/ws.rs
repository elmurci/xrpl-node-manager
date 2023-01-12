use crate::{enums::{Topic, EventType}, CLIENTS, TOPICS, Client};
use futures::{FutureExt, StreamExt};
use serde::Deserialize;
use serde_json::from_str;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tracing::{debug, error};
use warp::ws::{Message, WebSocket};

#[derive(Deserialize, Debug)]
pub struct TopicsRequest {
    event: EventType,
    topics: Vec<Topic>,
}

pub async fn client_connection(ws: WebSocket, id: String, mut client: Client) {
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    let client_rcv = UnboundedReceiverStream::new(client_rcv);
    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            debug!("error sending websocket msg: {}", e);
        }
    }));

    client.sender = Some(client_sender);
    CLIENTS.lock().await.insert(id.clone(), client);

    debug!("{} connected", id);

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                error!("error receiving ws message for id: {}): {}", id.clone(), e);
                break;
            }
        };
        client_msg(&id, msg).await;
    }

    CLIENTS.lock().await.remove(&id);
    debug!("{} disconnected", id);

}

async fn client_msg(id: &str, msg: Message) {
    debug!("Received message from {}: {:?}", id, msg);
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    if message == "ping" || message == "ping\n" {
        return;
    }

    let topics_req: TopicsRequest = match from_str(&message) {
        Ok(v) => v,
        Err(e) => {
            error!("error while parsing message to topics request: {}", e);
            return;
        }
    };

    debug!("Message: {:?}, msg: {}", topics_req.event, topics_req.event == EventType::SUBSCRIBE);

    if topics_req.event == EventType::SUBSCRIBE {
        let mut locked_read_topics = TOPICS.lock().await;
        debug!("Iterate through {} topics in request", topics_req.topics.len());


        for p in &topics_req.topics {
            if !locked_read_topics.contains(&p) {
                debug!("Adding topic {:?}", p);
                locked_read_topics.push(p.clone());
            } else {
                debug!("We don't add {:?} because it is already present", p);
            }
        }

        debug!("FINAL");

        // TODO: ?????
        let mut locked = CLIENTS.lock().await;
        if let Some(v) = locked.get_mut(id) {
            v.topics = topics_req.topics;
        }
    }

}