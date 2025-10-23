import { listen } from '@tauri-apps/api/event'
import { MsgType, NetworkStatus, WalletInfo } from "@/models";
import { useUserStore } from '@/stores/user';
import { Pinia } from 'pinia';


export async function setupTauriListener(pinia: Pinia) {
    const userStore = useUserStore(pinia)

    /* 循环进行网络状态更新 */
    await listen<NetworkStatus>(MsgType.PING, (event) => userStore.ping(event.payload));

    /* 某个钱包余额发生变化 */
    listen<WalletInfo>(MsgType.BALANCE_CHANGE, (event) => userStore.updateWallet(event.payload));

    /* TODO 监听后端通知 */
    await listen('msg', (event) => {
        console.log("===========");
        console.log(event);
        console.log("===========");
    })
}