import { defineStore } from 'pinia'

export const useNotificationStore = defineStore('notification', {
    state: () => ({
        messages: [] as {
            id: number
            type: 'success' | 'error' | 'warning' | 'info'
            text: string
        }[],
        maxMessages: 4
    }),
    actions: {
        push(type: 'success' | 'error' | 'warning' | 'info', text: string) {
            const id = Date.now()

            // 如果当前条数达到 maxAlerts，删除最早的一条
            if (this.messages.length >= this.maxMessages) {
                this.messages.shift()
            }

            this.messages.push({ id, type, text })

            // 自动 3 秒后消失
            setTimeout(() => this.remove(id), 3000)
        },
        remove(id: number) {
            this.messages = this.messages.filter(m => m.id !== id)
        },
    },
})