<template>
  <button class="btn" @click="NAV.Home()">Return</button>

  <hr class="my-5">
  <select class="select">
    <option disabled selected>Pick a wallet</option>
    <option>Crimson</option>
    <option>Amber</option>
    <option>Velvet</option>
  </select>

</template>
<script setup lang="ts">
import NAV from "@/router";
import { onMounted } from "vue";

onMounted(() => {
  console.log("----");
});
</script>