<template>
  <div class="flex">
    <div @click="NAV.Back()" class="bg-pix-500 rounded-[20px] p-[5px] shadow-lg">
      <div class="px-3 font-pix-secondary text-3xl">Return</div>
    </div>
  </div>
</template>
<script setup lang="ts">
import NAV from "@/router";
</script>