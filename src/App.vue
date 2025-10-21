<template>
  <main class="p-5 select-none">
    <router-view></router-view>
  </main>

  <GlobalToast />
  <GlobalAlert />
</template>

<script setup lang="ts">
import GlobalToast from "@/components/GlobalToast.vue";
import GlobalAlert from "@/components/GlobalAlert.vue";
</script>