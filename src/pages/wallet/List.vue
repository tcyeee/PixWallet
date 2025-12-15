<template>
  <div class="flex gap-8 items-start">
  <!-- 价格卡片列表 -->
    <TokenPriceCard
      v-for="item in tokenPriceList"
      :key="item.symbol"
      :symbol="item.symbol"
      :usd="item.usd"
      :expo="item.expo"
    />
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
        <div class="text-gray-400 text-sm">SUM</div>
        <button
          class="btn btn-sm btn-ghost text-gray-400 hover:text-white p-1 min-h-0 h-auto"
          :disabled="userStore.loading.refresh"
          @click="refreshBalance()">
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
        <span class="text-orange-400 text-5xl font-bold tracking-wide font-pix-primary">
          {{ totalBalance }}
        </span>
        <span class="text-gray-400 text-lg font-pix-secondary">SOL</span>
      </div>
      <div class="mt-4 text-gray-500 text-xs">
        Total {{ walletCount }} wallets
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import { listen , UnlistenFn} from "@tauri-apps/api/event"; 
import API from "@/api";
import NAV from "@/router";
import { useUserStore } from "@/stores/user";
import WalletCardList from "@/components/WalletCardList.vue";
import TokenPriceCard from "@/components/TokenPriceCard.vue";
import { lamportsToSol } from "@/utils/common";
import { MsgType , TokePriceResp} from "@/models";
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

const tokenPriceList = ref<TokePriceResp[]>([]);
const loadingPrices = ref(false);

function getPrice() {
   API.TokenPrice({"symbol":"SOL"}).then((res)=> {
    tokenPriceList.value = res;
    loadingPrices.value = true;
    console.log("token返回值:", res);
   })
}

// 1. 页面加载时获取价格
onMounted(async () => {
  console.log("📡 页面加载，开始获取价格...");
  
   getPrice();
  // 监听价格刷新事件
  setupPriceListener();
});

//  2. 清理监听器
onUnmounted(() => {
  if (unlistenPriceRefresh) {
    unlistenPriceRefresh();
  }
});

// 4. 监听后端价格刷新通知
let unlistenPriceRefresh: UnlistenFn | null = null;

async function setupPriceListener() {
  try {
    // 监听你后端发送的 RefreshTokenPrice 事件
    unlistenPriceRefresh = await listen("REFRESH_TOKEN_PRICE", (event) => {
      console.log("🔄 收到价格刷新通知:", event);
      
      // 重新获取价格
      getPrice();
    });
    
    console.log(" 已监听价格刷新事件");
    
  } catch (error) {
    console.error(" 设置价格监听失败:", error);
  }
}

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
