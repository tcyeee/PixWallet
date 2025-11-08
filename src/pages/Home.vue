<template>
  <div>
    <div class="text-9xl font-bold text-gray-300">PixWallet.</div>
    <NetworkMonitor />
    <WalletList class="mt-5" />
  </div>
</template>
<script setup>
import WalletList from "@/pages/wallet/List.vue";
import NetworkMonitor from "./components/NetworkMonitor.vue";
</script>