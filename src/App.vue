<template>
  <div class="select-none h-screen rounded-[15px] bg-pix-500 p-[10px]">
    <div class="bg-pix-300 rounded-[10px] h-full p-5">
      <main class="">
        <router-view></router-view>
      </main>
      <GlobalToast />
      <GlobalAlert />
    </div>
  </div>
</template>

<script setup lang="ts">
import GlobalToast from "@/components/GlobalToast.vue";
import GlobalAlert from "@/components/GlobalAlert.vue";
</script>