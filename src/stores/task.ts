import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { TaskState, TaskEvent, TaskStateMachine, getStateText } from '../utils/taskStateMachine'
import { toBackendFormat } from '../utils/time'
import { ErrorCode, ErrorHandler, safeExecute } from '../utils/errorHandler'

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
  retryCount: number
  status: TaskState
  stateMachine?: TaskStateMachine // 新增: 状态机实例
}

export const useTaskStore = defineStore('task', () => {
  const tasks = ref<Task[]>([])
  const isExecuting = ref(false)
  
  // 问题7修复：防抖优化localStorage
  let saveTimer: ReturnType<typeof setTimeout> | null = null

  // 问题N5修复：加载时重新初始化状态机
  const loadFromStorage = () => {
    try {
      const stored = localStorage.getItem('leafmaster_tasks')
      if (stored) {
        const loadedTasks = JSON.parse(stored)
        // 重新初始化状态机实例
        tasks.value = loadedTasks.map((task: any) => ({
          ...task,
          stateMachine: new TaskStateMachine(task.status)
        }))
      }
    } catch (error) {
      console.error('加载任务数据失败:', error)
      tasks.value = []
    }
  }

  // 问题7修复：防抖保存，减少频繁操作
  // 问题N5修复：序列化时排除状态机实例
  // M1 修复：处理存储大小限制和溢出
  const saveToStorage = (immediate: boolean = false) => {
    if (saveTimer) {
      clearTimeout(saveTimer)
      saveTimer = null
    }

    const serializeTasks = () => {
      return tasks.value.map(task => ({
        ...task,
        stateMachine: undefined
      }))
    }

    const performSave = () => {
      try {
        const data = JSON.stringify(serializeTasks())

        // M1 修复：检查数据大小（localStorage 限制 ~5MB）
        const sizeInMB = new Blob([data]).size / (1024 * 1024)
        if (sizeInMB > 4) {
          // 超过 4MB，警告用户
          console.warn(`任务数据过大: ${sizeInMB.toFixed(2)}MB，建议清理历史任务`)
          logToFile('WARN', 'TaskStore', `任务数据大小: ${sizeInMB.toFixed(2)}MB`)
        }

        localStorage.setItem('leafmaster_tasks', data)
      } catch (error) {
        console.error('保存任务数据失败:', error)

        // M1 修复：检测是否为 QuotaExceededError
        if (error instanceof DOMException && error.name === 'QuotaExceededError') {
          logToFile('ERROR', 'TaskStore', '存储空间不足，无法保存任务数据')
          handleStorageOverflow()
        }
      }
    }

    if (immediate) {
      performSave()
    } else {
      saveTimer = setTimeout(performSave, 500)
    }
  }

  // M1 修复：存储溢出处理
  const handleStorageOverflow = async () => {
    try {
      // 提示用户清理或导出
      console.warn('存储空间不足，建议清理历史任务或导出备份')

      // 可以选择导出数据到文件（需要用户确认）
      // 这里简化处理，仅记录日志
      logToFile('ERROR', 'TaskStore', '存储空间不足，部分数据可能丢失')
    } catch (error) {
      console.error('处理存储溢出失败:', error)
    }
  }

  // 问题3修复：应用启动时初始化执行状态
  const initializeExecutionState = async () => {
    try {
      const pendingCount = await invoke<number>('get_pending_tasks_count')
      isExecuting.value = pendingCount > 0
      
      if (isExecuting.value) {
        await logToFile('INFO', 'TaskStore', '检测到后端有正在执行的任务')
      }
    } catch (error) {
      await logToFile('ERROR', 'TaskStore', `初始化状态失败: ${error}`)
      isExecuting.value = false
    }
  }

  const addTask = async (task: Omit<Task, 'id' | 'executeCount' | 'status' | 'retryCount' | 'stateMachine'>) => {
    // 问题14修复：使用统一错误处理
    // 参数验证
    if (!task.recipient || task.recipient.trim() === '') {
      throw ErrorHandler.create(ErrorCode.EMPTY_RECIPIENT, '收件人不能为空')
    }
    if (!task.content || task.content.trim() === '') {
      throw ErrorHandler.create(ErrorCode.EMPTY_CONTENT, '消息内容不能为空')
    }
    if (task.maxExecuteCount <= 0) {
      throw ErrorHandler.create(ErrorCode.INVALID_PARAMETER, '最大执行次数必须大于0')
    }

    // 创建状态机实例
    const stateMachine = new TaskStateMachine(TaskState.IDLE)
    
    const newTask: Task = {
      ...task,
      id: Date.now(),
      executeCount: 0,
      retryCount: 0,
      status: TaskState.IDLE,
      stateMachine
    }
    
    tasks.value.push(newTask)
    saveToStorage()

    // 如果任务启用，启动状态转换
    if (task.enabled) {
      stateMachine.transition(TaskEvent.ENABLE)
      newTask.status = stateMachine.getState()
    }

    // 问题11修复：使用统一的时间格式
    // 支持接收 Date 对象、ISO 字符串或本地时间字符串
    let executeTime: string
    if (task.executeTime instanceof Date) {
      executeTime = toBackendFormat(task.executeTime)
    } else if (typeof task.executeTime === 'string' && task.executeTime) {
      // 检查是否已经是 ISO 8601 格式（带时区）
      if (task.executeTime.includes('T') && (task.executeTime.includes('+') || task.executeTime.includes('Z'))) {
        executeTime = task.executeTime
      } else {
        // 本地时间字符串，解析为 Date 后再转换
        executeTime = toBackendFormat(new Date(task.executeTime))
      }
    } else {
      executeTime = toBackendFormat(new Date())
    }

    // 问题1修复：完全依赖后端调度，使用safeExecute
    // 注意：后端使用蛇形命名，需要转换参数名
    await logToFile('DEBUG', 'TaskStore', `准备调用 schedule_task: id=${newTask.id}, recipient=${newTask.recipient}, execute_time=${executeTime}`)

    const result = await safeExecute(
      () => invoke('schedule_task', {
        id: newTask.id,
        recipient: newTask.recipient,
        content: newTask.content,
        contentType: newTask.contentType,
        executeTime: executeTime,
        interval: newTask.interval,
        maxExecuteCount: newTask.maxExecuteCount
      }),
      (error) => {
        // 保留后端返回的原始错误信息
        const originalMessage = typeof error === 'string' ? error : (error?.message || '任务调度失败')
        return ErrorHandler.create(ErrorCode.TASK_SCHEDULE_FAILED, originalMessage, error)
      }
    )

    if (result.success) {
      await logToFile('INFO', 'TaskStore', `任务 #${newTask.id} 已添加到后端调度器`)
    } else {
      const errorMsg = result.error?.message || '任务调度失败'
      await logToFile('ERROR', 'TaskStore', `添加任务到后端失败: ${errorMsg}`)
      // 后端调度失败，移除本地任务
      tasks.value = tasks.value.filter(t => t.id !== newTask.id)
      saveToStorage(true) // 立即保存
      throw result.error
    }

    return newTask
  }

  const updateTask = async (id: number, updates: Partial<Task>) => {
    const task = tasks.value.find(t => t.id === id)
    if (!task) return

    // 更新本地状态
    Object.assign(task, updates)
    saveToStorage()

    // 如果启用了任务，重新调度
    if (updates.enabled === true) {
      try {
        await invoke('cancel_task', { taskId: id })
        await invoke('schedule_task', {
          id: task.id,
          recipient: task.recipient,
          content: task.content,
          contentType: task.contentType,
          executeTime: task.nextExecute,
          interval: task.interval,
          maxExecuteCount: task.maxExecuteCount
        })
        await logToFile('INFO', 'TaskStore', `任务 #${id} 已重新调度`)
      } catch (error) {
        await logToFile('ERROR', 'TaskStore', `重新调度任务失败: ${error}`)
      }
    } else if (updates.enabled === false) {
      // 禁用任务，从后端移除
      try {
        await invoke('cancel_task', { taskId: id })
        await logToFile('INFO', 'TaskStore', `任务 #${id} 已取消调度`)
      } catch (error) {
        await logToFile('ERROR', 'TaskStore', `取消任务失败: ${error}`)
      }
    }
  }

  const deleteTask = async (id: number) => {
    const index = tasks.value.findIndex(t => t.id === id)
    if (index !== -1) {
      tasks.value.splice(index, 1)
      saveToStorage()

      // 从后端移除
      try {
        await invoke('cancel_task', { taskId: id })
        await logToFile('INFO', 'TaskStore', `任务 #${id} 已删除`)
      } catch (error) {
        await logToFile('ERROR', 'TaskStore', `删除任务失败: ${error}`)
      }
    }
  }

  const toggleTask = async (id: number) => {
    const task = tasks.value.find(t => t.id === id)
    if (!task) return
    
    // 初始化状态机（如果不存在）
    if (!task.stateMachine) {
      task.stateMachine = new TaskStateMachine(task.status)
    }

    const newEnabled = !task.enabled
    task.enabled = newEnabled
    
    // 使用状态机进行状态转换
    const event = newEnabled ? TaskEvent.ENABLE : TaskEvent.DISABLE
    if (task.stateMachine.transition(event)) {
      task.status = task.stateMachine.getState()
      saveToStorage()

      // 更新后端调度
      if (newEnabled) {
        try {
          const executeTime = task.executeTime ? toBackendFormat(new Date(task.executeTime)) : task.nextExecute
          await invoke('schedule_task', {
            id: task.id,
            recipient: task.recipient,
            content: task.content,
            contentType: task.contentType,
            executeTime: executeTime,
            interval: task.interval,
            maxExecuteCount: task.maxExecuteCount
          })
          await logToFile('INFO', 'TaskStore', `任务 #${id} 已启用并调度，状态: ${getStateText(task.status)}`)
        } catch (error) {
          await logToFile('ERROR', 'TaskStore', `重新调度任务失败: ${error}`)
        }
      } else {
        try {
          await invoke('cancel_task', { taskId: id })
          await logToFile('INFO', 'TaskStore', `任务 #${id} 已禁用，状态: ${getStateText(task.status)}`)
        } catch (error) {
          await logToFile('ERROR', 'TaskStore', `取消任务失败: ${error}`)
        }
      }
    } else {
      await logToFile('WARN', 'TaskStore', `任务 #${id} 状态转换失败`)
    }
  }

  const clearAllTasks = async () => {
    // 取消所有后端任务
    for (const task of tasks.value) {
      try {
        await invoke('cancel_task', { taskId: task.id })
      } catch (error) {
        await logToFile('ERROR', 'TaskStore', `取消任务 #${task.id} 失败: ${error}`)
      }
    }

    tasks.value = []
    saveToStorage(true) // 立即保存
    isExecuting.value = false
  }

  // 问题1修复：移除前端执行逻辑，完全依赖后端调度
  // 前端仅标记状态，实际执行由后端 TaskScheduler 处理
  const startExecution = async () => {
    await logToFile('INFO', 'TaskExecutor', `开始执行任务，总数: ${tasks.value.length}`)

    if (tasks.value.length === 0) {
      await logToFile('WARN', 'TaskExecutor', '没有可执行的任务')
      throw new Error('没有可执行的任务，请先添加任务')
    }

    const enabledTasks = tasks.value.filter(t => t.enabled && t.maxExecuteCount > 0)
    await logToFile('INFO', 'TaskExecutor', `启用的任务数: ${enabledTasks.length}`)

    if (enabledTasks.length === 0) {
      await logToFile('WARN', 'TaskExecutor', '没有启用的任务')
      throw new Error('没有启用的任务，请先启用任务')
    }

    // 标记为执行中状态（仅用于UI显示）
    // 实际执行由后端 TaskScheduler 处理
    isExecuting.value = true
    saveToStorage()

    await logToFile('INFO', 'TaskExecutor', '任务已提交到后端调度器，等待执行...')
  }

  const stopExecution = () => {
    isExecuting.value = false

    // 更新所有running状态的任务为paused
    tasks.value.forEach(task => {
      if (task.status === TaskState.RUNNING) {
        task.status = TaskState.PAUSED
      }
    })

    saveToStorage()
    logToFile('INFO', 'TaskExecutor', '前端执行状态已停止')
  }

  const statistics = computed(() => ({
    total: tasks.value.length,
    running: tasks.value.filter(t => t.status === TaskState.RUNNING).length,
    pending: tasks.value.filter(t => t.status === TaskState.PENDING).length,
    completed: tasks.value.filter(t => t.status === TaskState.COMPLETED).length,
    failed: tasks.value.filter(t => t.status === TaskState.FAILED).length
  }))

  // 初始化
  loadFromStorage()
  
  // 问题3修复：应用启动时同步后端状态
  initializeExecutionState()

  // 问题7修复：页面卸载前强制保存
  window.addEventListener('beforeunload', () => {
    if (saveTimer) {
      clearTimeout(saveTimer)
      localStorage.setItem('leafmaster_tasks', JSON.stringify(tasks.value))
    }
  })

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
    saveToStorage,
    initializeExecutionState
  }
})