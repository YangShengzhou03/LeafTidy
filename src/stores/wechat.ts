import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

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
    try {
      const result = await invoke<WeChatStatus>('get_wechat_status')
      // 保留本地维护的 task_running 状态
      const currentTaskRunning = status.value.task_running
      status.value = {
        ...result,
        task_running: currentTaskRunning
      }
    } catch (error) {
      console.error('检查微信状态失败:', error)
      status.value = {
        online: false,
        username: null,
        wechat_id: null,
        login_time: null,
        task_running: false
      }
    } finally {
      loading.value = false
    }
  }

  function setTaskRunning(running: boolean) {
    status.value.task_running = running
  }

  async function sendMessage(recipient: string, message: string): Promise<SendMessageResult> {
    try {
      const result = await invoke<SendMessageResult>('send_message', { recipient, message })
      return result
    } catch (error) {
      console.error('发送消息失败:', error)
      return {
        success: false,
        message: String(error)
      }
    }
  }

  async function sendFile(recipient: string, filepath: string): Promise<SendFileResult> {
    try {
      const result = await invoke<SendFileResult>('send_file', { recipient, filepath })
      return result
    } catch (error) {
      console.error('发送文件失败:', error)
      return {
        success: false,
        message: String(error)
      }
    }
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