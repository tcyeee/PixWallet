<template>
  <div class="flex justify-center items-center gap-4 mt-4">
    <div class="join">
      <button 
        class="join-item btn btn-sm bg-pix-800 hover:bg-pix-500  text-white font-pix-secondary border-none" 
        :disabled="currentPage === 1" 
        @click="handlePrev"
      >
        «
      </button>
      <button 
        class="join-item btn btn-sm bg-pix-800 text-white font-pix-secondary" 
        disabled
      >
         {{ currentPage }} / {{ totalPages }} 
      </button>
      <button 
        class="join-item btn btn-sm bg-pix-800 hover:bg-pix-500  text-white font-pix-secondary border-none" 
        :disabled="currentPage === totalPages || totalPages === 0" 
        @click="handleNext"
      >
        »
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

interface Props {
  /** 当前页码 */
  currentPage: number;
  /** 数据总数 */
  total: number;
  /** 每页大小，默认为 10 */
  pageSize?: number;
}

const props = withDefaults(defineProps<Props>(), {
  pageSize: 10,
});

const emit = defineEmits<{
  /** 分页变化时触发，参数为新页码 */
  change: [page: number];
}>();

/** 计算总页数 */
const totalPages = computed(() => {
  if (props.total === 0) return 0;
  return Math.ceil(props.total / props.pageSize);
});

function handlePrev() {
  if (props.currentPage > 1) {
    const newPage = props.currentPage - 1;
    emit('change', newPage);
  }
}

function handleNext() {
  if (props.currentPage < totalPages.value && totalPages.value > 0) {
    const newPage = props.currentPage + 1;
    emit('change', newPage);
  }
}
</script>

