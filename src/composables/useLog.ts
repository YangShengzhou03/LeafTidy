import { invoke } from '@tauri-apps/api/core'
import { getRelativePath } from '@/composables/useFileOps'
import type { LogDaySummary, LogEntry, OrganizeMetadata } from '@/types'

export interface FileResult {
  source_path: string
  target_path?: string
  new_name?: string
  success: boolean
  error?: string
  metadata?: OrganizeMetadata
  process_time_ms?: number
}

export interface OrganizeLogOptions {
  timeSource: string
  tags: string[]
  sourceDirs: string[]
  targetDir: string
}

export interface RenameLogOptions {
  timeSource: string
  template: string
  startIndex: number
  sourcePaths: string[]
  targetDir: string
}

const TIME_SOURCE_LABELS: Record<string, string> = {
  modified: '修改时间',
  created: '创建时间',
  taken: '拍摄日期',
}

const TAG_LABELS: Record<string, string> = {
  type: '文件类型',
  year: '年份',
  month: '月份',
  day: '日',
  date: '日期',
  province: '省份',
  city: '城市',
  district: '区县',
  place: '地点',
  make: '相机品牌',
  model: '相机型号',
  ext: '扩展名',
  size: '文件大小',
}

export function useLog() {
  async function writeEvent(action: string, size: number = 0): Promise<void> {
    try {
      await invoke('write_event', { action, size })
    } catch {
      // 静默失败，日志不应影响主流程
    }
  }

  function formatSettings(options: OrganizeLogOptions | RenameLogOptions, operationType: string): string {
    const lines: string[] = []
    lines.push('用户设置参数：')

    if (operationType === 'organize') {
      const opts = options as OrganizeLogOptions
      lines.push(`  时间数据来源: ${TIME_SOURCE_LABELS[opts.timeSource] || opts.timeSource}`)
      lines.push(`  分类标签: ${opts.tags.map(t => TAG_LABELS[t] || t).join(' > ')}`)
      lines.push(`  源目录: ${opts.sourceDirs.join(', ')}`)
      lines.push(`  目标目录: ${opts.targetDir}`)
    } else if (operationType === 'rename') {
      const opts = options as RenameLogOptions
      lines.push(`  时间数据来源: ${TIME_SOURCE_LABELS[opts.timeSource] || opts.timeSource}`)
      lines.push(`  重命名模板: ${opts.template}`)
      lines.push(`  起始序号: ${opts.startIndex}`)
      lines.push(`  源文件: ${opts.sourcePaths.length} 个`)
      lines.push(`  目标目录: ${opts.targetDir}`)
    }
    
    return lines.join('\n')
  }

  function formatMetadata(meta: OrganizeMetadata | undefined, processTimeMs?: number): string {
    const parts: string[] = []
    
    if (processTimeMs !== undefined) {
      if (processTimeMs < 1000) {
        parts.push(`耗时=${processTimeMs}ms`)
      } else {
        parts.push(`耗时=${(processTimeMs / 1000).toFixed(2)}s`)
      }
    }
    
    if (!meta) return parts.length > 0 ? ` [${parts.join(', ')}]` : ''
    if (meta.modified) parts.push(`修改时间=${meta.modified}`)
    if (meta.created) parts.push(`创建时间=${meta.created}`)
    if (meta.taken) parts.push(`拍摄日期=${meta.taken}`)
    
    if (meta.gps_latitude && meta.gps_longitude) {
      parts.push(`GPS坐标=${meta.gps_latitude.toFixed(6)},${meta.gps_longitude.toFixed(6)}`)
      if (meta.gps_province) parts.push(`省份=${meta.gps_province}`)
      if (meta.gps_city) parts.push(`城市=${meta.gps_city}`)
      if (meta.gps_district) parts.push(`区县=${meta.gps_district}`)
      if (meta.gps_place) parts.push(`地点=${meta.gps_place}`)
    }
    if (meta.camera_make) parts.push(`相机品牌=${meta.camera_make}`)
    if (meta.camera_model) parts.push(`相机型号=${meta.camera_model}`)
    if (meta.size) parts.push(`文件大小=${meta.size}字节`)
    if (meta.format) parts.push(`文件格式=${meta.format}`)
    
    return parts.length > 0 ? ` [${parts.join(', ')}]` : ''
  }

  function formatFileList(results: FileResult[], operationType: string, targetDir?: string): string {
    const lines: string[] = []
    const { successCount, failCount, totalTimeMs } = results.reduce(
      (acc, r) => {
        if (r.success) acc.successCount++
        else acc.failCount++
        acc.totalTimeMs += r.process_time_ms || 0
        return acc
      },
      { successCount: 0, failCount: 0, totalTimeMs: 0 }
    )
    
    lines.push(`任务执行结果统计：`)
    lines.push(`成功: ${successCount} 个文件`)
    lines.push(`失败: ${failCount} 个文件`)
    if (totalTimeMs > 0) {
      if (totalTimeMs < 1000) {
        lines.push(`总耗时: ${totalTimeMs}ms`)
      } else {
        lines.push(`总耗时: ${(totalTimeMs / 1000).toFixed(2)}s`)
      }
    }
    lines.push(``)
    lines.push(`详细文件列表：`)
    lines.push(``)
    
    results.forEach((r) => {
      const status = r.success ? '成功' : '失败'
      const metaInfo = formatMetadata(r.metadata, r.process_time_ms)
      
      if (operationType === 'rename') {
        lines.push(`[${status}] ${r.source_path} -> ${r.new_name || '未知'} -> ${r.target_path || '未知'}${metaInfo}`)
      } else if (operationType === 'duplicate_clean' || operationType === 'cleanup') {
        lines.push(`[${status}] ${r.source_path}${metaInfo}`)
      } else {
        const displayPath = r.target_path && targetDir 
          ? getRelativePath(r.target_path, targetDir) 
          : (r.target_path || '未知')
        lines.push(`[${status}] ${r.source_path} -> ${displayPath}${metaInfo}`)
      }
      if (!r.success && r.error) {
        lines.push(`  错误: ${r.error}`)
      }
    })
    
    return lines.join('\n')
  }

  function extractCommonDir(paths: string[]): string | null {
    if (paths.length === 0) return null
    if (paths.length === 1) {
      const parts = paths[0].split(/[/\\]/)
      return parts.slice(0, -1).join('/')
    }
    
    const firstParts = paths[0].split(/[/\\]/)
    let commonLength = firstParts.length - 1
    
    for (let i = 1; i < paths.length; i++) {
      const parts = paths[i].split(/[/\\]/)
      let j = 0
      while (j < Math.min(commonLength, parts.length - 1) && firstParts[j] === parts[j]) {
        j++
      }
      commonLength = Math.min(commonLength, j)
    }
    
    if (commonLength === 0) return null
    return firstParts.slice(0, commonLength).join('/')
  }



  async function logOrganizeResults(results: FileResult[], options: OrganizeLogOptions): Promise<void> {
    const successCount = results.filter(r => r.success).length
    const failCount = results.filter(r => !r.success).length
    const status: 'success' | 'fail' = failCount === 0 ? 'success' : (successCount > 0 ? 'success' : 'fail')
    
    const sourcePaths = results.map(r => r.source_path)
    const targetPaths = results.filter(r => r.target_path).map(r => r.target_path!)
    const sourceDir = extractCommonDir(sourcePaths)
    const targetDir = extractCommonDir(targetPaths)
    
    const settingsInfo = formatSettings(options, 'organize')
    const detailInfo = formatFileList(results, 'organize', targetDir || undefined)
    const detail = `${settingsInfo}\n\n${detailInfo}`
    const totalSize = results.reduce((sum, r) => sum + (r.metadata?.size || 0), 0)
    const sourceDesc = sourceDir ? `${sourceDir} (共${results.length}个文件)` : `共处理${results.length}个文件`

    await writeEvent(
      `整理: ${sourceDesc} -> ${targetDir || 'N/A'} [${status}] ${detail}`,
      totalSize
    )
  }

  async function logRenameResults(results: FileResult[], options: RenameLogOptions): Promise<void> {
    const successCount = results.filter(r => r.success).length
    const failCount = results.filter(r => !r.success).length
    const status: 'success' | 'fail' = failCount === 0 ? 'success' : (successCount > 0 ? 'success' : 'fail')
    
    const sourcePaths = results.map(r => r.source_path)
    const targetPaths = results.filter(r => r.target_path).map(r => r.target_path!)
    
    const sourceDir = extractCommonDir(sourcePaths)
    const targetDir = extractCommonDir(targetPaths)
    
    const settingsInfo = formatSettings(options, 'rename')
    const detailInfo = formatFileList(results, 'rename')
    const detail = `${settingsInfo}\n\n${detailInfo}`
    const totalSize = results.reduce((sum, r) => sum + (r.metadata?.size || 0), 0)
    const sourceDesc = sourceDir ? `${sourceDir} (共${results.length}个文件)` : `共处理${results.length}个文件`

    await writeEvent(
      `重命名: ${sourceDesc} -> ${targetDir || 'N/A'} [${status}] ${detail}`,
      totalSize
    )
  }

  async function logDuplicateCleanResults(results: FileResult[]): Promise<void> {
    const successCount = results.filter(r => r.success).length
    const failCount = results.filter(r => !r.success).length
    const status: 'success' | 'fail' = failCount === 0 ? 'success' : (successCount > 0 ? 'success' : 'fail')
    
    const sourcePaths = results.map(r => r.source_path)
    const sourceDir = extractCommonDir(sourcePaths)
    const detail = formatFileList(results, 'duplicate_clean')
    const totalSize = results.reduce((sum, r) => sum + (r.metadata?.size || 0), 0)
    const sourceDesc = sourceDir ? `${sourceDir} (共${results.length}个文件)` : `共处理${results.length}个文件`

    await writeEvent(
      `清理重复: ${sourceDesc} [${status}] ${detail}`,
      totalSize
    )
  }

  async function logLowQualityResults(results: FileResult[]): Promise<void> {
    const successCount = results.filter(r => r.success).length
    const failCount = results.filter(r => !r.success).length
    const status: 'success' | 'fail' = failCount === 0 ? 'success' : (successCount > 0 ? 'success' : 'fail')
    
    const sourcePaths = results.map(r => r.source_path)
    const sourceDir = extractCommonDir(sourcePaths)
    const detail = formatFileList(results, 'cleanup')
    const totalSize = results.reduce((sum, r) => sum + (r.metadata?.size || 0), 0)
    const sourceDesc = sourceDir ? `${sourceDir} (共${results.length}个文件)` : `共处理${results.length}个文件`

    await writeEvent(
      `清理: ${sourceDesc} [${status}] ${detail}`,
      totalSize
    )
  }

  async function logImageProcessResults(
    operationType: string,
    results: FileResult[],
    options: { params: Record<string, string>; targetDir?: string },
  ): Promise<void> {
    const successCount = results.filter(r => r.success).length
    const failCount = results.filter(r => !r.success).length
    const status: 'success' | 'fail' = failCount === 0 ? 'success' : (successCount > 0 ? 'success' : 'fail')

    const lines: string[] = ['用户设置参数：']
    for (const [key, value] of Object.entries(options.params)) {
      lines.push(`  ${key}: ${value}`)
    }
    lines.push('')
    lines.push(formatFileList(results, 'image_process', options.targetDir))
    const totalSize = results.reduce((sum, r) => sum + (r.metadata?.size || 0), 0)

    await writeEvent(
      `${operationType}: 共处理${results.length}个文件 -> ${options.targetDir || 'N/A'} [${status}] ${lines.join('\n')}`,
      totalSize
    )
  }

  async function logCancelledOperation(
    operationType: string,
    sourcePath: string,
    detail?: string,
  ): Promise<void> {
    await writeEvent(`取消: ${operationType} - ${sourcePath} (${detail || '用户手动终止操作'})`)
  }

  async function logOperationStart(
    operationName: string,
    params: Record<string, string | number | boolean>,
  ): Promise<void> {
    const parts: string[] = [`开始: ${operationName}`]
    const paramLines: string[] = []
    for (const [key, value] of Object.entries(params)) {
      paramLines.push(`  ${key}: ${value}`)
    }
    if (paramLines.length > 0) {
      parts.push('参数:')
      parts.push(...paramLines)
    }
    await writeEvent(parts.join('\n'))
  }

  async function logDirectoryOperation(
    operation: string,
    dirs: string[],
    extra?: Record<string, string | number>,
  ): Promise<void> {
    const parts: string[] = [`${operation}: ${dirs.length} 个目录`]
    for (const dir of dirs) {
      parts.push(`  - ${dir}`)
    }
    if (extra) {
      for (const [key, value] of Object.entries(extra)) {
        parts.push(`  ${key}: ${value}`)
      }
    }
    await writeEvent(parts.join('\n'))
  }

  async function queryLogsByDate(date: string): Promise<LogEntry[]> {
    try {
      return await invoke('query_logs_by_date', { date })
    } catch {
      return []
    }
  }

  async function queryDailySummary(): Promise<LogDaySummary[]> {
    try {
      const result = await invoke<[string, number, number][]>('query_daily_summary')
      return result.map(([date, count, totalSize]: [string, number, number]) => ({ date, count, totalSize }))
    } catch {
      return []
    }
  }

  return {
    logEvent: writeEvent,
    queryLogsByDate,
    queryDailySummary,
    logOrganizeResults,
    logRenameResults,
    logDuplicateCleanResults,
    logLowQualityResults,
    logImageProcessResults,
    logCancelledOperation,
    logOperationStart,
    logDirectoryOperation,
  }
}
