<template>
  <div v-bind="$attrs" class="flex gap-2">
    <button type="submit" class="btn btn-primary" @click="createWallet()">
      <span v-if="loadingCreateWallet" class="loading loading-spinner"></span>
      Create wallet</button>
    <button type="submit" class="btn btn-primary" :disabled="userStore.loading.refresh" @click="refreshBalance()">
      <span v-if="userStore.loading.refresh" class="loading loading-spinner"></span>
      Refresh Balance
    </button>
    <button type="submit" class="btn btn-primary" @click="NAV.GoTo('transfer')">Transfer</button>
  </div>
  <table class="table m-w-[300px]">
    <thead>
      <tr>
        <th></th>
        <th>Alias</th>
        <th>Public Key</th>
        <th>Network</th>
        <th>Balance</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="(item,index) in userStore.wallets" :key="item.public_key" @click="NAV.GoTo('wallet', item)">
        <th>{{ index+1 }}</th>
        <td>{{ item.alias || 'None' }}</td>
        <td>{{ item.public_key }}</td>
        <td>{{ item.network }}</td>
        <td>
          <div class="font-bold text-green-800">{{ lamportsToSol(item.balance) + ' Sol' }}</div>
        </td>
      </tr>
    </tbody>
  </table>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import { MsgType } from "@/models";
import { lamportsToSol } from "@/utils/common";
import API from "@/api";
import NAV from "@/router";
import { useUserStore } from "@/stores/user";

const userStore = useUserStore();

onMounted(async () => {
  userStore.updateWallets();
});

// 创建钱包
const loadingCreateWallet = ref(false);
function createWallet() {
  loadingCreateWallet.value = true;
  API.WalletCreate()
    .then((wallet) => userStore.addWallet(wallet))
    .finally(() => (loadingCreateWallet.value = false));
}

/* 余额刷新 */
function refreshBalance() {
  userStore.loading.refresh = true;
  API.WalletBalanceRefresh();
}
listen<null>(MsgType.BALANCE_REFRESH_END, () => {
  userStore.loading.refresh = false;
});
</script>