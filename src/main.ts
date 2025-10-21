import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import { notify } from './utils/notify'
import { alert } from './utils/alert'
import router from './router'
import './assets/global.css';
import { setupTauriListener } from './plugins/tauriListener'

// 添加Alert & Message 信号监听
setupTauriListener()

const app = createApp(App);
const pinia = createPinia();

window.Notify = notify
window.Alert = alert

app.use(pinia);
app.use(router);
app.mount("#app");