import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ErrorCode, ErrorHandler, safeExecute } from '../utils/errorHandler'

export interface WeChatStatus {
  online: boolean
  username: string | null
  wechat_id: string | null
  login_time: string | null
  task_running: boolean
}

export interface SendMessageResult {
  success: boolean
  message: string
}

export interface SendFileResult {
  success: boolean
  message: string
}

export const useWeChatStore = defineStore('wechat', () => {
  const status = ref<WeChatStatus>({
    online: false,
    username: null,
    wechat_id: null,
    login_time: null,
    task_running: false
  })

  const loading = ref(false)

  async function checkStatus() {
    loading.value = true
    const result = await safeExecute(
      () => invoke<WeChatStatus>('get_wechat_status'),
      (error) => ErrorHandler.create(ErrorCode.WECHAT_NOT_RUNNING, '检查微信状态失败', error)
    )

    if (result.success && result.data) {
      status.value = result.data
    } else {
      console.error('检查微信状态失败:', result.error)
      status.value = {
        online: false,
        username: null,
        wechat_id: null,
        login_time: null,
        task_running: false
      }
    }

    loading.value = false
  }

  function setTaskRunning(running: boolean) {
    status.value.task_running = running
  }

  async function sendMessage(recipient: string, message: string): Promise<SendMessageResult> {
    const result = await safeExecute(
      () => invoke<SendMessageResult>('send_message', { recipient, message }),
      (error) => ErrorHandler.create(ErrorCode.WECHAT_SEND_FAILED, '发送消息失败', error)
    )

    return result.success && result.data
      ? result.data
      : { success: false, message: result.error?.message || '未知错误' }
  }

  async function sendFile(recipient: string, filepath: string): Promise<SendFileResult> {
    const result = await safeExecute(
      () => invoke<SendFileResult>('send_file', { recipient, filepath }),
      (error) => ErrorHandler.create(ErrorCode.WECHAT_SEND_FAILED, '发送文件失败', error)
    )

    return result.success && result.data
      ? result.data
      : { success: false, message: result.error?.message || '未知错误' }
  }

  return {
    status,
    loading,
    checkStatus,
    setTaskRunning,
    sendMessage,
    sendFile
  }
})