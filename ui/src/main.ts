import { createApp } from 'vue'
import App from './App.vue'
import { Skeletor } from "vue-skeletor";
import { store } from "./store";
import routes from "./routes";
import VueClickAway from "vue3-click-away";
import VueNumberInput from "@chenfengyuan/vue-number-input";
import vfmPlugin from "vue-final-modal";
import VueToast from "vue-toast-notification";
import VueIframe from "vue-iframes";
import { i18n } from './i18n'
import ws from './util/WsClient'
import { Topic } from './enums';
import './index.css'

const wsClient = new ws('ws://127.0.0.1:8080');
wsClient.periodic(2000, Topic.STATUS);

createApp(App)
  .use(i18n)
  .use(store)
  .use(routes)
  .use(vfmPlugin)
  .use(VueToast)
  .use(VueClickAway)
  .use(VueIframe)
  .provide('wsClient', wsClient)
  .component(Skeletor.name, Skeletor)
  .component("modal", {
    template: "#modal-template",
  })
  .component(VueNumberInput.name!, VueNumberInput)
  .mount("#app");
