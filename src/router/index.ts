import { createRouter, createWebHistory } from 'vue-router'
import Home from '@/pages/home.vue'

// 路由规则
const routes = [
    {
        path: '/',       // URL 地址
        name: 'Home',    // 路由名称
        component: Home, // 对应组件
    },
]

// 创建路由实例
const router = createRouter({
    history: createWebHistory(),
    routes,
})

export default router