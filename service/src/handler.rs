use crate::{ws, Client, Result, enums::Topic, CLIENTS, structs::OutgoingMessage};
use serde::{Deserialize, Serialize};
use tracing::debug;
use uuid::Uuid;
use warp::{http::StatusCode, reply::json, ws::Message, Reply};
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    user_id: String,
}

#[derive(Serialize, Debug)]
pub struct RegisterResponse {
    uuid: String,
}

pub async fn update(topic: Topic, msg: OutgoingMessage) -> Result<impl Reply> {
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

pub async fn register_handler(body: RegisterRequest) -> Result<impl Reply> {
    let user_id = body.user_id;
    let uuid = Uuid::new_v4().as_simple().to_string();
    register_client(uuid.clone(), user_id).await;
    debug!("Registering client {}", uuid);
    Ok(json(&RegisterResponse {
        uuid,
    }))
}

async fn register_client(id: String, user_id: String) {
    CLIENTS.lock().await.insert(
        id,
        Client {
            user_id,
            topics: vec!(),
            sender: None,
        },
    );
}

pub async fn unregister_handler(id: String) -> Result<impl Reply> {
    CLIENTS.lock().await.remove(&id);
    Ok(StatusCode::OK)
}

pub async fn ws_handler(ws: warp::ws::Ws, id: String) -> Result<impl Reply> {
    let client = CLIENTS.lock().await.get(&id).cloned();
    debug!("ws_handler {}", id);
    match client {
        Some(c) => Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, id, c))),
        None => Err(warp::reject::not_found()),
    }
}

pub async fn health_handler() -> Result<impl Reply> {
    Ok(StatusCode::OK)
}