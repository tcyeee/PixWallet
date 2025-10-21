<template>
  <button class="btn" @click="goHome()">Return</button>

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
    <div class="flex items-center">
      <span class="w-24 font-medium">Public Key:</span>
      <span class="truncate break-all max-w-full text-sm bg-base-200 px-2 py-1 rounded">{{ walletInfo.public_key }}</span>
    </div>
  </div>
  <div class="flex justify-end gap-2">
    <button class="btn btn-error" @click="deleteAccount()">删除账户</button>
  </div>
</template>
<script setup lang="ts">
import { ref } from "vue";
import { useNav } from "@/hooks/useNav";
import API from "@/api";

const { goHome, pageQuery } = useNav();
const walletInfo = pageQuery();

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