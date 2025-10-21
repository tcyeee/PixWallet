import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import { notify } from './utils/notify'
import { alert } from './utils/alert'

import './assets/global.css';

const app = createApp(App);
const pinia = createPinia();

window.Notify = notify
window.Alert = alert

app.use(pinia);
app.mount("#app");