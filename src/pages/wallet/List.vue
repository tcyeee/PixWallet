<template>
  <div v-bind="$attrs" class="flex gap-2">
    <button type="submit" class="btn btn-primary" @click="API.WalletCreate()">Create wallet</button>
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
import { WalletInfo } from "@/models";
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

listen<Array<WalletInfo>>("refresh_balance", (event) => {
  loadingRefreshBalance.value = false;
  walletList.value = event.payload;
});
</script>