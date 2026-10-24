<template>
  <el-config-provider :locale="epLocale">
    <div class="app">
    <div class="main">
      <div class="sidebar-left" :style="{ width: layout.leftBarWidth + 'px' }">
        <div class="sidebar-header" @mousedown="startDragging">
          <div class="sidebar-title">
            <span class="title-leaf">Leaf</span><span class="title-tidy">Tidy</span>
          </div>
        </div>
        <LeftSidebar />
        <div class="resize-handle-left" @mousedown.stop="startResizeLeft"></div>
      </div>

      <div class="workspace">
        <div class="workspace-header" @mousedown="startDragging">
          <div class="toolbar-left">
            <el-button class="btn-nav" :disabled="!canGoBack" @mousedown.stop @click.stop="goBack" :title="t('app.back')">
              <el-icon>
                <ArrowLeft />
              </el-icon>
            </el-button>
            <el-button class="btn-nav" :disabled="!canGoForward" @mousedown.stop @click.stop="goForward" :title="t('app.forward')">
              <el-icon>
                <ArrowRight />
              </el-icon>
            </el-button>
          </div>
          <div class="toolbar-right">
            <div v-if="geoStatus === 'loading'" class="geo-status loading">
              <el-icon class="loading-icon"><svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg">
                  <path fill="currentColor"
                    d="M512 896c212.864 0 384-171.136 384-384s-171.136-384-384-384-384 171.136-384 384 171.136 384 384 384z m0-704c177.664 0 320 142.336 320 320s-142.336 320-320 320-320-142.336-320-320 142.336-320 320-320z"
                    opacity=".3" />
                  <path fill="currentColor"
                    d="M512 640a128 128 0 1 0-128-128 128 128 0 0 0 128 128zm0-192a64 64 0 1 1-64 64 64 64 0 0 1 64-64z" />
                </svg></el-icon>
              <span>{{ t('app.geoLoading') }}</span>
            </div>
            <div v-else-if="geoStatus === 'ready'" class="geo-status ready">
              <el-icon>
                <SuccessFilled />
              </el-icon>
              <span>{{ t('app.geoReady') }}</span>
            </div>
            <div v-else-if="geoStatus === 'failed'" class="geo-status failed">
              <el-icon>
                <CircleCloseFilled />
              </el-icon>
              <span>{{ t('app.geoFailed') }}</span>
            </div>
          </div>
        </div>
        <div class="workspace-body">
          <KeepAlive :max="20">
            <component :is="currentPage" />
          </KeepAlive>
        </div>
      </div>

      <div class="sidebar-right" :style="{ width: layout.rightBarWidth + 'px' }">
        <div class="sidebar-header" @mousedown="startDragging">
          <div class="window-btns">
            <el-button class="btn-win" @mousedown.stop @click.stop="minimizeWindow">
              <el-icon>
                <Minus />
              </el-icon>
            </el-button>
            <el-button class="btn-win" @mousedown.stop @click.stop="maximizeWindow">
              <el-icon v-if="!isMaximized">
                <svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg">
                  <path fill="currentColor" d="M160 160v704h704V160H160zm64 64h576v576H224V224z" />
                </svg>
              </el-icon>
              <el-icon v-else>
                <svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg">
                  <path fill="currentColor" d="M320 320v544h544V320H320zm-64 64h512v512H256V384z" />
                  <path fill="currentColor" d="M128 128v512h64V192h448v-64H128z" />
                </svg>
              </el-icon>
            </el-button>
            <el-button class="btn-win close" @mousedown.stop @click.stop="closeWindow">
              <el-icon>
                <Close />
              </el-icon>
            </el-button>
          </div>
        </div>
        <RightSidebar />
        <div class="resize-handle-right" @mousedown.stop="startResizeRight"></div>
      </div>
    </div>
  </div>
    <!-- 启动体验反馈：3 星及以上引导前往 GitHub 开源页面 -->
    <el-dialog
      v-model="rateDialogVisible"
      :title="t('app.rate.title')"
      width="360px"
      :append-to-body="true"
      align-center
    >
      <div class="rate-body">
        <p class="rate-question">{{ t('app.rate.question') }}</p>
        <el-rate v-model="rating" :max="5" size="large" />
      </div>
      <template #footer>
        <el-button type="primary" :disabled="rating === 0" @click="submitRating">
          {{ t('app.ok') }}
        </el-button>
      </template>
    </el-dialog>
  </el-config-provider>
</template>

<script setup lang="ts">
import { ref, provide, computed, onMounted, onUnmounted, watch, defineAsyncComponent } from 'vue'
import { Close, Minus, ArrowLeft, ArrowRight, SuccessFilled, CircleCloseFilled } from '@element-plus/icons-vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { ElMessage, ElMessageBox, ElNotification, ElRate } from 'element-plus'
import zhCn from 'element-plus/dist/locale/zh-cn.mjs'
import zhTw from 'element-plus/dist/locale/zh-tw.mjs'
import en from 'element-plus/dist/locale/en.mjs'
import LeftSidebar from './components/LeftSidebar.vue'
import RightSidebar from './components/RightSidebar.vue'
// 页面按需异步加载，避免首屏一次性打包全部 15 个页面
const HomePage = defineAsyncComponent(() => import('./views/HomePage.vue'))
const FileOrganizePage = defineAsyncComponent(() => import('./views/FileOrganizePage.vue'))
const BatchRenamePage = defineAsyncComponent(() => import('./views/BatchRenamePage.vue'))
const DuplicateCleanPage = defineAsyncComponent(() => import('./views/DuplicateCleanPage.vue'))
const CleanupPage = defineAsyncComponent(() => import('./views/CleanupPage.vue'))
const FixDatePage = defineAsyncComponent(() => import('./views/FixDatePage.vue'))
const ExifCleanPage = defineAsyncComponent(() => import('./views/ExifCleanPage.vue'))
const WriteGpsPage = defineAsyncComponent(() => import('./views/WriteGpsPage.vue'))
const AiClassifyPage = defineAsyncComponent(() => import('./views/AiClassifyPage.vue'))
const LogViewPage = defineAsyncComponent(() => import('./views/LogViewPage.vue'))
const LogDetailPage = defineAsyncComponent(() => import('./views/LogDetailPage.vue'))
const SettingsPage = defineAsyncComponent(() => import('./views/SettingsPage.vue'))
const PrivacyPolicyPage = defineAsyncComponent(() => import('./views/PrivacyPolicyPage.vue'))
const LicenseAgreementPage = defineAsyncComponent(() => import('./views/LicenseAgreementPage.vue'))
const AboutPage = defineAsyncComponent(() => import('./views/AboutPage.vue'))
import { useLayout } from '@/composables/useLayout'
import { useLog } from '@/composables/useLog'
import { loadSettings } from '@/utils/settings'
import { GITHUB_URL } from '@/utils/appInfo'
import { t, locale } from '@/i18n'
import { TASK_EVENT } from '@/constants/tasks'

// Element Plus 内部文案跟随当前语言实时切换
const epLocales = { 'zh-CN': zhCn, 'zh-TW': zhTw, 'en': en }
const epLocale = computed(() => epLocales[locale.value] ?? zhCn)
import type { FunctionPanel } from '@/types'

const { layout, activePanel, workDirs, outputDir, dirStats, currentLogId, currentLogDate, isProcessing } = useLayout()

provide('layout', layout)
provide('activePanel', activePanel)
provide('workDirs', workDirs)
provide('outputDir', outputDir)
provide('dirStats', dirStats)
provide('currentLogId', currentLogId)
provide('currentLogDate', currentLogDate)
provide('isProcessing', isProcessing)

const isMaximized = ref(false)
const resizing = ref<'left' | 'right' | null>(null)
const historyStack = ref<FunctionPanel[]>(['home'])
const historyIndex = ref(0)
// 历史上限：避免长时间使用后无界增长（仅存面板名字符串，取 50 足够回退）
const HISTORY_LIMIT = 50
const isNavigating = ref(false)
const geoStatus = ref<'loading' | 'ready' | 'failed'>('loading')
// 启动体验反馈
const rating = ref(0)
const rateDialogVisible = ref(false)
const RATE_SNOOZE_KEY = 'leaf-tidy-rate-last'
const RATE_SNOOZE_MS = 7 * 24 * 60 * 60 * 1000

// 3 星及以上视为满意：记录时间（7 天内不再打扰）并用系统浏览器打开 GitHub 开源页面
async function submitRating() {
  const stars = rating.value
  rateDialogVisible.value = false
  if (stars < 3) {
    ElMessage.success(t('app.rate.thanks'))
    return
  }
  try {
    localStorage.setItem(RATE_SNOOZE_KEY, String(Date.now()))
  } catch { /* localStorage 不可用时忽略 */ }
  try {
    await invoke('open_url', { url: GITHUB_URL })
  } catch (e) {
    ElNotification({ type: 'error', title: t('app.error'), message: t('app.openUrlFailed', { msg: String(e) }) })
  }
}

const canGoBack = computed(() => historyIndex.value > 0)
const canGoForward = computed(() => historyIndex.value < historyStack.value.length - 1)

const pageMap: Record<FunctionPanel, any> = {
  'home': HomePage,
  'file-organize': FileOrganizePage,
  'batch-rename': BatchRenamePage,
  'duplicate-clean': DuplicateCleanPage,
  'cleanup': CleanupPage,
  'fix-date': FixDatePage,
  'exif-clean': ExifCleanPage,
  'write-gps': WriteGpsPage,
  'ai-classify': AiClassifyPage,
  'log-view': LogViewPage,
  'log-detail': LogDetailPage,
  'settings': SettingsPage,
  'privacy-policy': PrivacyPolicyPage,
  'license-agreement': LicenseAgreementPage,
  'about': AboutPage,
}

const currentPage = computed(() => pageMap[activePanel.value] || HomePage)

const appWindow = getCurrentWindow()

// 关闭按钮行为由设置页下拉项决定：exit=退出程序，minimize=最小化窗口，tray=最小化到系统托盘
function loadCloseAction(): 'exit' | 'minimize' | 'tray' {
  return loadSettings().closeAction ?? 'exit'
}

async function startDragging() { await appWindow.startDragging() }
async function minimizeWindow() { await appWindow.minimize() }
async function maximizeWindow() {
  if (isMaximized.value) { await appWindow.unmaximize(); isMaximized.value = false }
  else { await appWindow.maximize(); isMaximized.value = true }
}
async function closeWindow() {
  if (isProcessing.value) {
    ElNotification({ type: 'warning', title: t('app.warning'), message: t('app.taskInProgress') })
    return
  }
  const action = loadCloseAction()
  if (action === 'minimize') {
    await appWindow.minimize()
  } else if (action === 'tray') {
    await appWindow.hide()
  } else {
    // 退出程序前弹窗确认，防止误触
    try {
      await ElMessageBox.confirm(
        t('app.confirmExitMsg'),
        t('app.confirmExit'),
        {
          confirmButtonText: t('app.ok'),
          cancelButtonText: t('app.cancel'),
          type: 'warning',
        }
      )
      await appWindow.destroy()
    } catch (e) {
      if (e !== 'cancel' && e !== 'close') {
        ElNotification({ type: 'error', title: t('app.error'), message: t('app.closeFailed') })
      }
    }
  }
}

function goBack() {
  if (canGoBack.value) {
    isNavigating.value = true
    historyIndex.value--
    activePanel.value = historyStack.value[historyIndex.value]
  }
}

function goForward() {
  if (canGoForward.value) {
    isNavigating.value = true
    historyIndex.value++
    activePanel.value = historyStack.value[historyIndex.value]
  }
}

const NAV_KEYS: Record<string, string> = {
  'home': 'nav.home',
  'file-organize': 'nav.fileOrganize',
  'batch-rename': 'nav.batchRename',
  'duplicate-clean': 'nav.duplicateClean',
  'cleanup': 'nav.cleanup',
  'fix-date': 'nav.fixDate',
  'exif-clean': 'nav.exifClean',
  'write-gps': 'nav.writeGps',
  'ai-classify': 'nav.aiClassify',
  'log-view': 'nav.logView',
  'log-detail': 'nav.logDetail',
  'settings': 'nav.settings',
  'privacy-policy': 'nav.privacyPolicy',
  'license-agreement': 'nav.licenseAgreement',
  'about': 'nav.about',
}

watch(activePanel, (newPanel) => {
  if (isNavigating.value) {
    isNavigating.value = false
    return
  }
  // 记录页面导航
  if (newPanel !== 'home') {
    useLog().logEvent(t('app.navLog', { page: t(NAV_KEYS[newPanel] || newPanel) }))
  }
  historyStack.value = historyStack.value.slice(0, historyIndex.value + 1)
  historyStack.value.push(newPanel)
  if (historyStack.value.length > HISTORY_LIMIT) {
    historyStack.value.splice(0, historyStack.value.length - HISTORY_LIMIT)
  }
  historyIndex.value = historyStack.value.length - 1
})

function startResizeLeft() {
  resizing.value = 'left'
  document.addEventListener('mousemove', onResize)
  document.addEventListener('mouseup', stopResize)
}

function startResizeRight() {
  resizing.value = 'right'
  document.addEventListener('mousemove', onResize)
  document.addEventListener('mouseup', stopResize)
}

function onResize(e: MouseEvent) {
  if (resizing.value === 'left') {
    const newWidth = Math.max(160, Math.min(400, e.clientX))
    layout.value.leftBarWidth = newWidth
  } else if (resizing.value === 'right') {
    const newWidth = Math.max(240, Math.min(400, window.innerWidth - e.clientX))
    layout.value.rightBarWidth = newWidth
  }
}

function stopResize() {
  resizing.value = null
  document.removeEventListener('mousemove', onResize)
  document.removeEventListener('mouseup', stopResize)
}

async function checkGeocoderStatus() {
  try {
    const ready = await invoke<boolean>('is_geocoder_ready')
    if (ready) {
      geoStatus.value = 'ready'
    } else {
      geoStatus.value = 'loading'
    }
  } catch {
    geoStatus.value = 'failed'
  }
}

let geoCheckInterval: number | null = null
let resizeUnlisten: UnlistenFn | null = null
let trayNavigateUnlisten: UnlistenFn | null = null
let trayOpenOutputUnlisten: UnlistenFn | null = null
let trayQuitBlockedUnlisten: UnlistenFn | null = null

onMounted(async () => {
  isMaximized.value = await appWindow.isMaximized()
  resizeUnlisten = await appWindow.onResized(async () => { isMaximized.value = await appWindow.isMaximized() })

  await checkGeocoderStatus()
  if (geoStatus.value === 'loading') {
    geoCheckInterval = window.setInterval(async () => {
      await checkGeocoderStatus()
      if (geoStatus.value !== 'loading' && geoCheckInterval) {
        clearInterval(geoCheckInterval)
        geoCheckInterval = null
      }
    }, 1000)
  }

  // 托盘菜单事件：导航到设置/关于页面
  trayNavigateUnlisten = await listen<string>('tray-navigate', (event) => {
    const panel = event.payload as FunctionPanel
    if (panel === 'settings' || panel === 'about') {
      activePanel.value = panel
    }
  })

  // 托盘菜单事件：打开输出目录
  trayOpenOutputUnlisten = await listen('tray-open-output', () => {
    if (outputDir.value) {
      invoke('open_in_explorer', { path: outputDir.value })
    }
  })

  // 托盘"退出程序"在任务运行中被后端拦截，前端给出提示
  trayQuitBlockedUnlisten = await listen(TASK_EVENT.trayQuitBlocked, () => {
    ElNotification({ type: 'warning', title: t('app.warning'), message: t('app.trayQuitBlocked') })
  })

  // 记录应用启动
  useLog().logEvent('应用启动')

  // 每次启动询问使用体验；给出 3 星及以上后 7 天内不再弹出
  let lastRated = 0
  try { lastRated = Number(localStorage.getItem(RATE_SNOOZE_KEY)) || 0 } catch { /* localStorage 不可用时忽略 */ }
  rateDialogVisible.value = Date.now() - lastRated >= RATE_SNOOZE_MS
})

onUnmounted(() => {
  document.removeEventListener('mousemove', onResize)
  document.removeEventListener('mouseup', stopResize)
  if (geoCheckInterval) {
    clearInterval(geoCheckInterval)
  }
  resizeUnlisten?.()
  trayNavigateUnlisten?.()
  trayOpenOutputUnlisten?.()
  trayQuitBlockedUnlisten?.()
})
</script>

<style scoped>
.app {
  height: 100vh;
  background: var(--bg);
}

.main {
  height: 100%;
  display: flex;
}

.sidebar-left {
  background: var(--panel);
  display: flex;
  flex-direction: column;
  position: relative;
}

.sidebar-left .sidebar-header {
  justify-content: center;
}

.sidebar-header {
  height: 40px;
  display: flex;
  align-items: center;
  padding: 0 4px;
}

.sidebar-title {
  display: flex;
  align-items: center;
  font-size: 24px;
  gap: 2px;
  font-weight: 620;
  padding-top: 12px;
  padding-bottom: 0px;
}

.title-leaf {
  color: var(--primary);
}

.title-tidy {
  color: var(--text);
}

.resize-handle-left {
  position: absolute;
  right: 0;
  top: 0;
  bottom: 0;
  width: 1px;
  cursor: ew-resize;
  background: transparent;
}

.resize-handle-left:hover {
  background: var(--primary);
}

.workspace {
  flex: 1;
  background: var(--bg);
  display: flex;
  flex-direction: column;
  min-width: 400px;
}

.workspace-header {
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 16px;
}

.toolbar-left {
  display: flex;
  align-items: center;
  gap: 8px;
}

.toolbar-right {
  display: flex;
  align-items: center;
}

.geo-status {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 10px;
  border-radius: 4px;
  font-size: 12px;
}

.geo-status.loading {
  background: rgba(var(--primary-rgb), 0.1);
  color: var(--primary);
}

.geo-status.ready {
  background: rgba(var(--success-rgb), 0.1);
  color: var(--success);
}

.geo-status.failed {
  background: rgba(var(--danger-rgb), 0.1);
  color: var(--danger);
}

.geo-status .loading-icon {
  font-size: 12px;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }

  to {
    transform: rotate(360deg);
  }
}

.btn-nav {
  width: 28px;
  height: 28px;
  min-width: 28px;
  padding: 0;
  border: none;
  background: transparent;
  color: var(--text-muted);
  border-radius: 6px;
}

.btn-nav:hover:not(:disabled) {
  background: var(--panel-2);
  color: var(--text);
}

.btn-nav:disabled {
  color: var(--border-strong);
  opacity: 0.5;
}

.btn-nav .el-icon {
  font-size: 14px;
}

.workspace-body {
  flex: 1;
  overflow: auto;
}

.sidebar-right {
  background: var(--panel);
  display: flex;
  flex-direction: column;
  position: relative;
}

.sidebar-right .sidebar-header {
  justify-content: flex-end;
}

.window-btns {
  display: flex;
  gap: 0px;
}

.btn-win {
  width: 32px;
  height: 32px;
  min-width: 32px;
  padding: 0;
  border: none;
  background: transparent;
  color: var(--text-muted);
  border-radius: 6px;
}

.btn-win:hover {
  background: var(--panel-2);
  color: var(--text);
}

.btn-win.close:hover {
  background: var(--danger);
  color: var(--white);
}

.btn-win .el-icon {
  font-size: 14px;
}

.resize-handle-right {
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 1px;
  cursor: ew-resize;
  background: transparent;
}

.resize-handle-right:hover {
  background: var(--primary);
}

.rate-body {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 4px 0 8px;
}

.rate-question {
  margin: 0;
  font-size: 14px;
  color: var(--text);
}
</style>
