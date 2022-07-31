//! A simple example of hooking up stdin/stdout to a WebSocket stream.
//!
//! This example will connect to a server specified in the argument list and
//! then forward all data read on stdin to the server, printing out all data
//! received on stdout.
//!
//! Note that this is not currently optimized for performance, especially around
//! buffer management. Rather it's intended to show an example of working with a
//! client.
//!
//! You can use this example together with the `server` example.

use futures::{future, pin_mut, StreamExt};

use async_std::io;
use async_std::prelude::*;
use async_std::task;
use async_tungstenite::async_std::connect_async;
use async_tungstenite::tungstenite::protocol::Message;
use tracing::{info, debug};

pub struct WsClient {
    port: u16,
  }
  
  impl WsClient {
    pub(crate) async fn run(endpoint: &str) {
        let (stdin_tx, stdin_rx) = futures::channel::mpsc::unbounded();
        Self::read_stdin(stdin_tx).await;

        let (ws_stream, _) = connect_async(endpoint)
            .await
            .expect("Failed to connect to websockets server");
        info!("WebSocket handshake has been successfully completed");

        let (write, read) = ws_stream.split();

        let stdin_to_ws = stdin_rx.map(Ok).forward(write);
        let ws_to_stdout = {
            read.for_each(|message| async {
                let data = message.unwrap().into_data();
                // async_std::io::stdout().write_all(&data).await.unwrap();
                debug!("Received data: {:?}", data)
            })
        };

        pin_mut!(stdin_to_ws, ws_to_stdout);
        future::select(stdin_to_ws, ws_to_stdout).await;
    }

    // Our helper method which will read data from stdin and send it along the
    // sender provided.
    async fn read_stdin(tx: futures::channel::mpsc::UnboundedSender<Message>) {
        let mut stdin = io::stdin();
        loop {
            let mut buf = vec![0; 1024];
            let n = match stdin.read(&mut buf).await {
                Err(_) | Ok(0) => break,
                Ok(n) => n,
            };
            buf.truncate(n);
            tx.unbounded_send(Message::binary(buf)).unwrap();
        }
    }
  }
