import { useNotificationStore } from '@/stores/notification'

const notify = {
    success(msg: string) {
        useNotificationStore().push('success', msg)
    },
    error(msg: string) {
        useNotificationStore().push('error', msg)
    },
    warning(msg: string) {
        useNotificationStore().push('warning', msg)
    },
    info(msg: string) {
        useNotificationStore().push('info', msg)
    },
}

export default notify