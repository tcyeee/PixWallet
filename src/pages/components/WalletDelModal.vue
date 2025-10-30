<template>
  <div>
    <!-- 打开弹窗的按钮 -->
    <button class="btn btn-error" @click="openModal()">删除账户</button>

    <!-- DaisyUI 弹窗 -->
    <dialog ref="modal" class="modal modal-bottom sm:modal-middle">
      <div class="modal-box">
        <h3 class="text-lg font-bold">删除账户</h3>
        <p class="py-4">是否确认删除该账户? 此操作不可逆.</p>
        <p>{{ publicKey }}</p>
        <div class="modal-action">
          <button class="btn btn-error" :disabled="loading" @click="confirm()">
            <span v-if="loading" class="loading loading-spinner"></span>
            Confirm
          </button>
          <button class="btn" @click="closeModal()">Close</button>
        </div>
      </div>
    </dialog>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import API from "@/api";
import { notify } from "@/utils/notify";
import NAV from "@/router";

const props = defineProps<{
  publicKey: String;
}>();

const modal = ref<HTMLDialogElement | null>(null);
function openModal() {
  modal.value?.showModal();
}

function closeModal() {
  modal.value?.close();
}

const loading = ref(false);
async function confirm() {
  const publicKey = String(props.publicKey);
  console.log(publicKey);
  loading.value = true;
  await API.WalletDel({ publicKey: publicKey });
  loading.value = false;
  notify.success("钱包删除成功");
  modal.value?.close();
  NAV.Back();
}
</script>