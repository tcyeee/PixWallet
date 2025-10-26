<template>
  <div class="text-9xl font-bold text-gray-300">Transfer</div>
  <button class="btn" @click="NAV.Home()">Return</button>

  <fieldset class="fieldset mt-10">
    <legend class="fieldset-legend">Select a payment account</legend>
    <select class="select" v-model="params.paying">
      <option disabled selected>{{ walletSelectTip }}</option>
      <option v-for="item in userStore.wallets" :key="item.public_key" :value="item.public_key">
        {{( item.alias || 'None') + '&ensp;' + item.public_key.slice(0,10) + '...' }}
      </option>
    </select>
    <span v-if="params.paying != walletSelectTip" class="label">{{ params.paying }}</span>
  </fieldset>

  <fieldset class="fieldset">
    <legend class="fieldset-legend">Enter recipient account</legend>
    <input v-model="params.receiving" type="text" class="input validator" placeholder="Type here" pattern="^[1-9A-HJ-NP-Za-km-z]{43,44}$" required />
    <p class="validator-hint">Please enter a valid Solana address (43–44 Base58 characters)</p>
  </fieldset>

  <fieldset class="fieldset">
    <legend class="fieldset-legend">Enter transfer amount</legend>
    <input type="number" class="input validator" placeholder="Type a number" v-model="params.amount" />
    <p class="validator-hint">Must be between be 1 to 10</p>
  </fieldset>

  <button class="btn btn-primary mt-3" @click="transfer()">Transfer</button>

  <hr class="my-10">
  <div>{{ params.receiving }}</div>
</template>

<script setup lang="ts">
import NAV from "@/router";
import { useUserStore } from "@/stores/user";
import { reactive } from "vue";
import API from "@/api";
import { TransferParams } from "@/models";

const userStore = useUserStore();

const walletSelectTip = "Pickup wallet";

const params = reactive<TransferParams>({
  paying: "",
  receiving: "",
  amount: 0,
});

function transfer() {
  API.Transfer(params);
}
</script>