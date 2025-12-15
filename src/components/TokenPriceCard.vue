<!-- TokenPriceCard.vue -->
<template>
  <div class="token-price-card">
    <div class="header">
      <span class="symbol">{{ formattedSymbol }}</span>
    </div>
    
    <div class="usd">
      {{ formattedPrice }}
    </div>
    
    <div class="footer">
      <div class="time">{{ formattedTime }}</div>
      <div class="raw" v-if="showRaw">
        原始: {{ usd.toLocaleString() }}e{{ expo }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

interface Props {
  symbol: string;
  usd: number; 
  expo: number;
  updated_at?: number;
  showRaw?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  updated_at: () => Math.floor(Date.now() / 1000),
  showRaw: false
});

// 计算实际价格
const actualPrice = computed(() => {
  return props.usd * Math.pow(10, props.expo);
});

// 格式化价格显示
const formattedPrice = computed(() => {
  const price = actualPrice.value;
  const symbol = props.symbol.toUpperCase();
  
  if (['USDT', 'USDC', 'DAI'].includes(symbol)) {
    return `$${price.toFixed(4)}`;
  }
  
  if (price >= 10000) {
    return `$${price.toLocaleString('en-US', { maximumFractionDigits: 0 })}`;
  } else if (price >= 1000) {
    return `$${price.toFixed(0)}`;
  } else if (price >= 1) {
    return `$${price.toFixed(2)}`;
  } else if (price >= 0.01) {
    return `$${price.toFixed(4)}`;
  } else {
    return `$${price.toFixed(6)}`;
  }
});

const formattedSymbol = computed(() => {
  const icons: Record<string, string> = {
    'BTC': '₿', 'ETH': 'Ξ', 'SOL': '◎',
    'USDT': '💵', 'USDC': '💵',
  };
  const icon = icons[props.symbol.toUpperCase()] || '💰';
  return `${icon} ${props.symbol}`;
});

const formattedTime = computed(() => {
  const now = Math.floor(Date.now() / 1000);
  const timestamp = props.updated_at || now;
  const diff = now - timestamp;
  
  if (diff < 60) return '刚刚';
  if (diff < 3600) return `${Math.floor(diff / 60)}分钟前`;
  if (diff < 86400) return `${Math.floor(diff / 3600)}小时前`;
  return `${Math.floor(diff / 86400)}天前`;
});

// 添加调试
console.log("TokenPriceCard 接收数据:", {
  symbol: props.symbol,
  usd: props.usd,
  expo: props.expo,
  actualPrice: actualPrice.value,
  formattedPrice: formattedPrice.value
});
</script>

<style scoped>
.token-price-card {
  background: linear-gradient(135deg, #2d3748 0%, #4a5568 100%);
  border-radius: 16px;
  padding: 20px;
  min-width: 180px;
  color: white;
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.3);
  transition: all 0.3s ease;
}

.token-price-card:hover {
  transform: translateY(-5px);
  box-shadow: 0 15px 35px rgba(0, 0, 0, 0.4);
}

.header {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 12px;
}

.usd {
  font-size: 28px;
  font-weight: 700;
  margin: 12px 0;
  color: #63b3ed;
}

.footer {
  font-size: 12px;
  color: #a0aec0;
}

.time {
  margin-bottom: 4px;
}

.raw {
  font-family: 'Courier New', monospace;
  font-size: 11px;
  opacity: 0.7;
}
</style>