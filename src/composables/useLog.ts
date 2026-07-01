import { invoke } from '@tauri-apps/api/core'

export interface LogParams {
  operation_type: string
  source_path: string
  target_path?: string
  status: 'success' | 'fail'
  detail?: string
}

export interface FileResult {
  source_path: string
  target_path?: string
  new_name?: string
  success: boolean
  error?: string
  // 元数据信息
  metadata?: FileMetadata
  // 处理耗时（毫秒）
  process_time_ms?: number
}

export interface FileMetadata {
  // 时间信息
  modified?: string
  created?: string
  taken?: string
  // GPS信息
  gps_latitude?: number
  gps_longitude?: number
  gps_province?: string
  gps_city?: string
  gps_district?: string
  gps_place?: string
  // EXIF信息
  camera_make?: string
  camera_model?: string
  // 文件信息
  size?: number
  format?: string
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

export function useLog() {
  /**
   * 写入单条日志
   */
  async function writeLog(params: LogParams): Promise<void> {
    try {
      await invoke('write_log', {
        operationType: params.operation_type,
        sourcePath: params.source_path,
        targetPath: params.target_path || null,
        status: params.status,
        detail: params.detail || null,
      })
    } catch (e) {
      console.error('写入日志失败:', e)
    }
  }

  /**
   * 格式化时间戳
   */
  function formatTimestamp(): string {
    const now = new Date()
    return now.toLocaleString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
      hour12: false
    }) + '.' + now.getMilliseconds().toString().padStart(3, '0')
  }

  /**
   * 格式化用户设置参数
   */
  function formatSettings(options: OrganizeLogOptions | RenameLogOptions, operationType: string): string {
    const lines: string[] = []
    lines.push('用户设置参数：')
    
    if (operationType === 'organize') {
      const opts = options as OrganizeLogOptions
      const timeSourceLabels: Record<string, string> = {
        modified: '修改时间',
        created: '创建时间',
        taken: '拍摄日期',
      }
      const tagLabels: Record<string, string> = {
        type: '文件类型',
        year: '年份',
        month: '月份',
        day: '日',
        date: '日期',
        province: '省份',
        city: '城市',
        district: '区县',
        make: '相机品牌',
        model: '相机型号',
        ext: '扩展名',
        size: '文件大小',
      }
      lines.push(`  时间数据来源: ${timeSourceLabels[opts.timeSource] || opts.timeSource}`)
      lines.push(`  分类标签: ${opts.tags.map(t => tagLabels[t] || t).join(' > ')}`)
      lines.push(`  源目录: ${opts.sourceDirs.join(', ')}`)
      lines.push(`  目标目录: ${opts.targetDir}`)
    } else if (operationType === 'rename') {
      const opts = options as RenameLogOptions
      const timeSourceLabels: Record<string, string> = {
        modified: '修改时间',
        created: '创建时间',
        taken: '拍摄日期',
      }
      lines.push(`  时间数据来源: ${timeSourceLabels[opts.timeSource] || opts.timeSource}`)
      lines.push(`  重命名模板: ${opts.template}`)
      lines.push(`  起始序号: ${opts.startIndex}`)
      lines.push(`  源文件: ${opts.sourcePaths.length} 个`)
      lines.push(`  目标目录: ${opts.targetDir}`)
    }
    
    return lines.join('\n')
  }

  /**
   * 格式化文件元数据
   */
  function formatMetadata(meta: FileMetadata | undefined, processTimeMs?: number): string {
    const parts: string[] = []
    
    // 处理耗时（放在最前面）
    if (processTimeMs !== undefined) {
      if (processTimeMs < 1000) {
        parts.push(`耗时=${processTimeMs}ms`)
      } else {
        parts.push(`耗时=${(processTimeMs / 1000).toFixed(2)}s`)
      }
    }
    
    if (!meta) return parts.length > 0 ? ` [${parts.join(', ')}]` : ''
    
    // 时间信息
    if (meta.modified) parts.push(`修改时间=${meta.modified}`)
    if (meta.created) parts.push(`创建时间=${meta.created}`)
    if (meta.taken) parts.push(`拍摄日期=${meta.taken}`)
    
    // GPS信息
    if (meta.gps_latitude && meta.gps_longitude) {
      parts.push(`GPS坐标=${meta.gps_latitude.toFixed(6)},${meta.gps_longitude.toFixed(6)}`)
      if (meta.gps_province) parts.push(`省份=${meta.gps_province}`)
      if (meta.gps_city) parts.push(`城市=${meta.gps_city}`)
      if (meta.gps_district) parts.push(`区县=${meta.gps_district}`)
      if (meta.gps_place) parts.push(`地点=${meta.gps_place}`)
    }
    
    // EXIF信息
    if (meta.camera_make) parts.push(`相机品牌=${meta.camera_make}`)
    if (meta.camera_model) parts.push(`相机型号=${meta.camera_model}`)
    
    // 文件信息
    if (meta.size) parts.push(`文件大小=${meta.size}字节`)
    if (meta.format) parts.push(`文件格式=${meta.format}`)
    
    return parts.length > 0 ? ` [${parts.join(', ')}]` : ''
  }

  /**
   * 格式化文件列表为文本格式
   */
  function formatFileList(results: FileResult[], operationType: string, targetDir?: string): string {
    const lines: string[] = []
    const successCount = results.filter(r => r.success).length
    const failCount = results.filter(r => !r.success).length
    
    // 计算总耗时
    const totalTimeMs = results.reduce((sum, r) => sum + (r.process_time_ms || 0), 0)
    
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
    
    results.forEach((r, index) => {
      const status = r.success ? '成功' : '失败'
      const metaInfo = formatMetadata(r.metadata, r.process_time_ms)
      
      if (operationType === 'rename') {
        lines.push(`[${status}] ${r.source_path} -> ${r.new_name || '未知'} -> ${r.target_path || '未知'}${metaInfo}`)
      } else if (operationType === 'duplicate_clean' || operationType === 'cleanup') {
        lines.push(`[${status}] ${r.source_path}${metaInfo}`)
      } else {
        // 对于文件整理，显示相对路径而不是绝对路径
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

  /**
   * 获取相对路径
   */
  function getRelativePath(fullPath: string, basePath: string): string {
    // 标准化路径分隔符
    const normalizedFull = fullPath.replace(/\\/g, '/')
    const normalizedBase = basePath.replace(/\\/g, '/')
    
    // 确保基础路径以 / 结尾
    const baseWithSlash = normalizedBase.endsWith('/') ? normalizedBase : normalizedBase + '/'
    
    // 如果完整路径以基础路径开头，返回相对路径
    if (normalizedFull.startsWith(baseWithSlash)) {
      return normalizedFull.substring(baseWithSlash.length)
    }
    
    // 如果完整路径以基础路径开头（不带斜杠）
    if (normalizedFull.startsWith(normalizedBase)) {
      const remaining = normalizedFull.substring(normalizedBase.length)
      // 去掉开头的斜杠
      return remaining.startsWith('/') ? remaining.substring(1) : remaining
    }
    
    // 否则返回完整路径
    return fullPath
  }

  /**
   * 写入文件整理日志 - 一次任务一条
   */
  async function logOrganizeResults(results: FileResult[], options: OrganizeLogOptions): Promise<void> {
    const successCount = results.filter(r => r.success).length
    const failCount = results.filter(r => !r.success).length
    const status: 'success' | 'fail' = failCount === 0 ? 'success' : (successCount > 0 ? 'success' : 'fail')
    
    // 获取处理范围和输出目录
    const sourcePaths = results.map(r => r.source_path)
    const targetPaths = results.filter(r => r.target_path).map(r => r.target_path!)
    
    // 提取公共源目录和目标目录
    const sourceDir = extractCommonDir(sourcePaths)
    const targetDir = extractCommonDir(targetPaths)
    
    const settingsInfo = formatSettings(options, 'organize')
    const detailInfo = formatFileList(results, 'organize', targetDir || undefined)
    const detail = `${settingsInfo}\n\n${detailInfo}`
    
    await writeLog({
      operation_type: 'organize',
      source_path: sourceDir ? `${sourceDir} (共${results.length}个文件)` : `共处理${results.length}个文件`,
      target_path: targetDir || undefined,
      status,
      detail,
    })
  }

  /**
   * 写入批量重命名日志 - 一次任务一条
   */
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
    
    await writeLog({
      operation_type: 'rename',
      source_path: sourceDir ? `${sourceDir} (共${results.length}个文件)` : `共处理${results.length}个文件`,
      target_path: targetDir || undefined,
      status,
      detail,
    })
  }

  /**
   * 写入重复文件清理日志 - 一次任务一条
   */
  async function logDuplicateCleanResults(results: FileResult[]): Promise<void> {
    const successCount = results.filter(r => r.success).length
    const failCount = results.filter(r => !r.success).length
    const status: 'success' | 'fail' = failCount === 0 ? 'success' : (successCount > 0 ? 'success' : 'fail')
    
    const sourcePaths = results.map(r => r.source_path)
    const sourceDir = extractCommonDir(sourcePaths)
    
    const detail = formatFileList(results, 'duplicate_clean')
    
    await writeLog({
      operation_type: 'duplicate_clean',
      source_path: sourceDir ? `${sourceDir} (共${results.length}个文件)` : `共处理${results.length}个文件`,
      status,
      detail,
    })
  }

  /**
   * 写入附属文件清理日志 - 一次任务一条
   */
  async function logCleanupResults(results: FileResult[]): Promise<void> {
    const successCount = results.filter(r => r.success).length
    const failCount = results.filter(r => !r.success).length
    const status: 'success' | 'fail' = failCount === 0 ? 'success' : (successCount > 0 ? 'success' : 'fail')
    
    const sourcePaths = results.map(r => r.source_path)
    const sourceDir = extractCommonDir(sourcePaths)
    
    const detail = formatFileList(results, 'cleanup')
    
    await writeLog({
      operation_type: 'cleanup',
      source_path: sourceDir ? `${sourceDir} (共${results.length}个文件)` : `共处理${results.length}个文件`,
      status,
      detail,
    })
  }

  /**
   * 提取公共目录路径
   */
  function extractCommonDir(paths: string[]): string | null {
    if (paths.length === 0) return null
    if (paths.length === 1) {
      const parts = paths[0].split(/[/\\]/)
      return parts.slice(0, -1).join('/')
    }
    
    // 找到所有路径的共同前缀
    const firstParts = paths[0].split(/[/\\]/)
    let commonLength = firstParts.length - 1 // 去掉文件名
    
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

  return {
    writeLog,
    formatFileList,
    formatSettings,
    formatMetadata,
    logOrganizeResults,
    logRenameResults,
    logDuplicateCleanResults,
    logCleanupResults,
  }
}