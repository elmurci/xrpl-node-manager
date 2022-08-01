use fehler::throws;
use anyhow::{Error, bail};

use crate::{structs::{IncomingMessage, OutgoingMessage}, enums::{Topic, ResponseStatus}, node::Node};

#[throws(_)]
pub async fn handle_incoming_message(message: IncomingMessage) -> OutgoingMessage {
    let node = Node::new("http://rippled:5005"); // TODO
    let msg = match &message.topic {
        Topic::Start => node.start().await?,
        Topic::Stop => node.stop().await?,
        Topic::Status => node.info().await?,
        Topic::Stats => node.stats().await?,
        Topic::Config => node.config().await?,
        Topic::Features => node.features().await?,
        _ => bail!("Topic not recognized!")
    };
    OutgoingMessage {
        topic: message.topic,
        status: ResponseStatus::Success,
        message: msg
    }
}