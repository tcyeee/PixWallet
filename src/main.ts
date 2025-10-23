import { createApp } from "vue";
import { createPinia } from "pinia";
import piniaPersistedstate from 'pinia-plugin-persistedstate'
import App from "./App.vue";
import { notify } from './utils/notify'
import { alert } from './utils/alert'
import router from './router/init'
import './assets/global.css';
import { setupTauriListener } from './plugins/tauriListener'

const pinia = createPinia()
    .use(piniaPersistedstate)

createApp(App)
    .use(pinia)
    .use(router)
    .mount("#app")

window.Notify = notify
window.Alert = alert

/* 监听方法, 这里面用到了pinia,报错了 */
setupTauriListener(pinia)