<template>
  <ReturnButton />
  <WalletList class="mt-5" />
</template>
<script setup>
import WalletList from "@/pages/wallet/List.vue";
import ReturnButton from "@/components/ReturnButton.vue";
import NAV from "@/router";
import { useUserStore } from "@/stores/user";
import { onMounted } from "vue";

const userStore = useUserStore();

onMounted(async () => {
  userStore.updateWallets();
});
</script>