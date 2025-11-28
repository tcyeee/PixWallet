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

  <!-- 拟物风格的银行卡列表（最多 5 张） -->
  <div class="relative w-[360px] max-w-full h-[220px] mt-2">
    <div
      v-for="(slot, index) in cardSlots"
      :key="index"
      class="wallet-card"
      :class="{
        'bg-gradient-to-br from-[#1b2735] to-[#283e51]': slot,
        'bg-gradient-to-br from-[#2c3e50] to-[#2c3e50] opacity-25 shadow-none': !slot,
        'cursor-pointer': slot
      }"
      :style="getCardStyle(index)"
      @click="slot && NAV.GoTo('wallet-item', slot)"
    >
      <template v-if="slot">
        <div class="flex justify-between items-center">
          <div class="font-mono text-[0.95rem] tracking-[0.15em] opacity-90">
            {{ formatCardNumber(slot.public_key) }}
          </div>
          <div class="text-[1.4rem] font-bold tracking-wide">
            <span class="text-orange-400">{{ lamportsToSol(slot.balance) }}</span> 
            <span class="text-xs text-gray-400"> SOL</span>
          </div>
        </div>
      </template>
      <template v-else>
        <div class="">
          <button
            class="btn btn-sm btn-primary text-white"
            :disabled="loadingCreateWallet"
            @click.stop="createWallet()"
          >
            <span v-if="loadingCreateWallet" class="loading loading-spinner loading-xs"></span>
            <span v-else>Create wallet</span>
          </button>
        </div>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { listen } from "@tauri-apps/api/event";
import { MsgType } from "@/models";
import { lamportsToSol } from "@/utils/common";
import API from "@/api";
import NAV from "@/router";
import { useUserStore } from "@/stores/user";

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
function getCardStyle(index: number) {
  return {
    transform: `translateY(${index * 60}px) translateX(${index * 0}px)`,
  };
}

// 卡号展示：取公钥前后几位，模拟银行卡号
function formatCardNumber(pubkey: string) {
  if (!pubkey) return "";
  const head = pubkey.slice(0, 4);
  const tail = pubkey.slice(-4);
  return `${head} •••• ${tail}`;
}
</script>

<style scoped>
.wallet-card {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 180px;
  border-radius: 18px;
  padding: 10px 20px;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  backdrop-filter: blur(4px);
  transition-property: all;
  transition-duration: 200ms;
  transition-timing-function: ease-in-out;
  color: #f5f7fa;
}

/* 为拟物风格卡片添加内阴影效果（仅非空卡片） */
.wallet-card:not(.opacity-25) {
  box-shadow:
    0 14px 30px rgba(0, 0, 0, 0.35),
    inset 0 1px 0 rgba(255, 255, 255, 0.15);
}

.wallet-card:not(.opacity-25):hover {
  box-shadow:
    0 20px 40px rgba(0, 0, 0, 0.45),
    inset 0 1px 0 rgba(255, 255, 255, 0.18);
}
</style>