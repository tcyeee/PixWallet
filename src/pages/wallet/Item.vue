<template>
  <ReturnButton />
  <div class="h-screen overflow-y-scroll pb-[30vh]">
    <div class="card bg-base-100 w-96 shadow-sm bg-gradient-to-r from-[#ff7e5f] to-[#feb47b] mt-10">
      <div class="card-body">
        <div class="flex gap-2 mb-5 items-center">
          <div class="text-5xl text-green-800 font-bold">${{ lamportsToSol(Number(walletInfo.balance)) }}</div>
          <div>
            <div class="text-green-900">Sol</div>
            <div class="bg-green-800/20 rounded px-2 text-xs text-green-700">{{ walletInfo.network }}</div>
          </div>
        </div>

        <div class="text-lg">{{ walletInfo.alias }}</div>

        <div class="flex gap-2 items-center">
          <div class=" truncate break-all text-sm  rounded bg-gray-900/10 text-gray-600 p-1">{{ walletInfo.public_key }}</div>
          <button class="btn btn-neutral btn-dash btn-xs" @click="copy(String(walletInfo.public_key))">Copy</button>
        </div>
      </div>
    </div>
    <div class="mt-3 flex mb-10">
      <WalletDelModal :publicKey="String(walletInfo.public_key)" />
    </div>

    <div class="space-y-2 mb-4">
      <div class="flex items-center">
        <span class="w-24 font-medium">Alias:</span>
        <div class="flex gap-2 w-full">
          <input type="text" class="input" placeholder="Type here" v-model="alias" />
          <button class="btn" @click="changeAlias()">修改</button>
        </div>
      </div>
    </div>

    <div class="overflow-x-auto">
      <table class="table table-xs table-fixed">
        <thead>
          <tr>
            <th class="w-8"></th>
            <th class="truncate min-w-64">SIGNATURE</th>
            <th class="w-20">SLOT</th>
            <th class="w-40">BLOCK TIME</th>
            <th class="w-30">STATUS</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(item,index) in list" :key="item.signature" @click="selectOne(item)">
            <th>{{ index+1 }}</th>
            <th class=" ">
              <div class="flex gap-2">
                <div class="truncate text-gray-500">{{ item.signature }}</div>
                <button class="btn btn-neutral btn-dash btn-xs" @click.stop="copy(String(item.signature))">Copy</button>
              </div>
            </th>
            <th>{{ item.slot }}</th>
            <th>
              <div v-if="item.new_flag" class="badge badge-xs badge-success">new</div>
              {{ formatRelativeTime(item.block_time!) }}
            </th>
            <th :class="item.new_flag?'opacity-100':'opacity-80'">
              <TransferStatus :status="item.confirmation_status" />
            </th>
          </tr>
        </tbody>
      </table>
      <div class="flex justify-between mt-4">
      <button class="btn" :disabled="page === 1" @click="prevPage">上一页</button>
      <span>第 {{ page }} 页 / 共 {{ totalPages }} 页</span>
      <button class="btn" :disabled="page === totalPages" @click="nextPage">下一页</button>
    </div>

    </div>
  </div>
</template>
<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed} from "vue";
import API from "@/api";
import { useRoute } from "vue-router";
import { AccountHistory, MsgType} from "@/models";
import { formatRelativeTime, lamportsToSol } from "@/utils/common";
import { listen } from "@tauri-apps/api/event";
import { notify } from "@/utils/notify";
import TransferStatus from "../components/TransferStatus.vue";
import WalletDelModal from "../components/WalletDelModal.vue";
import ReturnButton from "@/components/ReturnButton.vue";

const route = useRoute();
const walletInfo = route.query;



onMounted(() => {
  dataInit();
});

var list = ref<AccountHistory[]>([]);
const page = ref(1);
const pageSize = ref(30);
const total = ref(0);
const totalPages = computed(() => Math.ceil(total.value / pageSize.value));


// function dataInit() {
//   if (!walletInfo || !walletInfo.public_key) return;
//   API.WalletHistory(String(walletInfo.public_key)).then((res) => {
//     list.value = res || [];
//   });
// }
function dataInit() {
  if (!walletInfo || !walletInfo.public_key) return;
  

  API.WalletHistory(String(walletInfo.public_key), page.value, pageSize.value).then((res) => {
    list.value = res.list || [];
    total.value = res.total;
  });
}


const alias = ref("");
function changeAlias() {
  const params = {
    publicKey: walletInfo?.public_key,
    newAlias: alias.value,
  };
  API.WalletAliasUpdate(params);
}

function nextPage() {
  if (page.value < totalPages.value) {
    page.value++;
    dataInit();
  }
}

function prevPage() {
  if (page.value > 1) {
    page.value--;
    dataInit();
  }
}


async function copy(content: string) {
  try {
    await navigator.clipboard.writeText(content);
    notify.success("复制成功 ✅");
    alert();
  } catch (err) {
    notify.error("Err");
    console.error(err);
  }
}

let unlisten: (() => void) | null = null;
onMounted(async () => {
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