import { listen } from '@tauri-apps/api/event'

export async function setupTauriListener() {
    await listen('msg', (event) => {
        console.log("===========");
        console.log(event);
        console.log("===========");
        // const payload = event.payload as { type: string; content: string }
        // if (payload.type === 'alert') alert(payload.content)
        // if (payload.type === 'message') {
        //     // TODO
        // }
    })
}