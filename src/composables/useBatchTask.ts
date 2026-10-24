import { inject, ref, type Ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElNotification } from 'element-plus'
import { isCancelledError } from '@/utils/cancel'
import type { CancelResult } from '@/types'
import { useProgressListener } from './useProgressListener'

export interface RunHandlers<T> {
  /** 成功后的处理（更新结果、提示等） */
  onSuccess?: (data: T) => void | Promise<void>
  /** 用户主动取消后的处理（写取消日志、提示等） */
  onCancelled?: () => void | Promise<void>
  /** 失败后的处理（提示错误信息） */
  onError?: (error: string) => void
}

export interface CancelMessages {
  title: string
  message: string
  errorTitle: string
  failed: (error: unknown) => string
}

/**
 * 统一批量任务的执行骨架：
 * 互斥（isProcessing + 双击守卫）、进度监听、取消、成功/取消/失败分流。
 * 页面只负责自己的业务参数、结果处理与提示文案。
 */
export function useBatchTask<P>(config: {
  /** 后端的取消作用域，可传多个（如混合任务跨两类任务） */
  scope: string | string[]
  /** 进度事件名，可传多个（如混合任务分两段） */
  event: string | string[]
  initialProgress: () => P
  onProgress?: (progress: P) => void
}) {
  const isProcessing = inject<Ref<boolean>>('isProcessing')!
  const running = ref(false)
  const progress = ref(config.initialProgress()) as Ref<P>
  let guard = false

  const scopes = Array.isArray(config.scope) ? config.scope : [config.scope]
  const events = Array.isArray(config.event) ? config.event : [config.event]

  for (const event of events) {
    useProgressListener<P>(event, (payload) => {
      progress.value = payload
      config.onProgress?.(payload)
    })
  }

  async function run<T>(task: () => Promise<T>, handlers: RunHandlers<T> = {}): Promise<T | undefined> {
    // 同步守卫：Vue 的响应式更新是微任务，拦不住同事件循环内的第二次点击
    if (guard || running.value || isProcessing.value) return undefined
    guard = true
    running.value = true
    isProcessing.value = true
    progress.value = config.initialProgress()

    try {
      const data = await task()
      await handlers.onSuccess?.(data)
      return data
    } catch (e) {
      if (isCancelledError(e)) {
        await handlers.onCancelled?.()
      } else {
        handlers.onError?.(String(e))
      }
      return undefined
    } finally {
      running.value = false
      isProcessing.value = false
      guard = false
    }
  }

  /// 只发起取消请求，不提前释放 isProcessing（任务真正结束后由 run 的 finally 释放）
  async function cancel(messages: CancelMessages) {
    try {
      await Promise.all(scopes.map(scope => invoke<CancelResult>('cancel_operation', { scope })))
      ElNotification({ type: 'info', title: messages.title, message: messages.message })
    } catch (e) {
      ElNotification({ type: 'error', title: messages.errorTitle, message: messages.failed(e) })
    }
  }

  return { running, progress, run, cancel }
}
