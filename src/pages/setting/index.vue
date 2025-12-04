<template>
<div class="flex items-start gap-4">

  <ReturnButton/>

  <div class="h-screen overflow-y-scroll pb-[20vh] flex-1 hide-scrollbar">
    <!-- 网络设置 -->
    <div class="bg-pix-200 rounded-2xl p-5 mb-4 ">
      <div class="flex items-center mb-3">
        <div class="w-8 h-8 bg-pix-800 rounded-lg mr-3 flex items-center justify-center">
          <div class="w-4 h-4 bg-pix-300 rounded-full"></div>
        </div>
        <div class="text-2xl font-pix-secondary text-pix-800">Network</div>
      </div>
      <fieldset class="fieldset">
        <legend class="fieldset-legend">RPC Endpoint</legend>
        <select class="select w-full bg-pix-300 border-pix-800 text-pix-800 font-pix-secondary">
          <option selected>Mainnet (Default)</option>
          <option>Devnet</option>
          <option>Testnet</option>
          <option>Custom RPC</option>
        </select>
      </fieldset>
      <div class="mt-3 text-sm text-gray-600 font-pix-secondary">
        Current: https://api.mainnet-beta.solana.com
      </div>
    </div>

    <!-- 语言设置 -->
    <div class="bg-pix-200 rounded-2xl p-5 mb-4 ">
      <div class="flex items-center mb-3">
        <div class="w-8 h-8 bg-pix-800 rounded-lg mr-3 flex items-center justify-center">
          <div class="text-white text-xs font-pix-secondary">语言</div>
        </div>
        <div class="text-2xl font-pix-secondary text-pix-800">Language</div>
      </div>
      <select class="select w-full bg-pix-300 border-pix-800 text-pix-800 font-pix-secondary">
        <option selected>简体中文</option>
        <option>English</option>
      </select>
    </div>

    <!-- 钱包管理 -->
    <div class="bg-pix-200 rounded-2xl p-5 mb-4 ">
      <div class="flex items-center mb-3">
        <div class="w-8 h-8 bg-pix-800 rounded-lg mr-3 flex items-center justify-center">
          <div class="w-4 h-4 border-2 border-pix-300"></div>
        </div>
        <div class="text-2xl font-pix-secondary text-pix-800">Wallet Management</div>
      </div>
      <div class="space-y-2">
        <button class="btn btn-primary w-full font-pix-secondary bg-pix-800 hover:bg-pix-500 border-pix-800">
          Export Wallet
        </button>
        <button class="btn btn-outline w-full font-pix-secondary border-pix-800 text-pix-800 hover:bg-pix-300">
          Import Wallet
        </button>
      </div>
    </div>

    <!-- 关于 -->
    <div class="bg-pix-200 rounded-2xl p-5 mb-4 ">
      <div class="flex items-center mb-3">
        <div class="w-8 h-8 bg-pix-800 rounded-lg mr-3 flex items-center justify-center">
          <div class="text-white text-xs font-pix-secondary">i</div>
        </div>
        <div class="text-2xl font-pix-secondary text-pix-800">About</div>
      </div>
      <div class="space-y-2 text-pix-800 font-pix-secondary">
        <div class="flex justify-between">
          <span>Version</span>
          <span>0.1.0</span>
        </div>
        <div class="flex justify-between">
          <span>Build</span>
          <span>Development</span>
        </div>
        <div class="pt-2 border-t border-pix-800 flex justify-between">
          <a href="https://github.com/tcyeee/PixWallet" target="_blank" class="text-pix-800 hover:underline">
            GitHub Repository
          </a>
          <a href="https://github.com/tcyeee/PixWallet" target="_blank" class="text-pix-800 hover:underline">
            https://github.com/tcyeee/PixWallet
          </a>
        </div>
      </div>
    </div>
  </div>
</div>

</template>
<script setup>
import ReturnButton from "@/components/ReturnButton.vue";
</script>

<style scoped>
@import "@/assets/icon.scss";
</style>
