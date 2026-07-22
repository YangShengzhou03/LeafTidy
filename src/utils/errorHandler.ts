/**
 * 统一错误处理系统
 * 定义全局错误码、错误类型和错误处理工具
 */

/**
 * 错误码枚举
 * 按模块分类：1xxx-用户错误，2xxx-微信错误，3xxx-任务错误，4xxx-网络错误，5xxx-系统错误
 */
export enum ErrorCode {
  // 用户输入错误 (1000-1999)
  INVALID_PARAMETER = 1001,
  EMPTY_RECIPIENT = 1002,
  EMPTY_CONTENT = 1003,
  INVALID_TIME = 1004,
  FILE_NOT_FOUND = 1005,
  FILE_TOO_LARGE = 1006,
  FILE_PATH_FORBIDDEN = 1007,
  
  // 微信相关错误 (2000-2999)
  WECHAT_NOT_RUNNING = 2001,
  WECHAT_LOGIN_REQUIRED = 2002,
  WECHAT_WINDOW_NOT_FOUND = 2003,
  WECHAT_CONTACT_NOT_FOUND = 2004,
  WECHAT_SEND_FAILED = 2005,
  WECHAT_CLIPBOARD_FAILED = 2006,
  
  // 任务相关错误 (3000-3999)
  TASK_NOT_FOUND = 3001,
  TASK_ALREADY_EXISTS = 3002,
  TASK_EXECUTION_FAILED = 3003,
  TASK_SCHEDULE_FAILED = 3004,
  TASK_CANCEL_FAILED = 3005,
  TASK_INVALID_STATE = 3006,
  
  // 网络错误 (4000-4999)
  NETWORK_ERROR = 4001,
  NETWORK_TIMEOUT = 4002,
  API_ERROR = 4003,
  
  // 系统错误 (5000-5999)
  INTERNAL_ERROR = 5001,
  STORAGE_ERROR = 5002,
  PERMISSION_DENIED = 5003,
  RESOURCE_LOCKED = 5004,
  UNKNOWN_ERROR = 5999
}

/**
 * 应用错误类型
 */
export interface AppError {
  code: ErrorCode
  message: string
  details?: any
  timestamp: number
  userFriendly?: boolean
}

/**
 * 错误处理类
 */
export class ErrorHandler {
  /**
   * 创建标准错误对象
   */
  static create(
    code: ErrorCode, 
    message: string, 
    details?: any,
    userFriendly: boolean = true
  ): AppError {
    return {
      code,
      message,
      details,
      timestamp: Date.now(),
      userFriendly
    }
  }

  /**
   * 判断是否为用户可理解的错误
   */
  static isUserError(error: AppError): boolean {
    return error.code >= 1000 && error.code < 5000
  }

  /**
   * 判断是否为系统错误
   */
  static isSystemError(error: AppError): boolean {
    return error.code >= 5000
  }

  /**
   * 获取用户友好的错误消息
   */
  static getUserMessage(error: AppError | Error | unknown): string {
    if (this.isAppError(error)) {
      const appError = error as AppError
      return appError.userFriendly ? appError.message : '系统错误，请稍后重试'
    }
    
    if (error instanceof Error) {
      return error.message || '未知错误'
    }
    
    return '未知错误'
  }

  /**
   * 判断是否为AppError类型
   */
  static isAppError(error: unknown): boolean {
    return typeof error === 'object' && error !== null && 'code' in error && 'timestamp' in error
  }

  /**
   * 从原生错误转换
   */
  static fromError(error: Error, code: ErrorCode = ErrorCode.UNKNOWN_ERROR): AppError {
    return this.create(code, error.message, { stack: error.stack })
  }

  /**
   * 从Tauri错误转换
   */
  static fromTauriError(error: any): AppError {
    const message = typeof error === 'string' ? error : error.message || 'Tauri调用失败'
    return this.create(ErrorCode.INTERNAL_ERROR, message, error)
  }
}

/**
 * 安全执行包装器
 * 提供统一的错误处理和日志记录
 */
export async function safeExecute<T>(
  fn: () => Promise<T>,
  errorHandler?: (error: any) => AppError
): Promise<{ success: boolean; data?: T; error?: AppError }> {
  try {
    const data = await fn()
    return { success: true, data }
  } catch (error: any) {
    let appError: AppError
    
    if (errorHandler) {
      appError = errorHandler(error)
    } else if (ErrorHandler.isAppError(error)) {
      appError = error as AppError
    } else if (error instanceof Error) {
      appError = ErrorHandler.fromError(error)
    } else {
      appError = ErrorHandler.create(
        ErrorCode.UNKNOWN_ERROR,
        '未知错误',
        error
      )
    }

    console.error('[ErrorHandler]', appError)
    
    return { success: false, error: appError }
  }
}

/**
 * 错误码映射到用户消息
 */
const ERROR_MESSAGES: Partial<Record<ErrorCode, string>> = {
  [ErrorCode.INVALID_PARAMETER]: '参数无效',
  [ErrorCode.EMPTY_RECIPIENT]: '收件人不能为空',
  [ErrorCode.EMPTY_CONTENT]: '消息内容不能为空',
  [ErrorCode.INVALID_TIME]: '时间格式无效',
  [ErrorCode.FILE_NOT_FOUND]: '文件不存在',
  [ErrorCode.FILE_TOO_LARGE]: '文件大小超过限制',
  [ErrorCode.FILE_PATH_FORBIDDEN]: '文件路径不允许访问',
  
  [ErrorCode.WECHAT_NOT_RUNNING]: '微信未运行，请先启动微信',
  [ErrorCode.WECHAT_LOGIN_REQUIRED]: '请先登录微信',
  [ErrorCode.WECHAT_WINDOW_NOT_FOUND]: '找不到微信窗口',
  [ErrorCode.WECHAT_CONTACT_NOT_FOUND]: '找不到联系人',
  [ErrorCode.WECHAT_SEND_FAILED]: '发送失败',
  
  [ErrorCode.TASK_NOT_FOUND]: '任务不存在',
  [ErrorCode.TASK_EXECUTION_FAILED]: '任务执行失败',
  [ErrorCode.TASK_SCHEDULE_FAILED]: '任务调度失败',
  
  [ErrorCode.NETWORK_ERROR]: '网络连接失败',
  [ErrorCode.NETWORK_TIMEOUT]: '网络请求超时',
  
  [ErrorCode.INTERNAL_ERROR]: '系统错误',
  [ErrorCode.STORAGE_ERROR]: '数据存储失败',
  [ErrorCode.PERMISSION_DENIED]: '权限不足',
}

/**
 * 获取错误码对应的默认消息
 */
export function getDefaultMessage(code: ErrorCode): string {
  return ERROR_MESSAGES[code] || '操作失败'
}