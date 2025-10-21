<template>
  <main class="p-5 overflow-y-hidden overflow-x-hidden max-w-full select-none" data-theme="emerald">
    <Wallet />
  </main>

  <GlobalToast />
  <GlobalAlert />
</template>

<script setup lang="ts">
import GlobalToast from "@/components/GlobalToast.vue";
import GlobalAlert from "@/components/GlobalAlert.vue";
import Wallet from "./pages/wallet/wallet.vue";
</script>