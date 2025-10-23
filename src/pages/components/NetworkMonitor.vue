<template>
  <div class="flex items-center gap-2">
    <div class="inline-grid *:[grid-area:1/1]">
      <div v-if="networkFlag" class="status status-lg animate-ping" :class="[statusColor]"></div>
      <div class="status status-lg" :class="[statusColor]"></div>
    </div>
    <div class="text-green-400 text-sm" :class="[textColor]">{{ network?.ping }}</div>
  </div>
</template>
<script setup lang="ts">
import { MsgType, NetworkHealth, NetworkStatus } from "@/models";
import { listen } from "@tauri-apps/api/event";
import { ref, computed, watch } from "vue";

const networkFlag = computed(() => {
  return network.value && network.value?.status != NetworkHealth.LOST;
});

const network = ref<NetworkStatus>();
listen<NetworkStatus>(MsgType.PING, (event) => {
  network.value = event.payload;
});

const statusColor = ref("");
const textColor = ref("text-red-400");
watch(network, (newVal) => {
  if (!newVal) return;
  switch (newVal.status) {
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
});
</script>