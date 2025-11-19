<template>
  <div>
    <div class="text-9xl font-bold text-pix-800 font-pix-primary">PixWallet</div>

    <div class="mt-[30px] flex justify-between font-pix-primary text-6xl text-pix-500 text-stroke">
      <div class="flex flex-col items-center bg-orange-300 rounded-2xl p-5 inner-shadow">
        <div class="w-[200px] h-[200px] flex items-center justify-center">
          <img src="/src/assets/pic/wallet.png" class="w-[170px]">
        </div>
        <div>wallet</div>
      </div>

      <div class="flex flex-col items-center bg-orange-300 rounded-2xl p-5 inner-shadow">
        <div class="w-[200px] h-[200px] flex items-center justify-center">
          <img src="/src/assets/pic/house.png" class="w-[150px]">
        </div>
        <div>transfer</div>
      </div>

      <div class="flex flex-col items-center bg-orange-300 rounded-2xl p-5 inner-shadow">
        <div class="w-[200px] h-[200px] flex items-center justify-center">
          <img src="/src/assets/pic/box.png" class="w-[150px]">
        </div>
        <div>receive</div>
      </div>
    </div>
    <div class="fixed bottom-3 left-0 w-full px-[20px] font-pix-primary justify-center flex">
      <div class="flex items-center">
        <div class="mr-3 text-gray-500">
          <span>This is a development version and may contain some bugs. </span>
          <span> ⋅ </span>
          <a class="underline" target="_blank" href="https://github.com/tcyeee/PixWallet">Github</a>
        </div>
        <NetworkMonitor />
      </div>
    </div>
    <!-- <WalletList class="mt-5" /> -->
  </div>
</template>
<script setup>
import WalletList from "@/pages/wallet/List.vue";
import NetworkMonitor from "./components/NetworkMonitor.vue";
</script>

<style scoped>
.text-stroke {
  -webkit-text-stroke: 2px #b56035;
}

.inner-shadow {
  box-shadow: inset 0px 0px 10px rgba(0, 0, 0, 0.5);
}
</style>