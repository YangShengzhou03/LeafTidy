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
          <component :is="currentPage" />
        </div>
      </div>

      <div ref="rightSidebarRef" class="sidebar-right" :style="{ width: layout.rightBarWidth + 'px' }">
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
import { ElMessageBox, ElNotification } from 'element-plus'
import zhCn from 'element-plus/dist/locale/zh-cn.mjs'
import zhTw from 'element-plus/dist/locale/zh-tw.mjs'
import en from 'element-plus/dist/locale/en.mjs'
import { t, locale } from '@/i18n'

// Element Plus 内部文案跟随当前语言实时切换
const epLocales = { 'zh-CN': zhCn, 'zh-TW': zhTw, 'en': en }
const epLocale = computed(() => epLocales[locale.value] ?? zhCn)
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

import type { FunctionPanel } from '@/types'

const { layout, activePanel, workDirs, outputDir, dirStats, currentLogId, isProcessing } = useLayout()

provide('layout', layout)
provide('activePanel', activePanel)
provide('workDirs', workDirs)
provide('outputDir', outputDir)
provide('dirStats', dirStats)
provide('currentLogId', currentLogId)
provide('isProcessing', isProcessing)

const isMaximized = ref(false)
const resizing = ref<'left' | 'right' | null>(null)
const historyStack = ref<FunctionPanel[]>(['home'])
const historyIndex = ref(0)
const isNavigating = ref(false)
const geoStatus = ref<'loading' | 'ready' | 'failed' | 'unknown'>('loading')

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
        console.error('关闭应用失败:', e)
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

watch(activePanel, (newPanel) => {
  if (isNavigating.value) {
    isNavigating.value = false
    return
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

onMounted(async () => {
  isMaximized.value = await appWindow.isMaximized()
  appWindow.onResized(async () => { isMaximized.value = await appWindow.isMaximized() })

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
})

onUnmounted(() => {
  document.removeEventListener('mousemove', onResize)
  document.removeEventListener('mouseup', stopResize)
  if (geoCheckInterval) {
    clearInterval(geoCheckInterval)
  }
})
</script>

<style>
/* ===== 主题变量（全站唯一样式来源，改主题只改这里）===== */
:root {
  --bg: #18191C;
  --panel: #1F2023;
  --panel-2: #2A2B30;
  --panel-2-hover: #353639;
  --border: #3A3B40;
  --border-strong: #5A5F6A;
  --text: #E0E6ED;
  --text-secondary: #C8D0DC;
  --text-muted: #8A94A6;
  --primary: #3A86FF;
  --primary-hover: #5A9FFF;
  --success: #52C41A;
  --warning: #F59E0B;
  --danger: #E81123;
  --danger-hover: #FF4D4F;
  --white: #FFFFFF;
  --bg-rgb: 24, 25, 28;
  --border-strong-rgb: 90, 94, 106;
  --primary-rgb: 58, 134, 255;
  --success-rgb: 82, 196, 26;
  --warning-rgb: 245, 158, 11;
  --danger-rgb: 232, 17, 35;
  --white-rgb: 255, 255, 255;
}

:root[data-theme='light'] {
  --bg: #F5F6F8;
  --panel: #FFFFFF;
  --panel-2: #F0F1F4;
  --panel-2-hover: #E4E6EB;
  --border: #D8DCE3;
  --border-strong: #B4BAC5;
  --text: #1F2329;
  --text-secondary: #3D4350;
  --text-muted: #8A94A6;
  --primary: #3A86FF;
  --primary-hover: #2C6FE0;
  --success: #52C41A;
  --warning: #F59E0B;
  --danger: #D93026;
  --danger-hover: #B72219;
  --bg-rgb: 245, 246, 248;
  --border-strong-rgb: 180, 186, 197;
  --primary-rgb: 58, 134, 255;
  --success-rgb: 82, 196, 26;
  --warning-rgb: 245, 158, 11;
  --danger-rgb: 217, 48, 38;
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html,
body,
#app {
  height: 100%;
  overflow: hidden;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: var(--bg);
  color: var(--text);
  -webkit-user-select: none;
  user-select: none;
}

/* 保留输入框/文本域的可选中性 */
input,
textarea,
[contenteditable="true"] {
  -webkit-user-select: text;
  user-select: text;
}

::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: var(--border);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: var(--border-strong);
}

::-webkit-scrollbar-corner {
  background: transparent;
}

* {
  scrollbar-width: thin;
  scrollbar-color: var(--border) transparent;
}

.el-input__wrapper {
  background: var(--panel-2) !important;
  border-color: var(--border) !important;
  box-shadow: none !important;
}

.el-input__inner {
  color: var(--text) !important;
}

.el-input__inner::placeholder {
  color: var(--text-muted) !important;
}

.el-input:hover .el-input__wrapper {
  border-color: var(--border-strong) !important;
}

.el-input.is-focus .el-input__wrapper {
  border-color: var(--primary) !important;
}

.el-input-number .el-input__wrapper {
  background: var(--panel-2) !important;
  border-color: var(--border) !important;
}

.el-input-number__decrease,
.el-input-number__increase {
  background: var(--panel-2) !important;
  border-color: var(--border) !important;
  color: var(--text-secondary) !important;
}

.el-input-number__decrease:hover,
.el-input-number__increase:hover {
  background: var(--border) !important;
  color: var(--text) !important;
}

.el-input-number__decrease.is-disabled,
.el-input-number__increase.is-disabled {
  color: var(--border-strong) !important;
  background: var(--panel-2) !important;
}

.el-radio-button__inner {
  background: var(--panel) !important;
  border-color: var(--border) !important;
  color: var(--text-secondary) !important;
}

.el-radio-button__original-radio:checked+.el-radio-button__inner {
  background: var(--primary) !important;
  border-color: var(--primary) !important;
  color: var(--white) !important;
}

.el-radio-button:hover .el-radio-button__inner {
  background: var(--panel-2) !important;
}

.el-radio {
  color: var(--text-secondary) !important;
}

.el-radio__input.is-checked .el-radio__inner {
  background: var(--primary) !important;
  border-color: var(--primary) !important;
}

.el-radio__input.is-checked+.el-radio__label {
  color: var(--primary) !important;
}

.el-radio__inner {
  background: var(--panel) !important;
  border-color: var(--border) !important;
}

.el-radio:hover .el-radio__inner {
  border-color: var(--border-strong) !important;
}

.el-checkbox__inner {
  background: var(--panel) !important;
  border-color: var(--border) !important;
}

.el-checkbox__input.is-checked .el-checkbox__inner {
  background: var(--primary) !important;
  border-color: var(--primary) !important;
}

.el-checkbox__input.is-checked+.el-checkbox__label {
  color: var(--text) !important;
}

.el-checkbox__label {
  color: var(--text-secondary) !important;
  font-size: 13px;
  font-weight: 500;
}

.el-checkbox:hover .el-checkbox__inner {
  border-color: var(--border-strong) !important;
}

.el-switch__core {
  background: var(--border) !important;
  border-color: var(--border) !important;
}

.el-switch.is-checked .el-switch__core {
  background: var(--panel-2) !important;
  border-color: var(--panel-2) !important;
}

.el-switch__action {
  background: var(--bg) !important;
}

.el-switch__action .el-icon,
.el-switch__action .el-icon svg {
  color: var(--text) !important;
}

.el-switch__inner {
  color: var(--text-secondary) !important;
}

:root[data-theme='light'] .el-switch.is-checked .el-switch__core {
  background: var(--primary) !important;
  border-color: var(--primary) !important;
}

:root[data-theme='light'] .el-switch__action {
  background: var(--white) !important;
}

.el-slider__runway {
  background: var(--border) !important;
}

.el-slider__bar {
  background: var(--primary) !important;
}

.el-slider__button {
  border-color: var(--primary) !important;
  background: var(--primary) !important;
}

.el-slider__marks-text {
  color: var(--text-muted) !important;
}

.el-slider__stop {
  background: var(--border-strong) !important;
}

.el-dropdown-menu {
  background: var(--panel-2) !important;
  border-color: var(--border) !important;
}

.el-dropdown-menu__item {
  color: var(--text-secondary) !important;
}

.el-dropdown-menu__item:hover {
  background: var(--border) !important;
  color: var(--text) !important;
}

.el-dropdown-menu__item.is-disabled {
  color: var(--border-strong) !important;
}

.el-button {
  background: var(--panel-2) !important;
  border-color: var(--border) !important;
  color: var(--text-secondary) !important;
}

.el-button:hover {
  background: var(--border) !important;
  border-color: var(--border-strong) !important;
  color: var(--text) !important;
}

.el-button--primary {
  background: var(--primary) !important;
  border-color: var(--primary) !important;
  color: var(--white) !important;
}

.el-button--primary:hover {
  background: var(--primary-hover) !important;
  border-color: var(--primary-hover) !important;
  color: var(--white) !important;
}

.el-button--danger {
  background: rgba(var(--danger-rgb), 0.15) !important;
  border-color: var(--danger) !important;
  color: var(--danger) !important;
}

.el-button--danger:hover {
  background: rgba(var(--danger-rgb), 0.25) !important;
  border-color: var(--danger-hover) !important;
  color: var(--danger-hover) !important;
}

.el-button--text {
  color: var(--text-secondary) !important;
  background: transparent !important;
  border-color: transparent !important;
}

.el-button--text:hover {
  color: var(--text) !important;
  background: rgba(var(--white-rgb), 0.08) !important;
}

.el-button--text.el-button--danger {
  color: var(--danger) !important;
}

.el-button--text.el-button--danger:hover {
  color: var(--danger-hover) !important;
  background: rgba(var(--danger-rgb), 0.15) !important;
}

.el-button--plain {
  background: transparent !important;
  border-color: var(--border) !important;
  color: var(--text-secondary) !important;
}

.el-button--plain:hover {
  background: rgba(var(--white-rgb), 0.08) !important;
  border-color: var(--border-strong) !important;
  color: var(--text) !important;
}

.el-button--plain.el-button--primary {
  border-color: var(--primary) !important;
  color: var(--primary) !important;
}

.el-button--plain.el-button--primary:hover {
  background: rgba(var(--primary-rgb), 0.15) !important;
}

.el-button--plain.el-button--danger {
  border-color: var(--danger) !important;
  color: var(--danger) !important;
}

.el-button--plain.el-button--danger:hover {
  background: rgba(var(--danger-rgb), 0.15) !important;
}

.el-message-box {
  background: var(--panel) !important;
  border-color: var(--border) !important;
}

.el-message-box__header {
  border-bottom-color: var(--panel-2) !important;
}

.el-message-box__title {
  color: var(--text) !important;
}

.el-message-box__content {
  color: var(--text-secondary) !important;
}

.el-message-box__message {
  color: var(--text-secondary) !important;
}

.el-message-box__btns {
  border-top-color: var(--panel-2) !important;
}

.el-message {
  background: var(--panel) !important;
  border-color: var(--border) !important;
}

.el-message__content {
  color: var(--text-secondary) !important;
}

.el-message--success .el-message__icon {
  color: var(--success) !important;
}

.el-message--warning .el-message__icon {
  color: var(--warning) !important;
}

.el-message--error .el-message__icon {
  color: var(--danger) !important;
}

.el-message--info .el-message__icon {
  color: var(--primary) !important;
}

.el-date-editor .el-input__wrapper {
  background: var(--panel-2) !important;
  border-color: var(--border) !important;
}

.el-picker-panel {
  background: var(--panel) !important;
  border-color: var(--border) !important;
}

.el-picker-panel__footer {
  background: var(--panel) !important;
  border-top-color: var(--border) !important;
}

.el-picker-panel__footer .el-button.is-text {
  color: var(--text-secondary) !important;
}

.el-picker-panel__footer .el-button.is-plain {
  background: var(--panel-2) !important;
  border-color: var(--border) !important;
  color: var(--text) !important;
}

.el-date-table th {
  color: var(--text-muted) !important;
}

.el-date-table td {
  color: var(--text-secondary) !important;
}

.el-date-table td.today {
  color: var(--primary) !important;
}

.el-date-table td.available:hover {
  background: var(--panel-2) !important;
}

.el-date-table td.current:not(.disabled) {
  background: var(--primary) !important;
  color: var(--white) !important;
}

.el-date-table td.in-range {
  background: rgba(var(--primary-rgb), 0.15) !important;
}

.el-date-picker__header-label {
  color: var(--text-secondary) !important;
}

.el-date-picker__header-label:hover {
  color: var(--primary) !important;
}

.el-picker-panel__icon-btn {
  color: var(--text-muted) !important;
}

.el-picker-panel__icon-btn:hover {
  color: var(--text) !important;
}

.el-table {
  --el-table-bg-color: var(--panel) !important;
  --el-table-tr-bg-color: var(--panel) !important;
  --el-table-header-bg-color: var(--panel-2) !important;
  --el-table-row-hover-bg-color: var(--panel-2) !important;
  --el-table-text-color: var(--text) !important;
  --el-table-header-text-color: var(--text-secondary) !important;
  --el-table-border-color: var(--border) !important;
  --el-table-border: 1px solid var(--border) !important;
}

.el-table__body tr.current-row>td.el-table__cell {
  background: rgba(var(--primary-rgb), 0.08) !important;
}

.el-table__body tr:hover>td.el-table__cell {
  background: var(--panel-2) !important;
}

.el-tabs__header {
  border-bottom-color: var(--border) !important;
}

.el-tabs__item {
  color: var(--text-muted) !important;
}

.el-tabs__item.is-active {
  color: var(--text) !important;
}

.el-tabs__item:hover {
  color: var(--text) !important;
}

.el-tabs__active-bar {
  background: var(--primary) !important;
}

.el-tag--primary {
  background: rgba(var(--primary-rgb), 0.15) !important;
  border-color: rgba(var(--primary-rgb), 0.3) !important;
  color: var(--primary) !important;
}

.el-tag--primary.el-tag--dark {
  background: var(--primary) !important;
  border-color: var(--primary) !important;
  color: var(--white) !important;
}

.el-tag--info {
  background: rgba(var(--border-strong-rgb), 0.15) !important;
  border-color: rgba(var(--border-strong-rgb), 0.3) !important;
  color: var(--text-muted) !important;
}

.el-tag--info.el-tag--dark {
  background: var(--border-strong) !important;
  border-color: var(--border-strong) !important;
  color: var(--text) !important;
}

.el-tag {
  background: rgba(var(--border-strong-rgb), 0.15) !important;
  border-color: rgba(var(--border-strong-rgb), 0.3) !important;
  color: var(--text-secondary) !important;
}

.el-tag.el-tag--dark {
  background: var(--primary) !important;
  border-color: var(--primary) !important;
  color: var(--white) !important;
}

.el-select .el-select__wrapper {
  background: var(--panel-2) !important;
  border-color: var(--border) !important;
  box-shadow: none !important;
}

.el-select .el-select__wrapper:hover {
  border-color: var(--border-strong) !important;
}

.el-select .el-select__wrapper.is-focused {
  border-color: var(--primary) !important;
}

.el-select .el-select__placeholder {
  color: var(--text-muted) !important;
}

.el-select .el-select__selected-item {
  color: var(--text) !important;
}

.el-select .el-select__input {
  color: var(--text) !important;
}

.el-select .el-select__caret {
  color: var(--text-muted) !important;
}

.el-select .el-select__caret:hover {
  color: var(--text) !important;
}

.el-select-dropdown {
  background: var(--panel-2) !important;
  border-color: var(--border) !important;
}

.el-select-dropdown__item {
  color: var(--text-secondary) !important;
}

.el-select-dropdown__item:hover {
  background: var(--border) !important;
}

.el-select-dropdown__item.is-selected {
  background: rgba(var(--primary-rgb), 0.15) !important;
  color: var(--primary) !important;
}

.el-select-dropdown__item.is-hovering {
  background: var(--border) !important;
}

.el-popper.is-light {
  background: var(--panel-2) !important;
  border-color: var(--border) !important;
}

.el-tooltip__popper {
  background: var(--panel-2) !important;
  border-color: var(--border) !important;
}

.el-tooltip__popper .el-tooltip__arrow::before {
  border-color: var(--border) !important;
}

.el-tooltip__popper.is-dark {
  background: var(--panel) !important;
}

.el-popover {
  background: var(--panel) !important;
  border-color: var(--border) !important;
}

.el-popover__title {
  color: var(--text) !important;
  border-bottom-color: var(--panel-2) !important;
}

.el-popover__content {
  color: var(--text-secondary) !important;
}

.el-dialog {
  background: var(--panel) !important;
  border-color: var(--border) !important;
}

.el-dialog__header {
  border-bottom-color: var(--panel-2) !important;
}

.el-dialog__title {
  color: var(--text) !important;
}

.el-dialog__body {
  color: var(--text-secondary) !important;
}

.el-dialog__footer {
  border-top-color: var(--panel-2) !important;
}

.el-notification {
  background: var(--panel) !important;
  border-color: var(--border) !important;
}

.el-notification__title {
  color: var(--text) !important;
}

.el-notification__content {
  color: var(--text-secondary) !important;
}

.el-loading-mask {
  background: rgba(var(--bg-rgb), 0.8) !important;
}

.el-loading-spinner .el-loading-text {
  color: var(--text-secondary) !important;
}

.el-progress__text {
  color: var(--text-secondary) !important;
}

.el-progress-bar__outer {
  background: var(--panel-2) !important;
}

.el-progress-bar__inner {
  background: var(--primary) !important;
}

.el-alert {
  background: var(--panel) !important;
  border-color: var(--border) !important;
}

.el-alert__title {
  color: var(--text) !important;
}

.el-alert__description {
  color: var(--text-secondary) !important;
}

.el-alert--success {
  border-color: rgba(var(--success-rgb), 0.3) !important;
  background: rgba(var(--success-rgb), 0.1) !important;
}

.el-alert--warning {
  border-color: rgba(var(--warning-rgb), 0.3) !important;
  background: rgba(var(--warning-rgb), 0.1) !important;
}

.el-alert--error {
  border-color: rgba(var(--danger-rgb), 0.3) !important;
  background: rgba(var(--danger-rgb), 0.1) !important;
}

.el-alert--info {
  border-color: rgba(var(--primary-rgb), 0.3) !important;
  background: rgba(var(--primary-rgb), 0.1) !important;
}

.el-tour {
  --el-tour-title-text-color: var(--text) !important;
  --el-tour-text-color: var(--text-secondary) !important;
  --el-tour-bg-color: var(--panel) !important;
  --el-tour-border-color: var(--border) !important;
}

.el-tour__header {
  border-bottom-color: var(--panel-2) !important;
}

.el-tour__content {
  background: var(--panel) !important;
  border-color: var(--border) !important;
}

.el-tour__close {
  color: var(--text-muted) !important;
}

.el-tour__close:hover {
  color: var(--text) !important;
}

.el-tour__indicators {
  color: var(--text-muted) !important;
}

.el-tour__indicators-item {
  background: var(--border) !important;
}

.el-tour__indicators-item.is-active {
  background: var(--primary) !important;
}
</style>

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

.geo-status .check-icon,
.geo-status .error-icon {
  font-size: 12px;
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
  width: 28px !important;
  height: 28px !important;
  min-width: 28px !important;
  padding: 0 !important;
  border: none !important;
  background: transparent !important;
  color: var(--text-muted) !important;
  border-radius: 6px !important;
}

.btn-nav:hover:not(:disabled) {
  background: var(--panel-2) !important;
  color: var(--text) !important;
}

.btn-nav:disabled {
  color: var(--border-strong) !important;
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
  width: 32px !important;
  height: 32px !important;
  min-width: 32px !important;
  padding: 0 !important;
  border: none !important;
  background: transparent !important;
  color: var(--text-muted) !important;
  border-radius: 6px !important;
}

.btn-win:hover {
  background: var(--panel-2) !important;
  color: var(--text) !important;
}

.btn-win.close:hover {
  background: var(--danger) !important;
  color: var(--white) !important;
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