import { invoke } from '@tauri-apps/api/core'
import { ElNotification } from 'element-plus'
import { t } from '@/i18n'
import type { DirectoryStats } from '@/types'

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

  const getRelativePath = (path: string, basePath: string): string => {
    if (!path || !basePath) return path
    const normalizedPath = path.replace(/\\/g, '/')
    const normalizedBase = basePath.replace(/\\/g, '/')
    if (normalizedPath.startsWith(normalizedBase)) {
      let relative = normalizedPath.slice(normalizedBase.length)
      if (relative.startsWith('/')) relative = relative.slice(1)
      return relative
    }
    return path
  }

  return {
    getDirectoryStats,
    getFileName,
    formatSize,
    openFile,
    getRelativePath,
  }
}
