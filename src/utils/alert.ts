import { useAlertStore } from '@/stores/alert'

const alert = {
    success(msg: string) {
        useAlertStore().show('success', msg, false)
    },
    error(msg: string) {
        useAlertStore().show('error', msg, false)
    },
    warning(msg: string) {
        useAlertStore().show('warning', msg, false)
    },
    info(msg: string) {
        useAlertStore().show('info', msg, false)
    },
    clear() {
        useAlertStore().clearAll()
    },
}

export default alert