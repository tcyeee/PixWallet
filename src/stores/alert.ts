import { defineStore } from 'pinia'

export const useAlertStore = defineStore('alert', {
    state: () => ({
        alerts: [] as {
            id: number
            type: 'success' | 'error' | 'warning' | 'info'
            text: string
            closable?: boolean
        }[],
        maxAlerts: 5,
    }),
    actions: {
        show(type: 'success' | 'error' | 'warning' | 'info', text: string, closable = false) {
            const id = Date.now()

            // 如果当前条数达到 maxAlerts，删除最早的一条
            if (this.alerts.length >= this.maxAlerts) {
                this.alerts.shift()
            }

            this.alerts.push({ id, type, text, closable })

            // 自动 3 秒后消失
            setTimeout(() => this.remove(id), 3000)
        },
        remove(id: number) {
            this.alerts = this.alerts.filter(a => a.id !== id)
        },
        clearAll() {
            this.alerts = []
        },
    },
})