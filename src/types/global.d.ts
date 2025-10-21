export { }

declare global {
    interface Window {
        Notify: typeof import('@/utils/notify').notify
        Alert: typeof import('@/utils/alert').alert
    }
}