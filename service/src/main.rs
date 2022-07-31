use tokio::task;
use tracing::{error, info, debug};

// #[cfg(test)]
// mod tests;

use settings::get_settings;

use crate::{server::Server, handler::handle_incoming_message, structs::IncomingMessage, enums::Topic};

mod lib;
mod logger;
mod server;
mod ws_client;
mod settings;
mod structs;
mod enums;
mod handler;
mod rpc_client;
mod node;

#[tokio::main]
async fn main() {
  info!("Starting Websockets server!");
  let settings = get_settings();
  logger::setup();
  let result = handle_incoming_message(IncomingMessage { topic: Topic::Config }).await;
  debug!("file {:?}", result);
  // 1. Logs
  // 2. Log rotate
  // 3. Stop
  // 4. Start
  // 5. Log level
  // 6. Configuration
  // 7. Server info
  task::spawn(async {
    // Admin endpoint WS?
    // let mut client = Client::run("ws://localhost:6006").await;
    //client.listen().await;
    // client.send("pepe");
  });
  match Server::new(settings.server.port).start().await {
    Ok(()) => {}
    Err(error) => {
      error!("Errror {}", error);
    }
  };
}
