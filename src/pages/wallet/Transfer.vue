<template>
  <div class="text-9xl font-bold text-gray-300">Transfer</div>
  <button class="btn" @click="NAV.Home()">Return</button>

  <fieldset class="fieldset mt-10">
    <legend class="fieldset-legend">Select a payment account</legend>
    <select class="select" v-model="paymentAccount">
      <option disabled selected>{{ walletSelectTip }}</option>
      <option v-for="item in userStore.wallets" :key="item.public_key" :value="item.public_key">
        <span>{{ item.alias || 'None' }}</span>
        <span>&emsp;</span>
        <span class="text-so text-gray-400 bg-gray-200 border-lg">({{ item.public_key.slice(0,10) + '...' }})</span>
      </option>
    </select>
    <span v-if="paymentAccount != walletSelectTip" class="label">{{ paymentAccount }}</span>
  </fieldset>

  <fieldset class="fieldset">
    <legend class="fieldset-legend">Enter recipient account</legend>
    <input v-model="recipientAccount" type="text" class="input" placeholder="Type here" title="Please enter a valid Solana address (43–44 Base58 characters)" pattern="^[1-9A-HJ-NP-Za-km-z]{43,44}$" required />
  </fieldset>
  <button class="btn btn-primary mt-3" @click="transfer()">Transfer</button>

  <hr class="my-10">
  <div>{{ recipientAccount }}</div>
</template>

<script setup lang="ts">
import NAV from "@/router";
import { useUserStore } from "@/stores/user";
import { ref } from "vue";
const userStore = useUserStore();

const walletSelectTip = "Pickup wallet";
const paymentAccount = ref<String>(walletSelectTip);
const recipientAccount = ref<String>();

function transfer() {
  console.log("=====");
}
</script>