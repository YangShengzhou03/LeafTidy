import { invoke } from '@tauri-apps/api/core'
import { ElMessage } from 'element-plus'
import type { FileEntry, DirEntry, DirectoryStats } from '@/types'

export function useFileOps() {
  const listDirectory = (path: string): Promise<FileEntry[]> => {
    return invoke<FileEntry[]>('list_directory', { path })
  }

  const listSubdirs = (path: string): Promise<DirEntry[]> => {
    return invoke<DirEntry[]>('list_subdirs', { path })
  }

  const getDirectoryStats = (paths: string[]): Promise<DirectoryStats> => {
    return invoke<DirectoryStats>('get_directory_stats', { paths })
  }

  /**
   * 从路径中提取文件名
   */
  const getFileName = (path: string): string => {
    if (!path) return ''
    const parts = path.split(/[/\\]/)
    return parts[parts.length - 1] || path
  }

  /**
   * 格式化文件大小
   */
  const formatSize = (size: number): string => {
    if (size < 1024) return `${size} B`
    if (size < 1024 * 1024) return `${(size / 1024).toFixed(2)} KB`
    if (size < 1024 * 1024 * 1024) return `${(size / 1024 / 1024).toFixed(2)} MB`
    return `${(size / 1024 / 1024 / 1024).toFixed(2)} GB`
  }

  /**
   * 在资源管理器中打开文件
   */
  const openFile = async (path: string): Promise<void> => {
    if (!path) return
    try {
      await invoke('open_in_explorer', { path })
    } catch (e: any) {
      ElMessage.error(`打开文件失败: ${e}`)
    }
  }

  /**
   * 获取相对路径（相对于输出目录）
   */
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
    listDirectory,
    listSubdirs,
    getDirectoryStats,
    getFileName,
    formatSize,
    openFile,
    getRelativePath,
  }
}
