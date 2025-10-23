import { NetworkStatus, WalletInfo } from "@/models";
import { defineStore } from "pinia";

export const useUserStore = defineStore('app', {
    state: () => ({
        network: {} as NetworkStatus,  // 当前网络状态
        wallets: [] as WalletInfo[]    // 用户钱包列表
    }),
    actions: {
        ping(statue: NetworkStatus) {
            this.network = statue;
            console.log(`[PING]: ${statue.ping}ms`);
        }
    }
})