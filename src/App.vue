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
  </el-config-provider>
</template>

<script setup lang="ts">
import { ref, provide, computed, onMounted, onUnmounted, watch } from 'vue'
import { Close, Minus, ArrowLeft, ArrowRight, SuccessFilled, CircleCloseFilled } from '@element-plus/icons-vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { ElMessageBox, ElNotification } from 'element-plus'
import zhCn from 'element-plus/dist/locale/zh-cn.mjs'
import zhTw from 'element-plus/dist/locale/zh-tw.mjs'
import en from 'element-plus/dist/locale/en.mjs'
import LeftSidebar from './components/LeftSidebar.vue'
import RightSidebar from './components/RightSidebar.vue'
import HomePage from './views/HomePage.vue'
import FileOrganizePage from './views/FileOrganizePage.vue'
import BatchRenamePage from './views/BatchRenamePage.vue'
import DuplicateCleanPage from './views/DuplicateCleanPage.vue'
import CleanupPage from './views/CleanupPage.vue'
import FixDatePage from './views/FixDatePage.vue'
import ExifCleanPage from './views/ExifCleanPage.vue'
import WriteGpsPage from './views/WriteGpsPage.vue'
import AiClassifyPage from './views/AiClassifyPage.vue'
import LogViewPage from './views/LogViewPage.vue'
import LogDetailPage from './views/LogDetailPage.vue'
import SettingsPage from './views/SettingsPage.vue'
import PrivacyPolicyPage from './views/PrivacyPolicyPage.vue'
import LicenseAgreementPage from './views/LicenseAgreementPage.vue'
import AboutPage from './views/AboutPage.vue'
import { useLayout } from '@/composables/useLayout'
import { useLog } from '@/composables/useLog'
import { t, locale } from '@/i18n'

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
const isNavigating = ref(false)
const geoStatus = ref<'loading' | 'ready' | 'failed'>('loading')

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
  try {
    const saved = JSON.parse(localStorage.getItem('leaf-tidy-settings') || '{}')
    if (saved.closeAction === 'minimize' || saved.closeAction === 'tray') return saved.closeAction
  } catch { /* 使用默认值 */ }
  return 'exit'
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
    useLog().logEvent(`${t('app.navPrefix')}: ${t(NAV_KEYS[newPanel] || newPanel)}`)
  }
  historyStack.value = historyStack.value.slice(0, historyIndex.value + 1)
  historyStack.value.push(newPanel)
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
    const newWidth = Math.max(200, Math.min(400, e.clientX))
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

  // 记录应用启动
  useLog().logEvent('应用启动')
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
</style>
