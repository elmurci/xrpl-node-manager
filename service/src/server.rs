use std::{net::{TcpListener}, thread::spawn};
use fehler::throws;
use anyhow::Error;
use serde_json::json;
use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
    Message,
};
use tracing::{debug, info};

use crate::{structs::{IncomingMessage, OutgoingMessage}, handler::handle_incoming_message, enums::ResponseStatus};

pub struct Server {
  port: u16,
}

impl Server {
    pub fn new(port: u16) -> Server {
      Server {
        port,
      }
    }
    #[throws(_)]
    pub(crate) async fn start(&self) {
      let server = TcpListener::bind(format!("127.0.0.1:{}", self.port)).unwrap();
      info!("Server started at {:?}", server.local_addr()?);
      info!("Streams {:?}", server.incoming());
      for stream in server.incoming() {
        spawn(move || {
          let callback = |req: &Request, mut response: Response| {
              debug!("Received a new ws handshake");
              debug!("The request's path is: {}", req.uri().path());
              debug!("The request's headers are:");
              for (ref header, _value) in req.headers() {
                debug!("* {}", header);
              }

              // TODO: Let's add an additional header to our response to the client.
              let headers = response.headers_mut();
              headers.append("MyCustomHeader", ":)".parse().unwrap());
              headers.append("SOME_TUNGSTENITE_HEADER", "header_value".parse().unwrap());

              Ok(response)
          };
          let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();

          loop {
              let msg = websocket.read_message().unwrap();
              debug!("Incoming message {}", msg);
              // TODO: runtime error handling
              let request: IncomingMessage = serde_json::from_str(&msg.to_string()).unwrap();
              let topic =request.clone().topic;
              debug!("Incoming message has been deserialized: {:?}", request);
              tokio::runtime::Runtime::new().unwrap().handle().block_on(async {
                let result = match handle_incoming_message(request).await {
                  Ok(response) => response,
                  Err(err) => {
                    let error = json!({
                        "error": err.to_string()
                    });
                    OutgoingMessage { topic, status: ResponseStatus::Error, message: error }
                  }
                };
                debug!("response {:?}", result);
                if msg.is_binary() || msg.is_text() {
                  let response_message = Message::Text(serde_json::to_string(&result).unwrap());
                  websocket.write_message(response_message).unwrap();
                }
              });
          }
        });
      }
  }
}