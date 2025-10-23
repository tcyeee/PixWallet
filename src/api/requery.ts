import { invoke } from "@tauri-apps/api/core";
import { alert } from "@/utils/alert";

export async function requery<T>(cmd: string, args?: Record<string, any>): Promise<T> {
    try {
        return await invoke<T>(cmd, args)
    } catch (err) {
        alert.error(err as string)
        throw err
    }
}
