import { invoke } from "@tauri-apps/api/core";
import { alert } from "@/utils/alert";

export async function requery<T>(cmd: string, args?: Record<string, any>): Promise<T | null> {
    try {
        const res = await invoke<T>(cmd, args)
        return res
    } catch (err) {
        alert.error(err as string)
        return null
    }
}
