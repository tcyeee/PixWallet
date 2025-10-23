<template>
  <div>
    <div class="text-8xl font-bold text-gray-300">HOME</div>
    <NetworkMonitor />
    <WalletList class="mt-5" />
  </div>
</template>
<script setup>
import WalletList from "@/pages/wallet/List.vue";
import NetworkMonitor from "./components/NetworkMonitor.vue";
</script>