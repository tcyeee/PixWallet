<template>
  <div v-bind="$attrs" class="flex gap-2">
    <button type="submit" class="btn btn-primary" @click="createWallet()">
      <span v-if="loadingCreateWallet" class="loading loading-spinner"></span>
      Create wallet</button>
    <button type="submit" class="btn btn-primary" :disabled="loadingRefreshBalance" @click="refreshBalance()">
      <span v-if="loadingRefreshBalance" class="loading loading-spinner"></span>
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
      <tr v-for="(item,index) in walletList" :key="item.public_key" @click="NAV.GoTo('wallet', item)">
        <th>{{ index+1 }}</th>
        <td>{{ item.alias || 'None' }}</td>
        <td>{{ item.public_key }}</td>
        <td>{{ item.network }}</td>
        <td>{{ formatSol(item.balance) }}</td>
      </tr>
    </tbody>
  </table>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import { MsgType, WalletInfo } from "@/models";
import { formatSol } from "@/utils/common";
import API from "@/api";
import NAV from "@/router";

var walletList = ref<WalletInfo[]>([]);
onMounted(async () => {
  walletList.value = (await API.WalletList()) || [];
});

const loadingRefreshBalance = ref(false);
function refreshBalance() {
  loadingRefreshBalance.value = true;
  API.WalletBalanceRefresh();
}

// 创建钱包
const loadingCreateWallet = ref(false);
function createWallet() {
  loadingCreateWallet.value = true;
  API.WalletCreate()
    .then((list) => {
      if (list) walletList.value = list;
    })
    .finally(() => (loadingCreateWallet.value = false));
}

listen<null>(MsgType.BALANCE_REFRESH_END, () => {
  loadingRefreshBalance.value = false;
});

listen<WalletInfo>(MsgType.BALANCE_CHANGE, (event) => {
  let wallet = event.payload;
  console.log(event.payload);
  walletList.value.forEach((item) => {
    if (item.public_key == wallet.public_key) Object.assign(item, wallet);
  });
});
</script>