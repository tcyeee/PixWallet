import { listen } from '@tauri-apps/api/event'
import { MsgType, NetworkStatus } from "@/models";
import { useAppStore } from '@/stores/app';
import { Pinia } from 'pinia';


export async function setupTauriListener(pinia: Pinia) {
    const appStore = useAppStore(pinia)

    /* 循环进行网络状态更新 */
    await listen<NetworkStatus>(MsgType.PING, (event) => appStore.ping(event.payload));

    /* TODO 监听后端通知 */
    await listen('msg', (event) => {
        console.log("===========");
        console.log(event);
        console.log("===========");
    })
}