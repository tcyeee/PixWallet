import { defineStore } from 'pinia'

export const useNotificationStore = defineStore('notification', {
    state: () => ({
        messages: [] as {
            id: number
            type: 'success' | 'error' | 'warning' | 'info'
            text: string
        }[],
    }),
    actions: {
        push(type: 'success' | 'error' | 'warning' | 'info', text: string) {
            const id = Date.now()
            this.messages.push({ id, type, text })

            // 自动 3 秒后消失
            setTimeout(() => this.remove(id), 3000)
        },
        remove(id: number) {
            this.messages = this.messages.filter(m => m.id !== id)
        },
    },
})