import { createRouter, createWebHashHistory } from 'vue-router'
import Home from '@/pages/Home.vue'
import Item from '@/pages/wallet/Item.vue'

// 路由规则
const routes = [
    { path: '/', name: 'home', component: Home },
    { path: '/wallet', name: 'wallet', component: Item }
]

// 创建路由实例
const router = createRouter({
    history: createWebHashHistory(),
    routes,
})

export default router