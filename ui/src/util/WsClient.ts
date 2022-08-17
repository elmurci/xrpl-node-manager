import { EventType, Topic } from "@/enums";
import { store } from "@/store";
import { WsMessage } from "@/types/WsMessage";

export default class WsClient {

  client: WebSocket;

  endpoint: string;

  constructor(endpoint: string) {
    console.log('endpoint', endpoint);
    this.client = new WebSocket(endpoint);
    this.endpoint = endpoint;
    this.connect();
  }

  connect() {
    console.log('connect-------------');
    this.client.onerror = (e) => {
        console.log('Connection Error', e);
    };
    
    this.client.onopen = () =>  {
        console.log('WebSocket Client Connected');
        this.client.send(JSON.stringify({
          event: EventType.SUBSCRIBE,
          topics: [Topic.STATUS, Topic.CONFIG],
        }));
        store.commit('wsConnected');
    };
    
    this.client.onclose = () =>  {
        console.log('echo-protocol Client Closed');
        store.commit('wsDisconnected');
        store.commit('status', {});
    };
    
    this.client.onmessage = (e) =>  {
        console.log('onmessage-------------', e);
        if (typeof e.data === 'string') {
            const json = JSON.parse(e.data);
            console.log("received: " + json.topic, json);
            switch(json.topic) {
                case 'status': 
                  if (json.message.error) {
                    store.commit('status', false);
                  } else {
                    store.commit('status', true);
                    store.commit('info', json.message.info);
                  }
                  break;
                case 'stats': 
                  store.commit('stats', json.message);
                  break;
                case 'config': 
                  store.commit('config', json.message);
                  break;
                case 'features': 
                  store.commit('features', json.message);
                  break;
                default:
                  // TODO
            }
            postMessage(json);     
        }
    };
  }

  closeFeed(topic: Topic) {
    try {
      const unsubscribe = {
        event: EventType.UNSUBSCRIBE,
        topic,
      };
      this.client.send(JSON.stringify(unsubscribe));
      this.client.close();
      postMessage({
        type: "FEED_CLOSED",
      });
    } catch (e) {
      console.error("Caught error", e);
      throw e;
    }
  }

  openFeed(topic: Topic) {
    try {
      const subscribe: WsMessage = {
        event: EventType.SUBSCRIBE,
        topic,
      };
      this.client.send(JSON.stringify(subscribe));
      postMessage({
        type: "FEED_OPEN",
      });
    } catch (e) {
      console.error("Caught error", e);
      throw e;
    }
  }
}