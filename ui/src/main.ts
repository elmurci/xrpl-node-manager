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
import { i18n } from './i18n';
import './index.css';
import WorkerWrapper from './util/WorkerWrapper'
import axios from 'axios';
import { v4 as uuidv4 } from 'uuid';

let wr:WorkerWrapper;

// External function in module
function initializeApp () {
  return new Promise(async (resolve, reject) => {
    try {
      console.log('registration start');
      const registration = await axios.post('http://127.0.0.1:8080/register', { 'user_id': uuidv4() })
      console.log('registration', registration.data.uuid);
      wr = new WorkerWrapper(registration.data.uuid);
      resolve(true);
    } catch (e) {
      resolve(false);
    }
  })
}

initializeApp().then(_ => {
  createApp(App)
  .use(i18n)
  .use(store)
  .use(routes)
  .use(vfmPlugin)
  .use(VueToast)
  .use(VueClickAway)
  .use(VueIframe)
  .provide('worker', wr)
  .component(Skeletor.name, Skeletor)
  .component("modal", {
    template: "#modal-template",
  })
  .component(VueNumberInput.name!, VueNumberInput)
  .mount("#app");
}).catch(function (error) {
  if (error.response) {
      console.log('Error data : ', error.response.data);
      console.log('Error status : ', error.response.status);
      console.log('Error headers : ', error.response.headers);
  } else if (error.request) {
      console.log('Error request : ', error.request);
  } else {
      console.log('Error message : ', error.message);
  }
});
