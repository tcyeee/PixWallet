<template>
  <button class="btn" @click="NAV.Home()">Return</button>

  <div class="card bg-base-100 w-96 shadow-sm bg-gradient-to-r from-[#ff7e5f] to-[#feb47b] mt-10">
    <div class="card-body">
      <div>
        <h2 class="card-title">{{ walletInfo.alias }}</h2>
        <div>{{ walletInfo.balance }} SOL</div>
      </div>
      <span class="truncate break-all max-w-full text-sm py-1 rounded bg-gray-500/10 text-gray-600 pl-1">{{ walletInfo.public_key }}</span>
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
    <div class="flex items-center">
      <span class="w-24 font-medium">Network:</span>
      <span class="truncate">{{ walletInfo.network }}</span>
    </div>
  </div>

  <hr class="my-5" />

  <div class="overflow-x-auto">
    <table class="table table-xs">
      <thead>
        <tr>
          <th>SIGNATURE</th>
          <th>SLOT</th>
          <th>BLOCK TIME</th>
          <th>STATUS</th>
          <th>REMARK</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="item in list" :key="item.signature">
          <th>{{ item.signature }}</th>
          <th>{{ item.slot }}</th>
          <th>{{ item.block_time }}</th>
          <th>{{ item.confirmation_status }}</th>
          <th>{{ item.remark || 'None'}}</th>
        </tr>
      </tbody>
    </table>
  </div>
</template>
<script setup lang="ts">
import { ref, onMounted } from "vue";
import API from "@/api";
import { useRoute } from "vue-router";
import NAV from "@/router";
import { AccountHistory } from "@/models";

const route = useRoute();
const walletInfo = route.query;

onMounted(() => {
  dataInit();
});

var list = ref<AccountHistory[]>([]);
function dataInit() {
  if (!walletInfo || !walletInfo.public_key) return;
  API.WalletHistory(String(walletInfo.public_key)).then((res) => {
    console.log(res);
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
</script>