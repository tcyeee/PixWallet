<template>
  <div class="flex items-center gap-2">
    <div class="inline-grid *:[grid-area:1/1]">
      <div v-if="userStore.network?.status && userStore.network.status != NetworkHealth.LOST" class="status status-lg animate-ping" :class="[statusColor]"></div>
      <div class="status status-lg" :class="[statusColor]"></div>
    </div>
    <div class="text-gray-400 text-sm" :class="[textColor]">
      <div v-if="userStore.network">{{ userStore.network.ping + 'ms' }}</div>
      <div v-else class="text-gray-400">loading...</div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { NetworkHealth, NetworkStatus } from "@/models";
import { ref, watch, onMounted } from "vue";
import { useUserStore } from "@/stores/user";

const userStore = useUserStore();
watch(
  () => userStore.network,
  (network) => setClass(network),
  { deep: true }
);

onMounted(() => setClass(userStore.network));

const statusColor = ref("");
const textColor = ref("text-red-400");
function setClass(network: NetworkStatus) {
  switch (network.status) {
    case NetworkHealth.GOOD:
      statusColor.value = "status-success";
      textColor.value = "text-green-400";
      break;
    case NetworkHealth.POOR:
      statusColor.value = "status-warning";
      textColor.value = "text-orange-400";
      break;
    case NetworkHealth.LOST:
    default:
      statusColor.value = "";
      textColor.value = "text-red-400";
      break;
  }
}
</script>