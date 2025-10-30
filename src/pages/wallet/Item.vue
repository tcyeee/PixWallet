<template>
  <button class="btn" @click="NAV.Home()">Return</button>

  <div class="card bg-base-100 w-96 shadow-sm bg-gradient-to-r from-[#ff7e5f] to-[#feb47b] mt-10">
    <div class="card-body">
      <div class="flex gap-2 mb-5 items-center">
        <div class="text-4xl text-green-800 font-bold">{{ walletInfo.balance }}</div>
        <div>
          <div class="text-green-900">Lamport</div>
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
    <button class="btn btn-error" @click="deleteAccount()">删除账户</button>
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
          <th class="w-35">BLOCK TIME</th>
          <th class="w-30">STATUS</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(item,index) in list" :key="item.signature">
          <th>{{ index+1 }}</th>
          <th class=" ">
            <div class="flex gap-2">
              <div class="truncate text-gray-500">{{ item.signature }}</div>
              <button class="btn btn-neutral btn-dash btn-xs" @click="copy(String(item.signature))">Copy</button>
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
  </div>
</template>
<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import API from "@/api";
import { useRoute } from "vue-router";
import NAV from "@/router";
import { AccountHistory, MsgType } from "@/models";
import { formatRelativeTime } from "@/utils/common";
import { listen } from "@tauri-apps/api/event";
import { notify } from "@/utils/notify";
import TransferStatus from "../components/TransferStatus.vue";

const route = useRoute();
const walletInfo = route.query;

onMounted(() => {
  dataInit();
});

var list = ref<AccountHistory[]>([]);
function dataInit() {
  if (!walletInfo || !walletInfo.public_key) return;
  API.WalletHistory(String(walletInfo.public_key)).then((res) => {
    list.value = res || [];
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

function deleteAccount() {
  API.WalletDel({ publicKey: walletInfo?.public_key });
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
</script>