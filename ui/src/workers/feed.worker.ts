import { EventType, Topic } from "@/enums";
import WsClient from "@/util/WsClient";

let ws: WsClient;

self.onmessage = (message: MessageEvent) => {
  console.log('event-----', message.data);
  switch (message.data.event) {
    case EventType.ENDPOINT: // Outgoing
      ws = new WsClient(message.data.value);
      break;
    case EventType.SUBSCRIBE: // Outgoing
      ws.closeFeed(message.data.topic);
      break;
    case EventType.UNSUBSCRIBE: // Outgoing
      ws.openFeed(message.data.topic);
      break;
    case EventType.FEED: // Incoming
      console.log('MIENE UN FEED!!!!!!');
    default:
      throw('Event not recognized');
  }
};