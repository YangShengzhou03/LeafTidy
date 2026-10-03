import { onMounted, onUnmounted } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'

/**
 * 订阅后端进度事件：组件挂载时自动注册，卸载时自动清理。
 */
export function useProgressListener<T>(
  event: string,
  handler: (payload: T) => void,
): void {
  let unlisten: UnlistenFn | null = null

  onMounted(async () => {
    unlisten = await listen<T>(event, (e) => handler(e.payload))
  })

  onUnmounted(() => {
    unlisten?.()
    unlisten = null
  })
}
