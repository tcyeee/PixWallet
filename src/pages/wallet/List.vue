<template>
  <!-- <div v-bind="$attrs" class="flex gap-2 mb-6">
    <button type="submit" class="btn btn-primary" @click="createWallet()">
      <span v-if="loadingCreateWallet" class="loading loading-spinner"></span>
      Create wallet</button>
    <button type="submit" class="btn btn-primary" :disabled="userStore.loading.refresh" @click="refreshBalance()">
      <span v-if="userStore.loading.refresh" class="loading loading-spinner"></span>
      Refresh Balance
    </button>
  </div> -->

  <div class="flex gap-8 items-start">
    <!-- 左侧卡片列表 -->
    <WalletCardList
      :card-slots="cardSlots"
      :loading-create-wallet="loadingCreateWallet"
      @card-click="NAV.GoTo('wallet-item', $event)"
      @create-wallet="createWallet()"
    />

    <!-- 右侧显示用户的总余额，总余额等于用户所有银行卡片的余额之和 -->
    <div class="total-balance-card">
      <div class="flex items-center justify-between mb-2">
        <div class="text-gray-400 text-sm">总余额</div>
        <button
          class="btn btn-sm btn-ghost text-gray-400 hover:text-white p-1 min-h-0 h-auto"
          :disabled="userStore.loading.refresh"
          @click="refreshBalance()"
          title="刷新余额"
        >
          <span v-if="userStore.loading.refresh" class="loading loading-spinner loading-xs"></span>
          <svg
            v-else
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            class="w-4 h-4"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99"
            />
          </svg>
        </button>
      </div>
      <div class="flex items-baseline gap-2">
        <span class="text-orange-400 text-4xl font-bold tracking-wide">
          {{ totalBalance }}
        </span>
        <span class="text-gray-400 text-lg">SOL</span>
      </div>
      <div class="mt-4 text-gray-500 text-xs">
        共 {{ walletCount }} 个钱包
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { listen } from "@tauri-apps/api/event";
import { MsgType } from "@/models";
import API from "@/api";
import NAV from "@/router";
import { useUserStore } from "@/stores/user";
import WalletCardList from "@/components/WalletCardList.vue";
import { lamportsToSol } from "@/utils/common";

const userStore = useUserStore();

// 卡槽数据：固定 5 个，空卡槽排在前面，真实钱包排在后面
const cardSlots = computed(() => {
  const maxSlots = 5;
  const wallets = userStore.wallets || [];

  const rawSlots = Array.from({ length: maxSlots }, (_, i) => wallets[i] || null);
  const empties = rawSlots.filter((s) => !s);
  const filled = rawSlots.filter((s) => !!s);

  return [...empties, ...filled];
});

// 计算总余额：所有钱包余额之和
const totalBalance = computed(() => {
  const wallets = userStore.wallets || [];
  const totalLamports = wallets.reduce((sum, wallet) => sum + (wallet.balance || 0), 0);
  return lamportsToSol(totalLamports);
});

// 钱包数量
const walletCount = computed(() => {
  return userStore.wallets?.length || 0;
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

<style scoped>
.total-balance-card {
  background: linear-gradient(135deg, #1b2735 0%, #283e51 100%);
  border-radius: 18px;
  padding: 24px 32px;
  min-width: 200px;
  box-shadow:
    0 14px 30px rgba(0, 0, 0, 0.35),
    inset 0 1px 0 rgba(255, 255, 255, 0.15);
  color: #f5f7fa;
  transition-property: all;
  transition-duration: 200ms;
  transition-timing-function: ease-in-out;
}

.total-balance-card:hover {
  box-shadow:
    0 20px 40px rgba(0, 0, 0, 0.45),
    inset 0 1px 0 rgba(255, 255, 255, 0.18);
}
</style>
