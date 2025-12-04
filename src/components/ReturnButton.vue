<template>
  <div class="mb-4 flex items-center">
    <div @click="NAV.Back()" class="bg-pix-200 rounded-2xl p-3 cursor-pointer hover:opacity-80 transition-opacity duration-300 inline-flex items-center">
      <span class="icon--return w-6 h-6 bg-pix-800"></span>
      <div class="font-pix-secondary text-2xl text-pix-800 mt-1 ml-1">Return</div>
    </div>
    <div v-if="title" class="text-6xl font-bold text-pix-800 font-pix-primary ml-4">{{ title }}</div>
  </div>
</template>
<script setup lang="ts">
import NAV from "@/router";
defineProps<{title?: string|undefined }>();
</script>
<style scoped>  
@import "@/assets/icon.scss";
</style>