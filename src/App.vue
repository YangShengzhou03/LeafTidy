<template>
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
            <el-button class="btn-nav" :disabled="!canGoBack" @mousedown.stop @click.stop="goBack" title="后退">
              <el-icon>
                <ArrowLeft />
              </el-icon>
            </el-button>
            <el-button class="btn-nav" :disabled="!canGoForward" @mousedown.stop @click.stop="goForward" title="前进">
              <el-icon>
                <ArrowRight />
              </el-icon>
            </el-button>
          </div>
          <div class="toolbar-right">
            <div v-if="geoStatus === 'loading'" class="geo-status loading">
              <el-icon class="loading-icon"><svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M512 896c212.864 0 384-171.136 384-384s-171.136-384-384-384-384 171.136-384 384 171.136 384 384 384z m0-704c177.664 0 320 142.336 320 320s-142.336 320-320 320-320-142.336-320-320 142.336-320 320-320z" opacity=".3"/><path fill="currentColor" d="M512 640a128 128 0 1 0-128-128 128 128 0 0 0 128 128zm0-192a64 64 0 1 1-64 64 64 64 0 0 1 64-64z"/></svg></el-icon>
              <span>正在加载地理数据</span>
            </div>
            <div v-else-if="geoStatus === 'ready'" class="geo-status ready">
              <el-icon class="check-icon"><svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M819.2 332.8L486.4 665.6 204.8 384c-19.2-19.2-48-19.2-67.2 0l-64 64c-19.2 19.2-19.2 48 0 67.2l384 384c19.2 19.2 48 19.2 67.2 0l512-512c19.2-19.2 19.2-48 0-67.2l-64-64c-19.2-19.2-48-19.2-67.2 0z"/></svg></el-icon>
              <span>已加载地理数据</span>
            </div>
            <div v-else-if="geoStatus === 'failed'" class="geo-status failed">
              <el-icon class="error-icon"><svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M512 960C268.8 960 64 755.2 64 512S268.8 64 512 64s448 204.8 448 448-204.8 448-448 448zm0-832C294.4 128 128 294.4 128 512s166.4 384 384 384 384-166.4 384-384S729.6 128 512 128z"/><path fill="currentColor" d="M608 384l192 192-192 192-192-192 192-192z"/></svg></el-icon>
              <span>地理数据加载失败</span>
            </div>
          </div>
        </div>
        <div class="workspace-body">
          <component :is="currentPage" />
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
</template>

<script setup lang="ts">
import { ref, provide, computed, onMounted, onUnmounted, watch } from 'vue'
import { Close, Minus, ArrowLeft, ArrowRight } from '@element-plus/icons-vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/core'
import { ElMessageBox, ElMessage } from 'element-plus'
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
}

const currentPage = computed(() => pageMap[activePanel.value] || HomePage)

const appWindow = getCurrentWindow()

async function startDragging() { await appWindow.startDragging() }
async function minimizeWindow() { await appWindow.minimize() }
async function maximizeWindow() {
  if (isMaximized.value) { await appWindow.unmaximize(); isMaximized.value = false }
  else { await appWindow.maximize(); isMaximized.value = true }
}
async function closeWindow() {
  if (isProcessing.value) {
    ElMessage.warning('有任务正在处理中，请先结束任务再关闭')
    return
  }
  try {
    await ElMessageBox.confirm(
      '确定要退出应用吗？',
      '确认退出',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      }
    )
    await appWindow.close()
  } catch (e) {
    if (e !== 'cancel' && e !== 'close') {
      console.error('关闭应用失败:', e)
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
  background: #18191C;
  color: #E0E6ED;
}

::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: #3A3B40;
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: #5A5F6A;
}

::-webkit-scrollbar-corner {
  background: transparent;
}

* {
  scrollbar-width: thin;
  scrollbar-color: #3A3B40 transparent;
}

.el-input__wrapper {
  background: #2A2B30 !important;
  border-color: #3A3B40 !important;
  box-shadow: none !important;
}

.el-input__inner {
  color: #E0E6ED !important;
}

.el-input__inner::placeholder {
  color: #8A94A6 !important;
}

.el-input:hover .el-input__wrapper {
  border-color: #5A5F6A !important;
}

.el-input.is-focus .el-input__wrapper {
  border-color: #3A86FF !important;
}

.el-input-number .el-input__wrapper {
  background: #2A2B30 !important;
  border-color: #3A3B40 !important;
}

.el-input-number__decrease,
.el-input-number__increase {
  background: #2A2B30 !important;
  border-color: #3A3B40 !important;
  color: #C8D0DC !important;
}

.el-input-number__decrease:hover,
.el-input-number__increase:hover {
  background: #3A3B40 !important;
  color: #E0E6ED !important;
}

.el-input-number__decrease.is-disabled,
.el-input-number__increase.is-disabled {
  color: #5A5F6A !important;
  background: #2A2B30 !important;
}

.el-radio-button__inner {
  background: #1F2023 !important;
  border-color: #3A3B40 !important;
  color: #C8D0DC !important;
}

.el-radio-button__original-radio:checked+.el-radio-button__inner {
  background: #3A86FF !important;
  border-color: #3A86FF !important;
  color: #FFFFFF !important;
}

.el-radio-button:hover .el-radio-button__inner {
  background: #2A2B30 !important;
}

.el-radio {
  color: #C8D0DC !important;
}

.el-radio__input.is-checked .el-radio__inner {
  background: #3A86FF !important;
  border-color: #3A86FF !important;
}

.el-radio__input.is-checked+.el-radio__label {
  color: #3A86FF !important;
}

.el-radio__inner {
  background: #1F2023 !important;
  border-color: #3A3B40 !important;
}

.el-radio:hover .el-radio__inner {
  border-color: #5A5F6A !important;
}

.el-checkbox__inner {
  background: #1F2023 !important;
  border-color: #3A3B40 !important;
}

.el-checkbox__input.is-checked .el-checkbox__inner {
  background: #3A86FF !important;
  border-color: #3A86FF !important;
}

.el-checkbox__input.is-checked+.el-checkbox__label {
  color: #E0E6ED !important;
}

.el-checkbox__label {
  color: #C8D0DC !important;
}

.el-checkbox:hover .el-checkbox__inner {
  border-color: #5A5F6A !important;
}

.el-switch__core {
  background: #3A3B40 !important;
  border-color: #3A3B40 !important;
}

.el-switch.is-checked .el-switch__core {
  background: #3A86FF !important;
  border-color: #3A86FF !important;
}

.el-switch__inner {
  color: #C8D0DC !important;
}

.el-slider__runway {
  background: #3A3B40 !important;
}

.el-slider__bar {
  background: #3A86FF !important;
}

.el-slider__button {
  border-color: #3A86FF !important;
  background: #3A86FF !important;
}

.el-slider__marks-text {
  color: #8A94A6 !important;
}

.el-slider__stop {
  background: #5A5F6A !important;
}

.el-dropdown-menu {
  background: #2A2B30 !important;
  border-color: #3A3B40 !important;
}

.el-dropdown-menu__item {
  color: #C8D0DC !important;
}

.el-dropdown-menu__item:hover {
  background: #3A3B40 !important;
  color: #E0E6ED !important;
}

.el-dropdown-menu__item.is-disabled {
  color: #5A5F6A !important;
}

.el-button {
  background: #2A2B30 !important;
  border-color: #3A3B40 !important;
  color: #C8D0DC !important;
}

.el-button:hover {
  background: #3A3B40 !important;
  border-color: #5A5F6A !important;
  color: #E0E6ED !important;
}

.el-button--primary {
  background: #3A86FF !important;
  border-color: #3A86FF !important;
  color: #FFFFFF !important;
}

.el-button--primary:hover {
  background: #5A9FFF !important;
  border-color: #5A9FFF !important;
}

.el-button--danger {
  background: rgba(232, 17, 35, 0.15) !important;
  border-color: #E81123 !important;
  color: #E81123 !important;
}

.el-button--danger:hover {
  background: rgba(232, 17, 35, 0.25) !important;
  border-color: #FF4D4F !important;
  color: #FF4D4F !important;
}

.el-button--text {
  color: #C8D0DC !important;
  background: transparent !important;
  border-color: transparent !important;
}

.el-button--text:hover {
  color: #E0E6ED !important;
  background: rgba(255, 255, 255, 0.08) !important;
}

.el-button--text.el-button--danger {
  color: #E81123 !important;
}

.el-button--text.el-button--danger:hover {
  color: #FF4D4F !important;
  background: rgba(232, 17, 35, 0.15) !important;
}

.el-button--plain {
  background: transparent !important;
  border-color: #3A3B40 !important;
  color: #C8D0DC !important;
}

.el-button--plain:hover {
  background: rgba(255, 255, 255, 0.08) !important;
  border-color: #5A5F6A !important;
  color: #E0E6ED !important;
}

.el-button--plain.el-button--primary {
  border-color: #3A86FF !important;
  color: #3A86FF !important;
}

.el-button--plain.el-button--primary:hover {
  background: rgba(58, 134, 255, 0.15) !important;
}

.el-button--plain.el-button--danger {
  border-color: #E81123 !important;
  color: #E81123 !important;
}

.el-button--plain.el-button--danger:hover {
  background: rgba(232, 17, 35, 0.15) !important;
}

.el-message-box {
  background: #1F2023 !important;
  border-color: #3A3B40 !important;
}

.el-message-box__header {
  border-bottom-color: #2A2B30 !important;
}

.el-message-box__title {
  color: #E0E6ED !important;
}

.el-message-box__content {
  color: #C8D0DC !important;
}

.el-message-box__message {
  color: #C8D0DC !important;
}

.el-message-box__btns {
  border-top-color: #2A2B30 !important;
}

.el-message {
  background: #1F2023 !important;
  border-color: #3A3B40 !important;
}

.el-message__content {
  color: #C8D0DC !important;
}

.el-message--success .el-message__icon {
  color: #52C41A !important;
}

.el-message--warning .el-message__icon {
  color: #F59E0B !important;
}

.el-message--error .el-message__icon {
  color: #E81123 !important;
}

.el-message--info .el-message__icon {
  color: #3A86FF !important;
}

.el-date-editor .el-input__wrapper {
  background: #2A2B30 !important;
  border-color: #3A3B40 !important;
}

.el-picker-panel {
  background: #1F2023 !important;
  border-color: #3A3B40 !important;
}

.el-date-table th {
  color: #8A94A6 !important;
}

.el-date-table td {
  color: #C8D0DC !important;
}

.el-date-table td.today {
  color: #3A86FF !important;
}

.el-date-table td.available:hover {
  background: #2A2B30 !important;
}

.el-date-table td.current:not(.disabled) {
  background: #3A86FF !important;
  color: #FFFFFF !important;
}

.el-date-table td.in-range {
  background: rgba(58, 134, 255, 0.15) !important;
}

.el-date-picker__header-label {
  color: #C8D0DC !important;
}

.el-date-picker__header-label:hover {
  color: #3A86FF !important;
}

.el-picker-panel__icon-btn {
  color: #8A94A6 !important;
}

.el-picker-panel__icon-btn:hover {
  color: #E0E6ED !important;
}

.el-table {
  --el-table-bg-color: #1F2023 !important;
  --el-table-tr-bg-color: #1F2023 !important;
  --el-table-header-bg-color: #2A2B30 !important;
  --el-table-row-hover-bg-color: #2A2B30 !important;
  --el-table-text-color: #E0E6ED !important;
  --el-table-header-text-color: #C8D0DC !important;
  --el-table-border-color: #3A3B40 !important;
  --el-table-border: 1px solid #3A3B40 !important;
}

.el-table__body tr.current-row>td.el-table__cell {
  background: rgba(58, 134, 255, 0.08) !important;
}

.el-table__body tr:hover>td.el-table__cell {
  background: #2A2B30 !important;
}

.el-tabs__header {
  border-bottom-color: #3A3B40 !important;
}

.el-tabs__item {
  color: #8A94A6 !important;
}

.el-tabs__item.is-active {
  color: #E0E6ED !important;
}

.el-tabs__item:hover {
  color: #E0E6ED !important;
}

.el-tabs__active-bar {
  background: #3A86FF !important;
}

.el-tag--primary {
  background: rgba(58, 134, 255, 0.15) !important;
  border-color: rgba(58, 134, 255, 0.3) !important;
  color: #3A86FF !important;
}

.el-tag--primary.el-tag--dark {
  background: #3A86FF !important;
  border-color: #3A86FF !important;
  color: #FFFFFF !important;
}

.el-tag--info {
  background: rgba(90, 94, 106, 0.15) !important;
  border-color: rgba(90, 94, 106, 0.3) !important;
  color: #8A94A6 !important;
}

.el-tag--info.el-tag--dark {
  background: #5A5F6A !important;
  border-color: #5A5F6A !important;
  color: #E0E6ED !important;
}

.el-tag {
  background: rgba(90, 94, 106, 0.15) !important;
  border-color: rgba(90, 94, 106, 0.3) !important;
  color: #C8D0DC !important;
}

.el-tag.el-tag--dark {
  background: #3A86FF !important;
  border-color: #3A86FF !important;
  color: #FFFFFF !important;
}

.el-select .el-select__wrapper {
  background: #2A2B30 !important;
  border-color: #3A3B40 !important;
  box-shadow: none !important;
}

.el-select .el-select__wrapper:hover {
  border-color: #5A5F6A !important;
}

.el-select .el-select__wrapper.is-focused {
  border-color: #3A86FF !important;
}

.el-select .el-select__placeholder {
  color: #8A94A6 !important;
}

.el-select .el-select__selected-item {
  color: #E0E6ED !important;
}

.el-select .el-select__input {
  color: #E0E6ED !important;
}

.el-select .el-select__caret {
  color: #8A94A6 !important;
}

.el-select .el-select__caret:hover {
  color: #E0E6ED !important;
}

.el-select-dropdown {
  background: #2A2B30 !important;
  border-color: #3A3B40 !important;
}

.el-select-dropdown__item {
  color: #C8D0DC !important;
}

.el-select-dropdown__item:hover {
  background: #3A3B40 !important;
}

.el-select-dropdown__item.is-selected {
  background: rgba(58, 134, 255, 0.15) !important;
  color: #3A86FF !important;
}

.el-select-dropdown__item.is-hovering {
  background: #3A3B40 !important;
}

.el-popper.is-light {
  background: #2A2B30 !important;
  border-color: #3A3B40 !important;
}

.el-tooltip__popper {
  background: #2A2B30 !important;
  border-color: #3A3B40 !important;
}

.el-tooltip__popper .el-tooltip__arrow::before {
  border-color: #3A3B40 !important;
}

.el-tooltip__popper.is-dark {
  background: #1F2023 !important;
}

.el-popover {
  background: #1F2023 !important;
  border-color: #3A3B40 !important;
}

.el-popover__title {
  color: #E0E6ED !important;
  border-bottom-color: #2A2B30 !important;
}

.el-popover__content {
  color: #C8D0DC !important;
}

.el-dialog {
  background: #1F2023 !important;
  border-color: #3A3B40 !important;
}

.el-dialog__header {
  border-bottom-color: #2A2B30 !important;
}

.el-dialog__title {
  color: #E0E6ED !important;
}

.el-dialog__body {
  color: #C8D0DC !important;
}

.el-dialog__footer {
  border-top-color: #2A2B30 !important;
}

.el-notification {
  background: #1F2023 !important;
  border-color: #3A3B40 !important;
}

.el-notification__title {
  color: #E0E6ED !important;
}

.el-notification__content {
  color: #C8D0DC !important;
}

.el-loading-mask {
  background: rgba(24, 25, 28, 0.8) !important;
}

.el-loading-spinner .el-loading-text {
  color: #C8D0DC !important;
}

.el-progress__text {
  color: #C8D0DC !important;
}

.el-progress-bar__outer {
  background: #2A2B30 !important;
}

.el-progress-bar__inner {
  background: #3A86FF !important;
}

.el-alert {
  background: #1F2023 !important;
  border-color: #3A3B40 !important;
}

.el-alert__title {
  color: #E0E6ED !important;
}

.el-alert__description {
  color: #C8D0DC !important;
}

.el-alert--success {
  border-color: rgba(82, 196, 26, 0.3) !important;
  background: rgba(82, 196, 26, 0.1) !important;
}

.el-alert--warning {
  border-color: rgba(245, 158, 11, 0.3) !important;
  background: rgba(245, 158, 11, 0.1) !important;
}

.el-alert--error {
  border-color: rgba(232, 17, 35, 0.3) !important;
  background: rgba(232, 17, 35, 0.1) !important;
}

.el-alert--info {
  border-color: rgba(58, 134, 255, 0.3) !important;
  background: rgba(58, 134, 255, 0.1) !important;
}
</style>

<style scoped>
.app {
  height: 100vh;
  background: #18191C;
}

.main {
  height: 100%;
  display: flex;
}

.sidebar-left {
  background: #1F2023;
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
  font-weight: 620;
  padding-top: 12px;
  padding-bottom: 0px;
}

.title-leaf {
  color: #3A86FF;
}

.title-tidy {
  color: #E0E6ED;
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
  background: #3A86FF;
}

.workspace {
  flex: 1;
  background: #18191C;
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
  background: rgba(58, 134, 255, 0.1);
  color: #3A86FF;
}

.geo-status.ready {
  background: rgba(82, 196, 26, 0.1);
  color: #52C41A;
}

.geo-status.failed {
  background: rgba(232, 17, 35, 0.1);
  color: #E81123;
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
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.btn-nav {
  width: 28px !important;
  height: 28px !important;
  min-width: 28px !important;
  padding: 0 !important;
  border: none !important;
  background: transparent !important;
  color: #8A94A6 !important;
  border-radius: 6px !important;
}

.btn-nav:hover:not(:disabled) {
  background: #2A2B30 !important;
  color: #E0E6ED !important;
}

.btn-nav:disabled {
  color: #5A5F6A !important;
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
  background: #1F2023;
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
  color: #8A94A6 !important;
  border-radius: 6px !important;
}

.btn-win:hover {
  background: #2A2B30 !important;
  color: #E0E6ED !important;
}

.btn-win.close:hover {
  background: #E81123 !important;
  color: #FFFFFF !important;
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
  background: #3A86FF;
}
</style>