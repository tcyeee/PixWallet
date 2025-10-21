<template>
  <button class="btn" @click="to()">Return</button>
</template>
<script setup lang="ts">
import NAV from "@/router";

function to() {
  NAV.Home();
}
</script>