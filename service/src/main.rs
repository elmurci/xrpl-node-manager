#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use tokio::sync::Mutex;
use enums::Topic;
use tokio::sync::mpsc;
use warp::{ws::Message, Filter, Rejection};
use tracing::info;

mod handler;
mod ws;
mod logger;
mod settings;
mod node;
mod rpc_client;
mod lib;
mod structs;
mod enums;
mod worker;

use settings::get_settings;

type Result<T> = std::result::Result<T, Rejection>;

// Global clients and topics
lazy_static! {
    pub static ref CLIENTS: Mutex<HashMap<String, Client>> = Mutex::new(HashMap::new());
    pub static ref TOPICS: Mutex<Vec<Topic>> = Mutex::new(vec!());
}
    
#[derive(Debug, Clone)]
pub struct Client {
    pub user_id: String,
    pub topics: Vec<Topic>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

#[tokio::main]
async fn main() {
    let settings = get_settings();
    logger::setup();

    let health_route = warp::path!("health").and_then(handler::health_handler);

    let register = warp::path("register");
    let register_route = register
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handler::register_handler)
        .or(register
            .and(warp::delete())
            .and(warp::path::param())
            .and_then(handler::unregister_handler));

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        .and_then(handler::ws_handler);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["POST", "GET"]);

    let routes = health_route
        .or(register_route)
        .or(ws_route)
        .with(cors); // TODO: Check the need of CORS

    info!("Starting updates worker");
    tokio::task::spawn(async move {
        worker::main_worker(settings.server.refresh, &settings.rpc_endpoint).await;
    });

    info!("Starting Websockets server! (Port: {})", settings.server.port);
    warp::serve(routes).run(([127, 0, 0, 1], settings.server.port)).await;

}