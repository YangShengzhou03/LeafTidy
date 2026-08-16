import { ref, watch } from 'vue'
import type { LayoutState, FunctionPanel, WorkDirectory, DirectoryStats, PageState } from '@/types'

const LAYOUT_STORAGE_KEY = 'leaf-tidy-layout'
const WORK_DIRS_KEY = 'leaf-tidy-work-dirs'
const OUTPUT_DIR_KEY = 'leaf-tidy-output-dir'
const PAGE_STATE_KEY = 'leaf-tidy-page-state'
const SETTINGS_KEY = 'leaf-tidy-settings'
const DEFAULT_LEFT_BAR_RATIO = 0.15
const DEFAULT_RIGHT_BAR_RATIO = 0.14
const MIN_LEFT_WIDTH = 200
const MAX_LEFT_WIDTH = 400
const MIN_RIGHT_WIDTH = 240
const MAX_RIGHT_WIDTH = 400

function loadLayout(): LayoutState {
  try {
    const saved = localStorage.getItem(LAYOUT_STORAGE_KEY)
    if (saved) {
      const parsed = JSON.parse(saved)
      if (parsed.leftBarRatio !== undefined) {
        const windowWidth = window.innerWidth
        parsed.leftBarWidth = Math.max(MIN_LEFT_WIDTH, Math.min(MAX_LEFT_WIDTH, Math.round(windowWidth * parsed.leftBarRatio)))
        parsed.rightBarWidth = Math.max(MIN_RIGHT_WIDTH, Math.min(MAX_RIGHT_WIDTH, Math.round(windowWidth * parsed.rightBarRatio)))
        delete parsed.leftBarRatio
        delete parsed.rightBarRatio
      }
      return {
        showLeftBar: true,
        showRightBar: true,
        leftBarWidth: 300,
        rightBarWidth: 280,
        ...parsed,
      }
    }
  } catch { }
  const windowWidth = window.innerWidth
  return {
    showLeftBar: true,
    showRightBar: true,
    leftBarWidth: Math.max(MIN_LEFT_WIDTH, Math.min(MAX_LEFT_WIDTH, Math.round(windowWidth * DEFAULT_LEFT_BAR_RATIO))),
    rightBarWidth: Math.max(MIN_RIGHT_WIDTH, Math.min(MAX_RIGHT_WIDTH, Math.round(windowWidth * DEFAULT_RIGHT_BAR_RATIO))),
  }
}

function saveLayout(state: LayoutState) {
  try { localStorage.setItem(LAYOUT_STORAGE_KEY, JSON.stringify(state)) } catch { }
}

function loadWorkDirs(): WorkDirectory[] {
  try {
    const saved = localStorage.getItem(WORK_DIRS_KEY)
    if (saved) return JSON.parse(saved)
  } catch { }
  return []
}

function saveWorkDirs(dirs: WorkDirectory[]) {
  try { localStorage.setItem(WORK_DIRS_KEY, JSON.stringify(dirs)) } catch { }
}

function loadOutputDir(): string {
  try {
    const saved = localStorage.getItem(OUTPUT_DIR_KEY)
    if (saved) return saved
  } catch { }
  return ''
}

function saveOutputDir(path: string) {
  try { localStorage.setItem(OUTPUT_DIR_KEY, path) } catch { }
}

function loadPageState(): Record<string, PageState> {
  try {
    const saved = localStorage.getItem(PAGE_STATE_KEY)
    if (saved) return JSON.parse(saved)
  } catch { }
  return {}
}

function savePageState(state: Record<string, PageState>) {
  try { localStorage.setItem(PAGE_STATE_KEY, JSON.stringify(state)) } catch { }
}



export function useLayout() {
  const layout = ref<LayoutState>(loadLayout())
  const activePanel = ref<FunctionPanel>('home')
  const workDirs = ref<WorkDirectory[]>(loadWorkDirs())
  const outputDir = ref<string>(loadOutputDir())
  const dirStats = ref<DirectoryStats | null>(null)
  const currentLogId = ref<string>('')
  const pageStates = ref<Record<string, PageState>>(loadPageState())
  const isProcessing = ref(false)

  watch(layout, saveLayout, { deep: true })
  watch(workDirs, saveWorkDirs, { deep: true })
  watch(outputDir, saveOutputDir)
  watch(pageStates, savePageState, { deep: true })

  function savePageStateData(panel: FunctionPanel, state: PageState) {
    pageStates.value[panel] = state
  }

  function getPageStateData(panel: FunctionPanel): PageState | undefined {
    return pageStates.value[panel]
  }

  return {
    layout,
    activePanel,
    workDirs,
    outputDir,
    dirStats,
    currentLogId,
    pageStates,
    isProcessing,
    savePageStateData,
    getPageStateData,
  }
}