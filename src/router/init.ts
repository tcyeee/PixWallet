import { createRouter, createWebHashHistory } from 'vue-router'
import Home from '@/pages/Home.vue'
import Wallet from '@/pages/wallet/index.vue'
import Item from '@/pages/wallet/Item.vue'
import Transfer from '@/pages/transfer/Transfer.vue'

// 路由规则
const routes = [
    { path: '/', name: 'home', component: Home },
    { path: '/wallet', name: 'wallet', component: Wallet },
    { path: '/wallet-item', name: 'wallet-item', component: Item },
    { path: '/transfer', name: 'transfer', component: Transfer }
]

// 创建路由实例
const router = createRouter({
    history: createWebHashHistory(),
    routes,
})

export default router
