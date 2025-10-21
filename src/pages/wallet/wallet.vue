<template>
  <div class="pb-5">
    <div class="text-gray-500 font-bold text-8xl pb-5">Hi</div>
    <div class="flex gap-2">
      <button type="submit" class="btn btn-primary" @click="createWallet()">Create wallet</button>
      <button type="submit" class="btn btn-primary" :disabled="loadingRefreshBalance" @click="refreshBalance()">
        <span v-if="loadingRefreshBalance" class="loading loading-spinner"></span>
        Refresh Balance
      </button>
    </div>
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
      <tr v-for="(item,index) in walletList" :key="item.public_key" @click="select(item)">
        <th>{{ index+1 }}</th>
        <td>{{ item.alias || 'None' }}</td>
        <td>{{ item.public_key }}</td>
        <td>{{ item.network }}</td>
        <td>{{ formatSol(item.balance) }}</td>
      </tr>
    </tbody>
  </table>

  <!-- 对话框 -->
  <dialog id="wallet_modal" class="modal">
    <div v-if="selectWallet" class="modal-box">

      <div class="space-y-2 mb-4">
        <div class="flex items-center">
          <span class="w-24 font-medium">Alias:</span>
          <div class="flex gap-2 w-full">
            <input type="text" class="input" placeholder="Type here" v-model="alias" />
            <button class="btn" @click="changeAlias()">修改</button>
          </div>
        </div>
        <div class="flex items-center">
          <span class="w-24 font-medium">Network:</span>
          <span class="truncate">{{ selectWallet.network }}</span>
        </div>
        <div class="flex items-center">
          <span class="w-24 font-medium">Public Key:</span>
          <span class="truncate break-all max-w-full text-sm bg-base-200 px-2 py-1 rounded">{{ selectWallet.public_key }}</span>
        </div>
      </div>
      <div class="flex justify-end gap-2">
        <button class="btn btn-error" @click="deleteAccount()">删除账户</button>
        <button class="btn" @click="dialogToggle(false)">Close</button>
      </div>
    </div>
  </dialog>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";
import { listen } from "@tauri-apps/api/event";

// 请求页面初始数据
onMounted(async () => {
  await dataInit();
});

type WalletInfo = {
  public_key: string;
  alias: string;
  network: string;
  balance: number;
};

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
  console.log(event.payload);
});

async function createWallet() {
  try {
    const res = await invoke<Array<WalletInfo>>("create_wallet");
    walletList.value = res;
  } catch (e) {
    Alert.error(e as string);
  }
}

var selectWallet = ref<WalletInfo>();
var alias = ref<String>();
function select(wallet: WalletInfo) {
  selectWallet.value = wallet;
  alias.value = wallet.alias || "";
  dialogToggle(true);
}

async function changeAlias() {
  const params = {
    publicKey: selectWallet.value?.public_key,
    newAlias: alias.value,
  };
  walletList.value = await invoke<Array<WalletInfo>>("change_alias", params);
  dialogToggle(false);
}

async function deleteAccount() {
  const params = { publicKey: selectWallet.value?.public_key };
  walletList.value = await invoke<Array<WalletInfo>>("delete_wallet", params);
  dialogToggle(false);
}

function dialogToggle(show: boolean) {
  const modal = document.getElementById("wallet_modal") as HTMLDialogElement;
  if (!modal) return;
  if (show) {
    modal.showModal();
  } else {
    modal.close();
  }
}

function formatSol(lamport: number | undefined): string {
  if (!lamport) return "0 SOL";
  return (lamport / 1_000_000_000).toFixed(2) + " SOL";
}
</script>