import { NetworkStatus, WalletInfo } from "@/models";
import { defineStore } from "pinia";
import API from "@/api";

export const useUserStore = defineStore('app', {
    state: () => ({
        network: {} as NetworkStatus,  // 当前网络状态
        wallets: [] as WalletInfo[]    // 用户钱包列表
    }),
    actions: {
        ping(statue: NetworkStatus) {
            this.network = statue;
        },
        async updateWallets() {
            this.wallets = await API.WalletList();
        },
        updateWallet(wallet: WalletInfo) {
            this.wallets.forEach((item) => {
                if (item.public_key == wallet.public_key) Object.assign(item, wallet);
            });
        },
        addWallet(wallet: WalletInfo) {
            this.wallets.push(wallet)
        }
    },
    persist: true
})