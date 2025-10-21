import { listen } from '@tauri-apps/api/event'

// 用于后端自动触发报错/提示信息
export async function setupTauriListener() {
    await listen('backend-alert', (event) => {
        const payload = event.payload as { type: string; content: string }
        if (payload.type === 'alert') alert(payload.content)
        if (payload.type === 'message') {
            // TODO
        }
    })
}