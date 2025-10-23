import router from './init'

export default {
    Home: () => router.push('/'),
    Back: () => router.back(),
    GoTo: (name: string, params?: Record<string, any>) => router.push({ name, query: params || {} })
}
