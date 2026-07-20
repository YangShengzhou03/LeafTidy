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
  let executionTimer: ReturnType<typeof setInterval> | null = null

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
      status: 'pending' // 新建任务统一为待执行状态
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
      
      // 如果没有任务了，停止执行
      if (tasks.value.length === 0) {
        stopExecution()
      }
    }
  }

  // 切换任务状态
  const toggleTask = (id: number) => {
    const task = tasks.value.find(t => t.id === id)
    if (task) {
      task.enabled = !task.enabled
      task.status = task.enabled ? 'pending' : 'paused'
      saveToStorage()
    }
  }

  // 清空所有任务
  const clearAllTasks = () => {
    tasks.value = []
    saveToStorage()
    stopExecution() // 清空任务时停止执行
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

    // 更新任务状态：启用的任务变为"运行中"
    tasks.value.forEach(task => {
      if (task.enabled && task.status === 'pending') {
        task.status = 'running'
      }
    })

    saveToStorage()

    // 清理旧的定时器
    if (executionTimer) {
      clearInterval(executionTimer)
    }

    // 启动定时器检查任务执行时间
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
          
          // 验证时间格式是否有效
          if (isNaN(executeTime.getTime())) {
            console.error(`[任务错误] 任务#${task.id} 的执行时间格式错误:`, task.nextExecute)
            continue
          }
          
          // 只执行执行时间已到且未超过5分钟的任务（避免执行过期太久的任务）
          const timeDiff = now.getTime() - executeTime.getTime()
          console.log(`[任务检查] 任务#${task.id} 执行时间: ${executeTime.toLocaleString('zh-CN')}, 当前时间: ${now.toLocaleString('zh-CN')}, 时间差: ${Math.round(timeDiff / 1000)}秒`)
          
          if (timeDiff >= 0 && timeDiff < 5 * 60 * 1000 && task.executeCount < task.maxExecuteCount) {
            try {
              console.log(`[任务执行] 开始执行任务 #${task.id}: 发送 '${task.content}' 给 ${task.recipient}`)

              // 调用后端API发送微信消息
              const result = await invoke('send_message', {
                recipient: task.recipient,
                content: task.content
              })
              
              console.log(`[任务执行] 后端返回:`, result)

              // 更新执行次数
              task.executeCount++

              // 更新下次执行时间
              if (task.type === '间隔' && task.interval) {
                task.nextExecute = new Date(Date.now() + task.interval * 60 * 1000).toISOString()
              } else {
                // 定时任务执行一次后标记为完成
                task.status = 'completed'
              }

              saveToStorage()
              console.log(`[任务执行] 任务 #${task.id} 执行成功`)
            } catch (error) {
              console.error('[任务执行] 失败:', error)
              task.status = 'failed'
              saveToStorage()
              playErrorSound()
            }
          }
        }
      }
    }, 1000) // 每秒检查一次
  }

  // 播放失败提示音
  const playErrorSound = () => {
    try {
      const audio = new Audio('data:audio/wav;base64,UklGRnoGAABXQVZFZm10IBAAAAABAAEAQB8AAEAfAAABAAgAZGF0YQoGAACBhYqFbF1fdJivrJBhNjVgodDbq2EcBj+a2teleQKSy6upj4WMeH1zblBIRz05NzYzLisqKywtLi8xMzU2ODo8PDw8Ozo4NzUzMS4sKykoKCoqLC0uMDI0Njc5Ozw9PTw7Ojk3NTIwLi0ra2goKScnJygqKywtLi8xMzU2ODs8PT09PDs5NzUzMS8tLCkmainJCMhICAgICEiJCcoKi0uMDI0Njc5Ozw9PT09PDs5NzUzMS8tLCkmainJCMhICAfHx8gISQnKiotLzAyNDY3OTo7PD0+Pj09PDs5NzUzMS8tLCkmainJCMhICAfHh4eICEkJiqqrS8wMjQ2Nzk6Ozw9Pj5/Pz49PTw7OTc1MzEvLSwpJ6enpYyKiYiHhoWFhIiJioyNjo+QkJCQj42Ni4mIh4aFhIKBgYB+fX18e3p5eHd2dXRzcnFwb25tbGtqaWhnZmVkYmJgX15dXFtaWFdWVVRTUlFQT05NTEtKSUhHRkVEQ0JBQT8+PTw7Ojk3NTMxLy0sKScmJSQjISEfHh0cGxoZGBcWFRQTEhEPDg0LCgkIBwYFBAMCAQA=')
      audio.volume = 0.5
      audio.play()
    } catch (error) {
      console.error('播放提示音失败:', error)
    }
  }

  // 停止执行任务
  const stopExecution = () => {
    isExecuting.value = false

    // 清理定时器
    if (executionTimer) {
      clearInterval(executionTimer)
      executionTimer = null
    }

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
  
  // 确保初始化时状态正确
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