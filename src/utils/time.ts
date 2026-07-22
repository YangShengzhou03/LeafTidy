/**
 * 时间处理工具函数
 * 统一前后端时间格式，确保一致性
 */

/**
 * 格式化时间为本地时间格式 "YYYY-MM-DD HH:MM"
 * @param time - 时间字符串或Date对象
 * @returns 格式化后的时间字符串
 */
export function formatTime(time: string | Date | undefined): string {
  if (!time) return '-'

  try {
    const date = typeof time === 'string' ? new Date(time) : time
    
    if (isNaN(date.getTime())) return '-'

    const year = date.getFullYear()
    const month = String(date.getMonth() + 1).padStart(2, '0')
    const day = String(date.getDate()).padStart(2, '0')
    const hours = String(date.getHours()).padStart(2, '0')
    const minutes = String(date.getMinutes()).padStart(2, '0')

    return `${year}-${month}-${day} ${hours}:${minutes}`
  } catch {
    return '-'
  }
}

/**
 * 将Date对象转换为后端接受的格式
 * M2 修复：统一使用 ISO 8601 格式（带时区信息）
 * @param date - Date对象
 * @returns ISO 8601 格式的时间字符串（带时区）
 */
export function toBackendFormat(date: Date): string {
  // M2 修复：使用 ISO 8601 格式（带时区信息）
  // 格式: YYYY-MM-DDTHH:MM:SS+HH:MM 或 YYYY-MM-DDTHH:MM:SSZ

  // 获取时区偏移（分钟）
  const offset = -date.getTimezoneOffset()
  const sign = offset >= 0 ? '+' : '-'
  const hours = String(Math.floor(Math.abs(offset) / 60)).padStart(2, '0')
  const minutes = String(Math.abs(offset) % 60).padStart(2, '0')

  // 格式化时间部分
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  const hour = String(date.getHours()).padStart(2, '0')
  const minute = String(date.getMinutes()).padStart(2, '0')
  const second = String(date.getSeconds()).padStart(2, '0')

  // 组合成 ISO 8601 格式
  return `${year}-${month}-${day}T${hour}:${minute}:${second}${sign}${hours}:${minutes}`
}

/**
 * 解析后端时间字符串为Date对象
 * @param timeStr - 时间字符串（可以是 "YYYY-MM-DD HH:MM" 或 ISO 8601格式）
 * @returns Date对象或null
 */
export function parseTime(timeStr: string): Date | null {
  if (!timeStr) return null

  try {
    // 尝试解析为ISO 8601格式
    if (timeStr.includes('T')) {
      const date = new Date(timeStr)
      return isNaN(date.getTime()) ? null : date
    }

    // 解析本地时间格式 "YYYY-MM-DD HH:MM"
    const date = new Date(timeStr)
    return isNaN(date.getTime()) ? null : date
  } catch {
    return null
  }
}

/**
 * 验证时间是否在未来
 * @param time - 时间字符串或Date对象
 * @returns 是否在未来
 */
export function isFutureTime(time: string | Date): boolean {
  const date = typeof time === 'string' ? parseTime(time) : time
  if (!date) return false

  return date.getTime() > Date.now()
}

/**
 * 获取当前时间的后端格式
 * @returns 当前时间的格式化字符串
 */
export function getCurrentTimeBackend(): string {
  return toBackendFormat(new Date())
}

/**
 * 计算两个时间之间的分钟差
 * @param startTime - 开始时间
 * @param endTime - 结束时间
 * @returns 分钟差
 */
export function getMinutesDiff(startTime: Date | string, endTime: Date | string): number {
  const start = typeof startTime === 'string' ? parseTime(startTime) : startTime
  const end = typeof endTime === 'string' ? parseTime(endTime) : endTime

  if (!start || !end) return 0

  return Math.floor((end.getTime() - start.getTime()) / (1000 * 60))
}