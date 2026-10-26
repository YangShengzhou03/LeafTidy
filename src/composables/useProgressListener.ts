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
  let disposed = false

  onMounted(async () => {
    const fn = await listen<T>(event, (e) => handler(e.payload))
    // 监听注册完成前组件已卸载，立即注销，避免泄漏
    if (disposed) fn()
    else unlisten = fn
  })

  onUnmounted(() => {
    disposed = true
    unlisten?.()
    unlisten = null
  })
}
