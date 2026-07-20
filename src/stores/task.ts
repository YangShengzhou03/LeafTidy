import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const logToFile = async (level: 'INFO' | 'WARN' | 'ERROR' | 'DEBUG', module: string, message: string) => {
  try {
    await invoke('log_frontend', { level, module, message })
  } catch (error) {
    console.error('写入日志失败:', error)
  }
}

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
  let executionTimer: ReturnType<typeof setInterval> | null = null

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

  const saveToStorage = () => {
    try {
      localStorage.setItem('leafmaster_tasks', JSON.stringify(tasks.value))
    } catch (error) {
      console.error('保存任务数据失败:', error)
    }
  }

  const addTask = (task: Omit<Task, 'id' | 'executeCount' | 'status'>) => {
    const newTask: Task = {
      ...task,
      id: Date.now(),
      executeCount: 0,
      status: 'pending'
    }
    tasks.value.push(newTask)
    saveToStorage()
    return newTask
  }

  const updateTask = (id: number, updates: Partial<Task>) => {
    const index = tasks.value.findIndex(t => t.id === id)
    if (index !== -1) {
      tasks.value[index] = { ...tasks.value[index], ...updates }
      saveToStorage()
    }
  }

  const deleteTask = (id: number) => {
    const index = tasks.value.findIndex(t => t.id === id)
    if (index !== -1) {
      tasks.value.splice(index, 1)
      saveToStorage()

      if (tasks.value.length === 0) {
        stopExecution()
      }
    }
  }

  const toggleTask = (id: number) => {
    const task = tasks.value.find(t => t.id === id)
    if (task) {
      task.enabled = !task.enabled
      task.status = task.enabled ? 'pending' : 'paused'
      saveToStorage()
    }
  }

  const clearAllTasks = () => {
    tasks.value = []
    saveToStorage()
    stopExecution()
  }

  const startExecution = async () => {
    await logToFile('INFO', 'TaskExecutor', `开始执行任务，总数: ${tasks.value.length}`)

    if (tasks.value.length === 0) {
      await logToFile('ERROR', 'TaskExecutor', '没有可执行的任务')
      throw new Error('没有可执行的任务')
    }

    const enabledTasks = tasks.value.filter(t => t.enabled)
    await logToFile('INFO', 'TaskExecutor', `启用的任务数: ${enabledTasks.length}`)

    if (enabledTasks.length === 0) {
      await logToFile('ERROR', 'TaskExecutor', '没有启用的任务')
      throw new Error('没有启用的任务')
    }

    isExecuting.value = true

    tasks.value.forEach(task => {
      if (task.enabled && task.status === 'pending') {
        task.status = 'running'
      }
    })

    saveToStorage()

    if (executionTimer) {
      clearInterval(executionTimer)
    }

    await logToFile('INFO', 'TaskExecutor', '启动定时器，每秒检查一次任务')

    executionTimer = setInterval(async () => {
      if (!isExecuting.value) {
        if (executionTimer) {
          clearInterval(executionTimer)
          executionTimer = null
        }
        return
      }

      const now = new Date()

      for (const task of tasks.value) {
        if (task.enabled && task.status === 'running') {
          const executeTime = new Date(task.nextExecute)

          if (isNaN(executeTime.getTime())) {
            await logToFile('ERROR', 'TaskExecutor', `任务 #${task.id} 的执行时间格式错误: ${task.nextExecute}`)
            task.status = 'failed'
            saveToStorage()
            continue
          }

          const timeDiff = now.getTime() - executeTime.getTime()

          if (timeDiff < 0) {
            continue
          }

          if (task.executeCount < task.maxExecuteCount) {
            try {
              await logToFile('INFO', 'TaskExecutor', `任务 #${task.id} 开始执行: 发送给 ${task.recipient}`)

              task.executeCount++
              saveToStorage()

              let result
              try {
                result = await invoke('send_message', {
                  recipient: task.recipient,
                  message: task.content
                })
              } catch (invokeError: any) {
                throw new Error(`后端调用失败: ${invokeError?.message || invokeError}`)
              }

              if (!result || !result.success) {
                throw new Error(result?.message || '发送失败')
              }

              if (task.type === '间隔' && task.interval) {
                task.nextExecute = new Date(Date.now() + task.interval * 60 * 1000).toISOString()
              } else {
                task.status = 'completed'
              }

              saveToStorage()
              await logToFile('INFO', 'TaskExecutor', `任务 #${task.id} 执行成功`)
            } catch (error: any) {
              const errorMsg = error?.message || error?.toString() || '未知错误'
              await logToFile('ERROR', 'TaskExecutor', `任务 #${task.id} 执行失败: ${errorMsg}`)
              task.status = 'failed'
              task.executeCount--
              saveToStorage()
              playErrorSound()
            }
          }
        }
      }

      const hasRunningTask = tasks.value.some(t => t.status === 'running' && t.enabled)
      if (!hasRunningTask) {
        await logToFile('INFO', 'TaskExecutor', '所有任务已完成，自动停止执行')
        stopExecution()
      }
    }, 1000)
  }

  const playErrorSound = () => {
    try {
      const audio = new Audio('data:audio/wav;base64,UklGRnoGAABXQVZFZm10IBAAAAABAAEAQB8AAEAfAAABAAgAZGF0YQoGAACBhYqFbF1fdJivrJBhNjVgodDbq2EcBj+a2teleQKSy6upj4WMeH1zblBIRz05NzYzLisqKywtLi8xMzU2ODo8PDw8Ozo4NzUzMS4sKykoKCoqLC0uMDI0Njc5Ozw9PTw7Ojk3NTIwLi0ra2goKScnJygqKywtLi8xMzU2ODs8PT09PDs5NzUzMS8tLCkmainJCMhICAgICEiJCcoKi0uMDI0Njc5Ozw9PT09PDs5NzUzMS8tLCkmainJCMhICAfHx8gISQnKiotLzAyNDY3OTo7PD0+Pj09PDs5NzUzMS8tLCkmainJCMhICAfHh4eICEkJiqqrS8wMjQ2Nzk6Ozw9Pj5/Pz49PTw7OTc1MzEvLSwpJ6enpYyKiYiHhoWFhIiJioyNjo+QkJCQj42Ni4mIh4aFhIKBgYB+fX18e3p5eHd2dXRzcnFwb25tbGtqaWhnZmVkYmJgX15dXFtaWFdWVVRTUlFQT05NTEtKSUhHRkVEQ0JBQT8+PTw7Ojk3NTMxLy0sKScmJSQjISEfHh0cGxoZGBcWFRQTEhEPDg0LCgkIBwYFBAMCAQA=')
      audio.volume = 0.5
      audio.play()
    } catch (error) {
      console.error('播放提示音失败:', error)
    }
  }

  const stopExecution = () => {
    isExecuting.value = false

    if (executionTimer) {
      clearInterval(executionTimer)
      executionTimer = null
    }

    tasks.value.forEach(task => {
      if (task.status === 'running') {
        task.status = 'paused'
      }
    })

    saveToStorage()
  }

  const statistics = computed(() => ({
    total: tasks.value.length,
    running: tasks.value.filter(t => t.status === 'running').length,
    pending: tasks.value.filter(t => t.status === 'pending').length,
    completed: tasks.value.filter(t => t.status === 'completed').length,
    failed: tasks.value.filter(t => t.status === 'failed').length
  }))

  loadFromStorage()

  isExecuting.value = false
  if (executionTimer) {
    clearInterval(executionTimer)
    executionTimer = null
  }

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