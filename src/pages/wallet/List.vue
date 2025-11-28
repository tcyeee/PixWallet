<template>
  <div v-bind="$attrs" class="flex gap-2 mb-6">
    <button type="submit" class="btn btn-primary" @click="createWallet()">
      <span v-if="loadingCreateWallet" class="loading loading-spinner"></span>
      Create wallet</button>
    <button type="submit" class="btn btn-primary" :disabled="userStore.loading.refresh" @click="refreshBalance()">
      <span v-if="userStore.loading.refresh" class="loading loading-spinner"></span>
      Refresh Balance
    </button>
  </div>

  <!-- 拟物风格的银行卡列表（最多 5 张） -->
  <div class="wallet-cards-wrapper">
    <div
      v-for="i in 5"
      :key="i"
      class="wallet-card"
      :class="{
        'wallet-card--empty': !userStore.wallets[i - 1],
        'cursor-pointer': userStore.wallets[i - 1]
      }"
      :style="getCardStyle(i)"
      @click="userStore.wallets[i - 1] && NAV.GoTo('wallet-item', userStore.wallets[i - 1])"
    >
      <template v-if="userStore.wallets[i - 1]">
        <div class="wallet-card__header">
          <span class="wallet-card__alias">
            {{ userStore.wallets[i - 1].alias || '未命名钱包' }}
          </span>
        </div>
        <div class="wallet-card__balance">
          {{ lamportsToSol(userStore.wallets[i - 1].balance) }} SOL
        </div>
        <div class="wallet-card__number">
          {{ formatCardNumber(userStore.wallets[i - 1].public_key) }}
        </div>
      </template>
      <template v-else>
        <div class="wallet-card__placeholder">
          空卡槽
        </div>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import { MsgType } from "@/models";
import { lamportsToSol } from "@/utils/common";
import API from "@/api";
import NAV from "@/router";
import { useUserStore } from "@/stores/user";

const userStore = useUserStore();

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

// 拟物卡片样式：根据索引做位移与层级，形成叠放效果
function getCardStyle(i: number) {
  const index = i - 1;
  return {
    transform: `translateY(${index * 10}px) translateX(${index * 8}px)`,
    zIndex: String(10 - index),
  };
}

// 卡号展示：取公钥前后几位，模拟银行卡号
function formatCardNumber(pubkey: string) {
  if (!pubkey) return "";
  const head = pubkey.slice(0, 4);
  const tail = pubkey.slice(-4);
  return `${head} •••• •••• ${tail}`;
}
</script>

<style scoped>
.wallet-cards-wrapper {
  position: relative;
  width: 360px;
  max-width: 100%;
  height: 220px;
  margin-top: 8px;
}

.wallet-card {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 180px;
  border-radius: 18px;
  padding: 18px 20px;
  background: linear-gradient(135deg, #1b2735, #283e51);
  box-shadow:
    0 14px 30px rgba(0, 0, 0, 0.35),
    inset 0 1px 0 rgba(255, 255, 255, 0.15);
  color: #f5f7fa;
  transition:
    transform 0.2s ease,
    box-shadow 0.2s ease,
    opacity 0.2s ease;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  backdrop-filter: blur(3px);
}

.wallet-card:hover {
  box-shadow:
    0 20px 40px rgba(0, 0, 0, 0.45),
    inset 0 1px 0 rgba(255, 255, 255, 0.18);
}

.wallet-card--empty {
  background: linear-gradient(135deg, #2c3e50, #2c3e50);
  opacity: 0.25;
  box-shadow: none;
}

.wallet-card__header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 0.9rem;
  opacity: 0.9;
}

.wallet-card__alias {
  font-weight: 600;
}

.wallet-card__balance {
  margin-top: 16px;
  font-size: 1.4rem;
  font-weight: 700;
  letter-spacing: 0.05em;
}

.wallet-card__number {
  font-family: "SF Mono", ui-monospace, Menlo, Monaco, Consolas, "Liberation Mono",
    "Courier New", monospace;
  font-size: 0.95rem;
  letter-spacing: 0.15em;
  margin-top: 10px;
  opacity: 0.9;
}

.wallet-card__placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.95rem;
  color: rgba(255, 255, 255, 0.7);
}
</style>