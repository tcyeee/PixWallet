import { useRouter, useRoute } from 'vue-router'

export function useNav() {
    const router = useRouter()
    const route = useRoute()

    const goHome = () => router.push('/')
    const goBack = () => router.back()
    const goTo = (name: string, params?: Record<string, any>) => router.push({ name, query: params || {} })
    const pageQuery = () => route.query

    return { router, pageQuery, goHome, goBack, goTo }
}