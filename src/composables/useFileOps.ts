import { invoke } from '@tauri-apps/api/core'
import { ElNotification } from 'element-plus'
import { t } from '@/i18n'
import type { DirectoryStats } from '@/types'

// 与后端 is_image_file 保持一致的图片格式
const IMAGE_EXTENSIONS = /\.(jpg|jpeg|png|gif|bmp|webp|tiff|tif|heic|heif)$/i
// 元数据写入/清除类后端（fix_date/strip_exif/write_gps）只处理 JPG
const JPG_EXTENSIONS = /\.(jpg|jpeg)$/i

export function getRelativePath(path: string, basePath: string): string {
  if (!path || !basePath) return path
  const normalizedPath = path.replace(/\\/g, '/')
  const normalizedBase = basePath.replace(/\\/g, '/')
  // 必须按路径分隔符边界匹配，否则 /a/bc 会被误判为 /a/b 下的文件
  const baseWithSep = normalizedBase.endsWith('/') ? normalizedBase : `${normalizedBase}/`
  if (normalizedPath.startsWith(baseWithSep)) {
    return normalizedPath.slice(baseWithSep.length)
  }
  return path
}

export function useFileOps() {
  const getDirectoryStats = (paths: string[]): Promise<DirectoryStats> => {
    return invoke<DirectoryStats>('get_directory_stats', { paths })
  }

  const getFileName = (path: string): string => {
    if (!path) return ''
    const parts = path.split(/[/\\]/)
    return parts[parts.length - 1] || path
  }

  const formatSize = (size: number): string => {
    if (size < 1024) return `${size} B`
    if (size < 1024 * 1024) return `${(size / 1024).toFixed(2)} KB`
    if (size < 1024 * 1024 * 1024) return `${(size / 1024 / 1024).toFixed(2)} MB`
    return `${(size / 1024 / 1024 / 1024).toFixed(2)} GB`
  }

  const openFile = async (path: string): Promise<void> => {
    if (!path) return
    try {
      await invoke('open_in_explorer', { path })
    } catch (e: any) {
      ElNotification({ type: 'error', title: t('app.error'), message: t('app.openFileFailed', { msg: String(e) }) })
    }
  }

  const collectImagePaths = (files: { path: string }[]): string[] =>
    files.filter(f => IMAGE_EXTENSIONS.test(f.path)).map(f => f.path)

  const collectJpgPaths = (files: { path: string }[]): string[] =>
    files.filter(f => JPG_EXTENSIONS.test(f.path)).map(f => f.path)

  return {
    getDirectoryStats,
    getFileName,
    formatSize,
    openFile,
    getRelativePath,
    collectImagePaths,
    collectJpgPaths,
  }
}
