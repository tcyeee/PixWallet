<template>
  <ReturnButton />
  <div class="p-5 h-screen overflow-y-scroll pb-[30vh]">
    <!-- 钱包信息卡片 -->
    <div class="wallet-info-card mb-4">
      <div class="flex items-center justify-between mb-4">
        <div class="flex items-baseline gap-2">
          <span class="text-orange-400 text-5xl font-bold tracking-wide font-pix-primary">
            {{ lamportsToSol(Number(walletInfo.balance)) }}
          </span>
          <span class="text-gray-400 text-lg font-pix-secondary">SOL</span>
        </div>
        <div class="bg-gray-800/30 rounded-lg px-3 py-1">
          <div class="text-gray-300 text-xs font-pix-secondary">{{ walletInfo.network }}</div>
        </div>
      </div>

      <div class="flex gap-2 items-center">
        <div class="font-mono text-[0.95rem] tracking-[0.15em] opacity-90">
          {{ formatCardNumber(String(walletInfo.public_key)) }}
        </div>
        <button class="btn btn-xs bg-pix-800 hover:bg-pix-500 border-pix-800 text-white font-pix-secondary" 
          @click="copy(String(walletInfo.public_key))" > Copy </button>
      </div>
    </div>

    <!-- 删除钱包按钮 -->
    <div class="mb-4">
      <WalletDelModal :publicKey="String(walletInfo.public_key)" />
    </div>

    <!-- 交易历史卡片 -->
    <div class="bg-pix-200 rounded-2xl p-5 mb-4">
      <div class="flex items-center mb-4">
        <div class="w-8 h-8 bg-pix-800 rounded-lg mr-3 flex items-center justify-center">
          <div class="text-white text-xs font-pix-secondary">📋</div>
        </div>
        <div class="text-2xl font-pix-secondary text-pix-800">Transaction History</div>
      </div>

      <div class="overflow-x-auto">
        <table class="table table-xs table-fixed w-full">
          <thead>
            <tr class="bg-pix-300">
              <th class="w-8 text-pix-800 font-pix-secondary">#</th>
              <th class="truncate min-w-64 text-pix-800 font-pix-secondary">SIGNATURE</th>
              <th class="w-20 text-pix-800 font-pix-secondary">SLOT</th>
              <th class="w-40 text-pix-800 font-pix-secondary">BLOCK TIME</th>
              <th class="w-30 text-pix-800 font-pix-secondary">STATUS</th>
            </tr>
          </thead>
          <tbody>
            <tr 
              v-for="(item,index) in list" 
              :key="item.signature" 
              @click="selectOne(item)"
              class="hover:bg-pix-300/50 cursor-pointer transition-colors"
            >
              <td class="text-pix-800 font-pix-secondary">{{ index+1 }}</td>
              <td>
                <div class="flex gap-2 items-center">
                  <div class="truncate text-pix-800 font-mono text-xs">{{ item.signature }}</div>
                  <button 
                    class="btn btn-xs bg-pix-800 hover:bg-pix-500 border-pix-800 text-white font-pix-secondary" 
                    @click.stop="copy(String(item.signature))"
                  >
                    Copy
                  </button>
                </div>
              </td>
              <td class="text-pix-800 font-pix-secondary">{{ item.slot }}</td>
              <td class="text-pix-800 font-pix-secondary">
                <div class="flex items-center gap-2">
                  <div v-if="item.new_flag" class="badge badge-xs bg-green-500 text-white">new</div>
                  <span>{{ formatRelativeTime(item.block_time!) }}</span>
                </div>
              </td>
              <td :class="item.new_flag?'opacity-100':'opacity-80'">
                <TransferStatus :status="item.confirmation_status" />
              </td>
            </tr>
          </tbody>
        </table>

        <!-- 分页 -->
        <Pagination 
          :current-page="page" 
          :total="total" 
          :page-size="pageSize" 
          @change="handlePageChange" 
        />
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import API from "@/api";
import { useRoute } from "vue-router";
import { AccountHistory, MsgType, HistoryQuery } from "@/models";
import { formatRelativeTime, lamportsToSol, formatCardNumber } from "@/utils/common";
import { listen } from "@tauri-apps/api/event";
import { notify } from "@/utils/notify";
import TransferStatus from "../components/TransferStatus.vue";
import WalletDelModal from "../components/WalletDelModal.vue";
import ReturnButton from "@/components/ReturnButton.vue";
import Pagination from "@/components/Pagination.vue";

const route = useRoute();
const walletInfo = route.query;

var list = ref<AccountHistory[]>([]);

const page = ref(1);
const pageSize = ref(10);
const total = ref(0);

/** 获取查询参数 */
const getQueryParams = (): HistoryQuery => ({
  public_key: String(walletInfo.public_key),
  page: page.value,
  page_size: pageSize.value,
});

function dataInit() {
  if (!walletInfo || !walletInfo.public_key) return;

  API.WalletHistory(getQueryParams()).then((res) => {
    list.value = res.list || [];
    total.value = res.total;
  });
}

function handlePageChange(newPage: number) {
  page.value = newPage;
  dataInit();
}

async function copy(content: string) {
  try {
    await navigator.clipboard.writeText(content);
    notify.success("复制成功 ✅");
  } catch (err) {
    notify.error("Err");
    console.error(err);
  }
}

let unlisten: (() => void) | null = null;
onMounted(async () => {
  dataInit();
  unlisten = await listen<Array<AccountHistory>>(
    MsgType.REFRESH_HISTORY,
    (res) => {
      var newList = res.payload;
      newList.forEach((item) => {
        item.new_flag = true;
      });
      list.value = [...res.payload, ...list.value];
    }
  );
});

onUnmounted(() => {
  if (unlisten) unlisten();
});

function selectOne(item: AccountHistory) {
  API.TransferDetail(item.signature).then((res) => {
    console.log("======");
    console.log(res);
  });
}
</script>

<style scoped>
.wallet-info-card {
  background: linear-gradient(135deg, #1b2735 0%, #283e51 100%);
  border-radius: 18px;
  padding: 24px 32px;
  box-shadow:
    0 14px 30px rgba(0, 0, 0, 0.35),
    inset 0 1px 0 rgba(255, 255, 255, 0.15);
  color: #f5f7fa;
  transition-property: all;
  transition-duration: 200ms;
  transition-timing-function: ease-in-out;
}
</style>