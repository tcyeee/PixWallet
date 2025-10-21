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
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import { WalletInfo } from "@/models";
import { useNav } from "@/hooks/useNav";

const { goTo } = useNav();

onMounted(async () => {
  await dataInit();
});

var walletList = ref<WalletInfo[]>([]);
async function dataInit() {
  try {
    const res = await invoke<WalletInfo[]>("query_wallet");
    walletList.value = res || [];
  } catch (err) {
    Alert.error(`加载钱包数据失败: ${err}`);
  }
}

const loadingRefreshBalance = ref(false);
function refreshBalance() {
  loadingRefreshBalance.value = true;
  invoke<null>("refresh_balance");
}

listen<Array<WalletInfo>>("refresh_balance", (event) => {
  loadingRefreshBalance.value = false;
  walletList.value = event.payload;
});

async function createWallet() {
  try {
    const res = await invoke<Array<WalletInfo>>("create_wallet");
    walletList.value = res;
  } catch (e) {
    Alert.error(e as string);
  }
}

function selectOne(wallet: WalletInfo) {
  goTo("wallet", wallet);
}

function formatSol(lamport: number | undefined): string {
  if (!lamport) return "0 SOL";
  return (lamport / 1_000_000_000).toFixed(2) + " SOL";
}
</script>