import { ref, watch } from 'vue'
import type { LayoutState, FunctionPanel, WorkDirectory, DirectoryStats } from '@/types'
import { debounce } from '@/utils/debounce'

const LAYOUT_STORAGE_KEY = 'leaf-tidy-layout'
const WORK_DIRS_KEY = 'leaf-tidy-work-dirs'
const OUTPUT_DIR_KEY = 'leaf-tidy-output-dir'
const DEFAULT_LEFT_BAR_RATIO = 0.15
const DEFAULT_RIGHT_BAR_RATIO = 0.14
const MIN_LEFT_WIDTH = 200
const MAX_LEFT_WIDTH = 400
const MIN_RIGHT_WIDTH = 240
const MAX_RIGHT_WIDTH = 400

/// 安全读取 localStorage，解析失败或不可用时返回 fallback。
function safeGetJSON<T>(key: string, fallback: T): T {
  try {
    const saved = localStorage.getItem(key)
    if (saved) return JSON.parse(saved) as T
  } catch { /* localStorage 不可用时忽略 */ }
  return fallback
}

/// 安全读取 localStorage 原始字符串。
function safeGetString(key: string): string {
  try { return localStorage.getItem(key) ?? '' } catch { return '' }
}

/// 安全写入 localStorage。
function safeSetItem(key: string, value: unknown): void {
  try { localStorage.setItem(key, JSON.stringify(value)) } catch { /* localStorage 不可用时忽略 */ }
}

function loadLayout(): LayoutState {
  const parsed = safeGetJSON<Partial<LayoutState>>(LAYOUT_STORAGE_KEY, {})
  if (parsed.leftBarRatio !== undefined) {
    const windowWidth = window.innerWidth
    parsed.leftBarWidth = Math.max(MIN_LEFT_WIDTH, Math.min(MAX_LEFT_WIDTH, Math.round(windowWidth * parsed.leftBarRatio)))
    parsed.rightBarWidth = Math.max(MIN_RIGHT_WIDTH, Math.min(MAX_RIGHT_WIDTH, Math.round(windowWidth * (parsed.rightBarRatio ?? 0))))
    delete parsed.leftBarRatio
    delete parsed.rightBarRatio
  }
  return {
    showLeftBar: true,
    showRightBar: true,
    leftBarWidth: MIN_LEFT_WIDTH,
    rightBarWidth: MIN_RIGHT_WIDTH,
    ...parsed,
  }
}

function saveLayout(state: LayoutState) { safeSetItem(LAYOUT_STORAGE_KEY, state) }

function cleanPath(p: string): string {
  // 修复历史脏数据：路径被存成 JSON 字符串 "D:\\xxx" → D:\xxx
  if (p.startsWith('"') && p.endsWith('"')) {
    try { return JSON.parse(p) } catch { /* fall through */ }
  }
  return p
}

function loadWorkDirs(): WorkDirectory[] {
  const dirs = safeGetJSON<WorkDirectory[]>(WORK_DIRS_KEY, [])
  return dirs.filter(d => d && typeof d.path === 'string' && typeof d.name === 'string')
    .map(d => {
      const path = cleanPath(d.path.trim())
      const name = d.name.trim()
      return { path, name: name || path.split(/[/\\]/).pop() || path }
    })
    .filter(d => d.path)
}
function saveWorkDirs(dirs: WorkDirectory[]) { safeSetItem(WORK_DIRS_KEY, dirs) }

function loadOutputDir(): string {
  const v = safeGetString(OUTPUT_DIR_KEY)
  if (!v) return ''
  // 兼容历史数据：路径可能被 JSON.stringify 存成带引号的字符串
  if (v.startsWith('"') && v.endsWith('"')) {
    try { return JSON.parse(v) } catch { /* fall through */ }
  }
  return v
}
function saveOutputDir(path: string) {
  // 空值直接移除 key，避免 JSON.stringify('') 存入 '""' 导致下次加载为 truthy
  if (!path) {
    try { localStorage.removeItem(OUTPUT_DIR_KEY) } catch { /* ignore */ }
    return
  }
  safeSetItem(OUTPUT_DIR_KEY, path)
}

export function useLayout() {
  const layout = ref<LayoutState>(loadLayout())
  const activePanel = ref<FunctionPanel>('home')
  const workDirs = ref<WorkDirectory[]>(loadWorkDirs())
  const outputDir = ref<string>(loadOutputDir())
  const dirStats = ref<DirectoryStats | null>(null)
  const currentLogId = ref<string>('')
  const currentLogDate = ref<string>('')
  const isProcessing = ref(false)

  watch(layout, debounce(saveLayout, 200), { deep: true })
  watch(workDirs, debounce(saveWorkDirs, 200), { deep: true })
  watch(outputDir, debounce(saveOutputDir, 200))

  return {
    layout,
    activePanel,
    workDirs,
    outputDir,
    dirStats,
    currentLogId,
    currentLogDate,
    isProcessing,
  }
}