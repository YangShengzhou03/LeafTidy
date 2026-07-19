import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface WeChatStatus {
  online: boolean
  username: string | null
  login_time: string | null
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
    login_time: null
  })

  const loading = ref(false)

  async function checkStatus() {
    loading.value = true
    try {
      const result = await invoke<WeChatStatus>('get_wechat_status')
      status.value = result
    } catch (error) {
      console.error('检查微信状态失败:', error)
      status.value = {
        online: false,
        username: null,
        login_time: null
      }
    } finally {
      loading.value = false
    }
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
    sendMessage,
    sendFile
  }
})