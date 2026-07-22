/**
 * 任务状态机
 * 定义任务的状态转换规则，确保状态管理的一致性和可预测性
 */

/**
 * 任务状态枚举
 */
export enum TaskState {
  IDLE = 'idle',           // 空闲状态（任务创建但未启用）
  PENDING = 'pending',     // 待执行（任务已调度，等待执行时间）
  RUNNING = 'running',     // 运行中（任务正在执行）
  PAUSED = 'paused',       // 已暂停（任务被手动暂停）
  COMPLETED = 'completed', // 已完成（任务达到最大执行次数）
  FAILED = 'failed'        // 失败（任务执行失败且重试次数用尽）
}

/**
 * 任务事件枚举
 */
export enum TaskEvent {
  START = 'start',       // 启动任务
  ENABLE = 'enable',     // 启用任务
  DISABLE = 'disable',   // 禁用任务
  PAUSE = 'pause',       // 暂停任务
  RESUME = 'resume',     // 恢复任务
  EXECUTE = 'execute',   // 执行任务
  COMPLETE = 'complete', // 任务完成
  FAIL = 'fail',         // 任务失败
  RETRY = 'retry',       // 重试任务
  RESET = 'reset'        // 重置任务
}

/**
 * 状态转换定义
 * 定义每个状态下可以接受的事件以及转换后的目标状态
 */
const STATE_TRANSITIONS: Record<TaskState, Partial<Record<TaskEvent, TaskState>>> = {
  [TaskState.IDLE]: {
    [TaskEvent.ENABLE]: TaskState.PENDING,
    [TaskEvent.START]: TaskState.PENDING
  },

  [TaskState.PENDING]: {
    [TaskEvent.DISABLE]: TaskState.IDLE,
    [TaskEvent.EXECUTE]: TaskState.RUNNING,
    [TaskEvent.PAUSE]: TaskState.PAUSED
  },

  [TaskState.RUNNING]: {
    [TaskEvent.COMPLETE]: TaskState.COMPLETED,
    [TaskEvent.FAIL]: TaskState.FAILED,
    [TaskEvent.PAUSE]: TaskState.PAUSED
  },

  [TaskState.PAUSED]: {
    [TaskEvent.RESUME]: TaskState.PENDING,
    [TaskEvent.DISABLE]: TaskState.IDLE,
    [TaskEvent.RESET]: TaskState.IDLE
  },

  [TaskState.COMPLETED]: {
    [TaskEvent.RESET]: TaskState.IDLE
  },

  [TaskState.FAILED]: {
    [TaskEvent.RETRY]: TaskState.PENDING,
    [TaskEvent.RESET]: TaskState.IDLE
  }
}

/**
 * 任务状态机类
 * 管理单个任务的状态转换
 */
export class TaskStateMachine {
  private currentState: TaskState
  
  constructor(initialState: TaskState = TaskState.IDLE) {
    this.currentState = initialState
  }
  
  /**
   * 尝试执行状态转换
   * @param event - 触发事件
   * @returns 转换是否成功
   */
  transition(event: TaskEvent): boolean {
    const nextState = STATE_TRANSITIONS[this.currentState]?.[event]
    
    if (nextState) {
      const previousState = this.currentState
      this.currentState = nextState
      
      console.log(`[TaskStateMachine] 状态转换: ${previousState} -> ${nextState} (事件: ${event})`)
      return true
    }
    
    console.warn(`[TaskStateMachine] 无效的状态转换: ${this.currentState} -> ${event}`)
    return false
  }
  
  /**
   * 获取当前状态
   */
  getState(): TaskState {
    return this.currentState
  }
  
  /**
   * 检查是否可以执行指定事件
   * @param event - 要检查的事件
   */
  canTransition(event: TaskEvent): boolean {
    return !!STATE_TRANSITIONS[this.currentState]?.[event]
  }
  
  /**
   * 重置状态机到初始状态
   */
  reset(): void {
    this.currentState = TaskState.IDLE
  }
}

/**
 * 获取状态的显示文本
 */
export function getStateText(state: TaskState): string {
  const stateTextMap: Record<TaskState, string> = {
    [TaskState.IDLE]: '空闲',
    [TaskState.PENDING]: '待执行',
    [TaskState.RUNNING]: '运行中',
    [TaskState.PAUSED]: '已暂停',
    [TaskState.COMPLETED]: '已完成',
    [TaskState.FAILED]: '失败'
  }
  
  return stateTextMap[state] || state
}

/**
 * 获取状态对应的CSS类名
 */
export function getStateClass(state: TaskState): string {
  const stateClassMap: Record<TaskState, string> = {
    [TaskState.IDLE]: 'state-idle',
    [TaskState.PENDING]: 'state-pending',
    [TaskState.RUNNING]: 'state-running',
    [TaskState.PAUSED]: 'state-paused',
    [TaskState.COMPLETED]: 'state-completed',
    [TaskState.FAILED]: 'state-failed'
  }
  
  return stateClassMap[state] || ''
}