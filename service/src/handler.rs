use crate::{ws, Client, Result, enums::Topic, CLIENTS, structs::OutgoingMessage, node::Node, settings::Settings};
use serde::{Deserialize, Serialize};
use tracing::debug;
use uuid::Uuid;
use warp::{http::StatusCode, reply::json, ws::Message, Reply};

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    user_id: String,
}

#[derive(Deserialize, Debug)]
pub struct ClientRequest {
    uuid: String,
}

#[derive(Serialize, Debug)]
pub struct RegisterResponse {
    uuid: String,
}

#[derive(Serialize, Debug)]
pub struct ClientResponse {
    message: String,
}

#[derive(Clone)]
pub struct Handler {
    pub node: Node
}
  
impl Handler {
    pub(crate) fn new(settings: &Settings) -> Handler {
        let node = Node::new(&settings.rpc_endpoint, &settings.config);
        Handler {
            node
        }
    }

    pub async fn update(&self, topic: Topic, msg: OutgoingMessage) -> Result<impl Reply> {
        CLIENTS.lock().await.iter()
            .filter(|(_, client)| client.topics.contains(&topic))
            .for_each(|(_, client)| {
                debug!("Updating client {}", client.user_id);
                if let Some(sender) = &client.sender {
                    let _ = sender.send(Ok(Message::text(serde_json::to_string(&msg).unwrap())));
                }
            });
    
        Ok(StatusCode::OK)
    }
    
    pub async fn register_handler(&self, body: RegisterRequest) -> Result<impl Reply> {
        let user_id = body.user_id;
        let uuid = Uuid::new_v4().as_simple().to_string();
        self.register_client(uuid.clone(), user_id).await;
        debug!("Registering client {}", uuid);
        Ok(json(&RegisterResponse {
            uuid,
        }))
    }
    
    pub async fn start_handler(&self) -> Result<impl Reply> {
        // TODO: Verify client is in the list
        debug!("Starting node");
        let start = match self.node.start().await {
            Ok(result) => result.to_string(),
            Err(err) => err.to_string()
        };
    
        Ok(json(&ClientResponse {
            message: start,
        }))
    }
    
    pub async fn stop_handler(&self) -> Result<impl Reply> {
        // TODO: Verify client is in the list
        debug!("Stopping node");
        let start = match self.node.stop().await {
            Ok(result) => result.to_string(),
            Err(err) => err.to_string()
        };
    
        Ok(json(&ClientResponse {
            message: start,
        }))
    }
    
    async fn register_client(&self, id: String, user_id: String) {
        CLIENTS.lock().await.insert(
            id,
            Client {
                user_id,
                topics: vec!(),
                sender: None,
            },
        );
    }
    
    pub async fn unregister_handler(&self, id: String) -> Result<impl Reply> {
        CLIENTS.lock().await.remove(&id);
        Ok(StatusCode::OK)
    }
    
    pub async fn ws_handler(&self, ws: warp::ws::Ws, id: String) -> Result<impl Reply> {
        let client = CLIENTS.lock().await.get(&id).cloned();
        debug!("ws_handler {}", id);
        match client {
            Some(c) => Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, id, c))),
            None => Err(warp::reject::not_found()),
        }
    }
    
    pub async fn health_handler(&self) -> Result<impl Reply> {
        Ok(StatusCode::OK)
    }
}