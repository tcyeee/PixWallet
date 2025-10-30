import { listen } from '@tauri-apps/api/event'
import { MsgType, NetworkStatus, NoticeEnum, NoticeParams, WalletInfo } from "@/models";
import { useUserStore } from '@/stores/user';
import { Pinia } from 'pinia';
import { notify } from "@/utils/notify";
import { alert } from '@/utils/alert';

export async function setupTauriListener(pinia: Pinia) {
    const userStore = useUserStore(pinia)
    /* 循环进行网络状态更新 */
    await listen<NetworkStatus>(MsgType.PING, (event) => userStore.ping(event.payload));
    /* 全部钱包更新 */
    listen<WalletInfo>(MsgType.REFRESH_WALLET, (event) => userStore.updateWallet(event.payload));

    /* 监听后端通知 */
    await listen(MsgType.NOTICE, (event) => {
        let param = event.payload as NoticeParams;
        if (param.level == NoticeEnum.INFO) notify.info(param.content)
        if (param.level == NoticeEnum.SUCCESS) notify.success(param.content)
        if (param.level == NoticeEnum.WARNING) notify.warning(param.content)
        if (param.level == NoticeEnum.ERROR) notify.error(param.content)
    })

    /* 监听后端警告 */
    await listen(MsgType.ALERT, (event) => {
        let param = event.payload as NoticeParams;
        if (param.level == NoticeEnum.INFO) alert.info(param.content)
        if (param.level == NoticeEnum.SUCCESS) alert.success(param.content)
        if (param.level == NoticeEnum.WARNING) alert.warning(param.content)
        if (param.level == NoticeEnum.ERROR) alert.error(param.content)
    })
}