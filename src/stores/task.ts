import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface Task {
  id: number
  recipient: string
  content: string
  contentType: 'text' | 'file'
  fileName?: string
  type: '定时' | '间隔'
  executeTime?: string
  repeatMode?: string
  interval?: number
  weekdays?: string[]
  executeMode: string
  maxExecuteCount: number
  nextExecute: string
  enabled: boolean
  executeCount: number
  autoSplit: boolean
  retryOnFail: boolean
  status: 'pending' | 'running' | 'paused' | 'completed' | 'failed'
}

export const useTaskStore = defineStore('task', () => {
  const tasks = ref<Task[]>([])
  const isExecuting = ref(false)

  // 从 localStorage 加载数据
  const loadFromStorage = () => {
    try {
      const stored = localStorage.getItem('leafmaster_tasks')
      if (stored) {
        tasks.value = JSON.parse(stored)
      }
    } catch (error) {
      console.error('加载任务数据失败:', error)
    }
  }

  // 保存到 localStorage
  const saveToStorage = () => {
    try {
      localStorage.setItem('leafmaster_tasks', JSON.stringify(tasks.value))
    } catch (error) {
      console.error('保存任务数据失败:', error)
    }
  }

  // 添加任务
  const addTask = (task: Omit<Task, 'id' | 'executeCount' | 'status'>) => {
    const newTask: Task = {
      ...task,
      id: Date.now(),
      executeCount: 0,
      status: task.enabled ? 'running' : 'pending'
    }
    tasks.value.push(newTask)
    saveToStorage()
    return newTask
  }

  // 更新任务
  const updateTask = (id: number, updates: Partial<Task>) => {
    const index = tasks.value.findIndex(t => t.id === id)
    if (index !== -1) {
      tasks.value[index] = { ...tasks.value[index], ...updates }
      saveToStorage()
    }
  }

  // 删除任务
  const deleteTask = (id: number) => {
    const index = tasks.value.findIndex(t => t.id === id)
    if (index !== -1) {
      tasks.value.splice(index, 1)
      saveToStorage()
    }
  }

  // 切换任务状态
  const toggleTask = (id: number) => {
    const task = tasks.value.find(t => t.id === id)
    if (task) {
      task.enabled = !task.enabled
      task.status = task.enabled ? 'running' : 'paused'
      saveToStorage()
    }
  }

  // 清空所有任务
  const clearAllTasks = () => {
    tasks.value = []
    saveToStorage()
  }

  // 开始执行任务
  const startExecution = async () => {
    if (tasks.value.length === 0) {
      throw new Error('没有可执行的任务')
    }

    const enabledTasks = tasks.value.filter(t => t.enabled)
    if (enabledTasks.length === 0) {
      throw new Error('没有启用的任务')
    }

    isExecuting.value = true

    // 更新任务状态：启用的任务变为"运行中"，未启用的保持"已停用"
    tasks.value.forEach(task => {
      if (task.enabled) {
        task.status = 'running'
      } else {
        task.status = 'paused'
      }
    })

    saveToStorage()

    // 启动定时器检查任务执行时间
    const checkInterval = setInterval(() => {
      if (!isExecuting.value) {
        clearInterval(checkInterval)
        return
      }

      const now = new Date()
      tasks.value.forEach(async (task) => {
        if (task.enabled && task.status === 'running') {
          const executeTime = new Date(task.nextExecute)
          if (executeTime <= now && task.executeCount < task.maxExecuteCount) {
            // 执行任务
            try {
              console.log(`执行任务 #${task.id}: 发送 '${task.content}' 给 ${task.recipient}`)

              // 调用后端API发送微信消息
              await invoke('send_message', {
                recipient: task.recipient,
                content: task.content
              })

              // 更新执行次数
              task.executeCount++

              // 更新下次执行时间
              if (task.type === '间隔' && task.interval) {
                task.nextExecute = new Date(Date.now() + task.interval * 60 * 1000).toISOString()
              } else if (task.executeCount >= task.maxExecuteCount) {
                task.status = 'completed'
              }

              saveToStorage()
            } catch (error) {
              console.error('任务执行失败:', error)
              task.status = 'failed'
              saveToStorage()

              // 播放失败提示音
              playErrorSound()
            }
          }
        }
      })
    }, 1000) // 每秒检查一次
  }

  // 播放失败提示音
  const playErrorSound = () => {
    try {
      const audio = new Audio('data:audio/wav;base64,UklGRnoGAABXQVZFZm10IBAAAAABAAEAQB8AAEAfAAABAAgAZGF0YQoGAACBhYqFbF1fdJivrJBhNjVgodDbq2EcBj+a2teleQKSy6upj4WMeH1zblBIRz05NzYzLisqKywtLi8xMzU2ODo8PDw8Ozo4NzUzMS4sKykoKCoqLC0uMDI0Njc5Ozw9PTw7Ojk3NTIwLi0ra2goKScnJygqKywtLi8xMzU2ODs8PT09PDs5NzUzMS8tLCkomainJCMhICAgICEiJCcoKi0uMDI0Njc5Ozw9PT09PDs5NzUzMS8tLCkomainJCMhICAfHx8gISQnKiotLzAyNDY3OTo7PD0+Pj09PDs5NzUzMS8tLCkmainJCMhICAfHh4eICEkJiqqrS8wMjQ2Nzk6Ozw9Pj5/Pz49PTw7OTc1MzEvLSwpJ6enpYyKiYiHhoWFhIiJioyNjo+QkJCQj42Ni4mIh4aFhIKBgYB+fX18e3p5eHd2dXRzcnFwb25tbGtqaWhnZmVkYmJgX15dXFtaWFdWVVRTUlFQT05NTEtKSUhHRkVEQ0JBQT8+PTw7Ojk3NTMxLy0sKScmJSQjISEfHh0cGxoZGBcWFRQTEhEPDg0LCgkIBwYFBAMCAQA=')
      audio.volume = 0.5
      audio.play()
    } catch (error) {
      console.error('播放提示音失败:', error)
    }
  }

  // 停止执行任务
  const stopExecution = () => {
    isExecuting.value = false

    // 将所有running状态的任务改为paused
    tasks.value.forEach(task => {
      if (task.status === 'running') {
        task.status = 'paused'
      }
    })

    saveToStorage()
  }

  // 统计数据
  const statistics = computed(() => ({
    total: tasks.value.length,
    running: tasks.value.filter(t => t.status === 'running').length,
    pending: tasks.value.filter(t => t.status === 'pending').length,
    completed: tasks.value.filter(t => t.status === 'completed').length,
    failed: tasks.value.filter(t => t.status === 'failed').length
  }))

  // 初始化时加载数据
  loadFromStorage()

  return {
    tasks,
    isExecuting,
    statistics,
    addTask,
    updateTask,
    deleteTask,
    toggleTask,
    clearAllTasks,
    startExecution,
    stopExecution,
    loadFromStorage,
    saveToStorage
  }
})