<template>
  <div class="fixed top-5 right-5 flex flex-col gap-3 z-50">
    <transition-group name="fade" tag="div">
      <div v-for="msg in store.messages" :key="msg.id" :class="toastClass(msg.type)" class="alert shadow-lg w-80 mt-2">
        <span>{{ msg.text }}</span>
      </div>
    </transition-group>
  </div>
</template>

<script setup lang="ts">
import { useNotificationStore } from "@/stores/notification";

const store = useNotificationStore();

const toastClass = (type: string) => {
  switch (type) {
    case "success":
      return "alert-success";
    case "error":
      return "alert-error";
    case "warning":
      return "alert-warning";
    default:
      return "alert-info";
  }
};
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: all 0.3s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}
</style>