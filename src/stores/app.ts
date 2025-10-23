import { NetworkStatus } from "@/models";
import { defineStore } from "pinia";

export const useAppStore = defineStore('app', {
    state: () => ({
        network: {} as NetworkStatus
    }),
    actions: {
        ping(statue: NetworkStatus) {
            this.network = statue;
            console.log("[PING]: " + JSON.stringify(statue));
        }
    }
})