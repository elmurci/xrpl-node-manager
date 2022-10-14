#[macro_use]
extern crate lazy_static;

use std::{collections::HashMap, sync::Arc};

use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tracing::info;
use warp::{ws::Message, Filter, Rejection};

use enums::Topic;
use settings::get_settings;

use crate::node::Node;
use crate::worker::Worker;

mod enums;
mod handler;
mod lib;
mod logger;
mod node;
mod rpc_client;
mod settings;
mod structs;
mod worker;
mod ws;

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

fn with_node(
  node: Arc<Node>,
) -> impl Filter<Extract = (Arc<Node>,), Error = std::convert::Infallible> + Clone {
  warp::any().map(move || node.clone())
}

#[tokio::main]
async fn main() {
  let settings = get_settings();
  logger::setup();

  let node = Arc::new(Node::new(&settings.rpc_endpoint, &settings.config));

  // Routes
  let health_route = warp::path!("health").and_then(handler::health_handler);
  let register = warp::path("register");
  let register_route = register
    .and(warp::post())
    .and(warp::body::json())
    .and_then(handler::register_handler)
    .or(
      register
        .and(warp::delete())
        .and(warp::path::param())
        .and_then(handler::unregister_handler),
    );

  let start_route = warp::post()
    .and(warp::path("start"))
    .and(with_node(Arc::clone(&node)))
    .and_then(handler::start_handler);

  let stop_route = warp::path("stop")
    .and(with_node(Arc::clone(&node)))
    .and_then(handler::stop_handler);

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
    .or(start_route)
    .or(stop_route)
    .or(ws_route)
    .with(cors); // TODO: Check the need of CORS

  info!("Starting updates worker");
  let worker = Worker::new(Arc::clone(&node), settings.server.refresh);

  tokio::task::spawn(async move {
    worker.start(settings.server.refresh).await;
  });

  info!(
    "Starting Websockets server! (Port: {})",
    settings.server.port
  );
  warp::serve(routes)
    .run(([127, 0, 0, 1], settings.server.port))
    .await;
}
