<template>
  <div v-bind="$attrs" class="flex gap-2">
    <button type="submit" class="btn btn-primary" @click="createWallet()">Create wallet</button>
    <button type="submit" class="btn btn-primary" :disabled="loadingRefreshBalance" @click="refreshBalance()">
      <span v-if="loadingRefreshBalance" class="loading loading-spinner"></span>
      Refresh Balance
    </button>
  </div>
  <table class="table m-w-[300px]">
    <!-- head -->
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
      <!-- row 1 -->
      <tr v-for="(item,index) in walletList" :key="item.public_key" @click="selectOne(item)">
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
import { useNav } from "@/hooks/useNav";
import { WalletCreate, WalletBalanceRefresh, WalletList } from "@/api";
import { formatSol } from "@/utils/common";

const { goTo } = useNav();

onMounted(async () => {
  await dataInit();
});

var walletList = ref<WalletInfo[]>([]);
async function dataInit() {
  walletList.value = (await WalletList()) || [];
}

const loadingRefreshBalance = ref(false);
function refreshBalance() {
  loadingRefreshBalance.value = true;
  WalletBalanceRefresh();
}

listen<Array<WalletInfo>>("refresh_balance", (event) => {
  loadingRefreshBalance.value = false;
  walletList.value = event.payload;
});

async function createWallet() {
  await WalletCreate();
}

function selectOne(wallet: WalletInfo) {
  goTo("wallet", wallet);
}
</script>