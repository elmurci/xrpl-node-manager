import { Topic } from "@/enums";
import { store } from "@/store";

export default class WsClient {

  client: WebSocket;

  endpoint: string;

  constructor(endpoint: string) {
    this.client = new WebSocket(endpoint);
    this.endpoint = endpoint;
    this.connect();
  }

  periodic(interval: number, topic: Topic) {
    const intervalId = setInterval(() => {
        this.send(topic);
    }, interval);
  }

  connect() {
    console.log('connecting to ', this.endpoint);
    this.client = new WebSocket(this.endpoint);
    this.client.onerror = () => {
        console.log('Connection Error');
    };
    
    this.client.onopen = () =>  {
        console.log('WebSocket Client Connected');
        store.commit('wsConnected');
    };
    
    this.client.onclose = () =>  {
        console.log('echo-protocol Client Closed');
        store.commit('wsDisconnected');
        store.commit('status', {});
        setTimeout(() => {
            this.connect();
        }, 1000);
    };
    
    this.client.onmessage = (e) =>  {
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
        }
    };
  }

  send(
    topic: Topic,
  ) {
    if (this.client.readyState === this.client.OPEN) {
        this.client.send(JSON.stringify({
            topic,
        }));
    } else {
        // TODO
        console.log('Error, not connected');
    }
  }
}
