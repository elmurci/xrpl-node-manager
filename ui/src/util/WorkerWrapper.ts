import { EventType } from '@/enums';
import { WsMessage } from '@/types/WsMessage';
import MyWorker from '../workers/feed.worker?worker&url';

export default class AppWorker {
  worker: Worker;
  constructor(uuid: string) {
    const worker = new Worker(new URL('../workers/feed.worker.ts', import.meta.url), {
      type: 'module'
    });
    worker.postMessage({ event: EventType.ENDPOINT, value: `ws://127.0.0.1:8080/ws/${uuid}`});

    worker.onmessage = (event) => {
        console.log('pepico', event.data);
        switch (event.data.type) {
            case "FEED_CLOSED":
              console.log("frontend: feed killed");
              break;
            default:
              console.log('feed ', event);
        }
    };

    this.worker = worker;
  }

  message(message: WsMessage) {
    this.worker.postMessage(message);
  }
}