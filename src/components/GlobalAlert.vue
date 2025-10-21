<template>
  <div class="fixed top-4 left-1/2 transform -translate-x-1/2 z-50 w-auto max-w-[90%] min-w-[300px]">
    <!-- 使用 Vue 的 transition-group 实现淡入淡出 -->
    <transition-group name="fade" tag="div">
      <div v-for="a in store.alerts" :key="a.id" :class="['alert', typeClass(a.type), 'shadow-lg transition-all duration-300 mt-2']">
        <div class="flex items-center justify-between w-full">
          <span>{{ a.text }}</span>
          <button v-if="a.closable" class="btn btn-ghost btn-xs" @click="store.remove(a.id)">
            ✕
          </button>
        </div>
      </div>
    </transition-group>
  </div>
</template>

<script setup lang="ts">
import { useAlertStore } from "@/stores/alert";
const store = useAlertStore();

const typeClass = (type: string) => {
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
/* 淡入淡出动画 */
.fade-enter-active,
.fade-leave-active {
  transition: all 0.35s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}
</style>