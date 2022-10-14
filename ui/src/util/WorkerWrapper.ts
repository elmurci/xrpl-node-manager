import { EventType } from '@/enums';
import { WsMessage } from '@/types/WsMessage';
import { store } from '@/store';

export default class AppWorker {
  worker: Worker;
  constructor(uuid: string) {
    const worker = new Worker(new URL('../workers/feed.worker.ts', import.meta.url), {
      type: 'module'
    });
    worker.postMessage({ event: EventType.ENDPOINT, value: `ws://127.0.0.1:8080/ws/${uuid}`});
    store.commit('wsConnected', true); // TODO, fix this
    worker.onmessage = (event) => {
      switch (event.data.topic) {
          case 'feed_closed':
            console.log('frontend: feed killed');
            break;
          case 'status':
            console.log('status: actualizamos estado', event.data.message)
            store.commit('info', event.data.message.info);
            store.commit('status', event.data.status === 'success');
            break;
          case 'features':
              console.log('features: actualizamos estado', event.data.message)
              store.commit('features', event.data.message.features);
              store.commit('status', event.data.status === 'success');
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