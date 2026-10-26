<template>
  <div class="settings-page">
    <div class="settings-content">
      <div class="settings-group">
        <div class="group-title">{{ t('settings.group.appearance') }}</div>
        <div class="setting-item">
          <span class="setting-label">{{ t('settings.themeMode') }}</span>
          <el-switch v-model="isDark" :active-action-icon="Moon" :inactive-action-icon="Sunny" />
        </div>
        <div class="setting-item">
          <span class="setting-label">{{ t('settings.language') }}</span>
          <el-select v-model="language" class="setting-input">
            <el-option :label="t('settings.lang.zhCN')" value="zh-CN" />
            <el-option :label="t('settings.lang.zhTW')" value="zh-TW" />
            <el-option :label="t('settings.lang.en')" value="en" />
          </el-select>
        </div>
        <div class="setting-item">
          <span class="setting-label">{{ t('settings.closeAction.label') }}</span>
          <el-select v-model="closeAction" class="setting-input">
            <el-option :label="t('settings.closeAction.exit')" value="exit" />
            <el-option :label="t('settings.closeAction.minimize')" value="minimize" />
            <el-option :label="t('settings.closeAction.tray')" value="tray" />
          </el-select>
        </div>
      </div>

      <div class="settings-group">
        <div class="group-title">{{ t('settings.group.about') }}</div>
        <div class="about-content">
          <h2>{{ t('settings.appName') }} <span class="version">v{{ APP_VERSION }}</span></h2>
          <p class="about-desc">{{ t('settings.aboutDesc') }}</p>
          <div class="feature-tags">
            <span class="feature-tag">{{ t('settings.tag.organize') }}</span>
            <span class="feature-tag">{{ t('settings.tag.rename') }}</span>
            <span class="feature-tag">{{ t('settings.tag.duplicates') }}</span>
            <span class="feature-tag">{{ t('settings.tag.exifClean') }}</span>
            <span class="feature-tag">{{ t('settings.tag.fixDate') }}</span>
            <span class="feature-tag">{{ t('settings.tag.gps') }}</span>
          </div>
          <div class="tech-stack">
            <span class="tech-badge">Tauri</span>
            <span class="tech-badge">Rust</span>
            <span class="tech-badge">Vue 3</span>
            <span class="tech-badge">TypeScript</span>
            <span class="tech-badge">Element Plus</span>
          </div>
          <div class="license-row">
            <a class="github-link" title="GitHub" @click="openGithub">
              <svg viewBox="0 0 16 16" width="16" height="16" aria-hidden="true">
                <path fill="currentColor"
                  d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27s1.36.09 2 .27c1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.01 8.01 0 0 0 16 8c0-4.42-3.58-8-8-8Z" />
              </svg>
              <span>GitHub</span>
            </a>
            <p class="license">MIT License</p>
          </div>
          <!-- 检查更新 -->
          <div class="update-section">
            <el-button
              :loading="checkingUpdate"
              :disabled="checkingUpdate"
              @click="handleCheckUpdate"
            >{{ checkingUpdate ? t('app.updating') : t('app.checkUpdate') }}</el-button>
            <span v-if="downloadPercent > 0 && downloadPercent < 100" class="update-progress">
              {{ t('app.updateDownloading', { percent: downloadPercent }) }}
            </span>
          </div>
        </div>
      </div>

      <div class="settings-group">
        <div class="group-title">{{ t('settings.group.ai') }}</div>

        <!-- 内置模型列表 -->
        <div class="ai-section">
          <div class="ai-section-label">{{ t('settings.ai.builtinModels') }}</div>
          <div class="model-list">
            <div v-for="model in builtinModels" :key="model.id" class="model-card">
              <div class="model-card-info">
                <span class="model-card-name">{{ model.name }}</span>
                <span class="model-card-meta">
                  {{ t('settings.ai.sizeInfo', { size: model.size_mb, acc: model.top1_accuracy }) }}
                </span>
              </div>
              <div class="model-card-actions">
                <el-button
                  v-if="defaultModelId !== model.id"
                  link
                  type="primary"
                  @click="setDefaultModel(model.id)">{{ t('settings.ai.setDefault') }}</el-button>
                <el-tag v-else type="primary">{{ t('settings.ai.default') }}</el-tag>
              </div>
            </div>
          </div>
        </div>

        <!-- 自定义模型列表 -->
        <div class="ai-section">
          <div class="ai-section-header">
            <span class="ai-section-label">{{ t('settings.ai.customModels') }}</span>
            <el-button type="primary" @click="showAddModelDialog = true">
              {{ t('settings.ai.addModel') }}
            </el-button>
          </div>
          <div class="model-list" v-if="customModels.length > 0">
            <div v-for="model in customModels" :key="model.id" class="model-card">
              <div class="model-card-info">
                <span class="model-card-name">{{ model.name }}</span>
                <span class="model-card-path">{{ model.id }}</span>
              </div>
              <div class="model-card-actions">
                <el-button
                  v-if="defaultModelId !== model.id"
                  link
                  type="primary"
                  @click="setDefaultModel(model.id)"
                >{{ t('settings.ai.setDefault') }}</el-button>
                <el-tag v-else type="primary">{{ t('settings.ai.default') }}</el-tag>
                <el-button link type="danger" @click="deleteCustomModel(model)">
                  {{ t('settings.ai.delete') }}
                </el-button>
              </div>
            </div>
          </div>
          <div v-else class="ai-empty">{{ t('settings.ai.customEmpty') }}</div>
        </div>
      </div>

      <div class="settings-group">
        <div class="group-title">{{ t('settings.group.log') }}</div>
        <div class="setting-item">
          <span class="setting-label">{{ t('settings.logRetention') }}</span>
          <el-select v-model="logRetentionDays" class="setting-input" @change="saveLogRetention">
            <el-option :label="t('settings.days', { n: 7 })" :value="7" />
            <el-option :label="t('settings.days', { n: 14 })" :value="14" />
            <el-option :label="t('settings.days', { n: 30 })" :value="30" />
          </el-select>
        </div>
      </div>

      <div class="settings-group">
        <div class="group-title">{{ t('home.smb.connectedShares') }}</div>
        <div v-if="connectedShares.length > 0" class="smb-share-list">
          <div v-for="share in connectedShares" :key="share.unc_path" class="smb-share-item">
            <div class="smb-share-info">
              <span class="smb-share-path">{{ share.unc_path }}</span>
              <el-tag :type="share.accessible ? 'success' : 'danger'" size="small" effect="plain">
                {{ share.accessible ? t('home.smb.statusConnected') : t('home.smb.statusDisconnected') }}
              </el-tag>
            </div>
            <el-button link type="danger" size="small" @click="disconnectShare(share)">
              {{ t('home.smb.disconnect') }}
            </el-button>
          </div>
        </div>
        <div v-else class="smb-empty">{{ t('home.smb.noShares') }}</div>
      </div>

      <div class="settings-group">
        <div class="group-title">{{ t('settings.group.legal') }}</div>
        <div class="legal-links">
          <span class="legal-link" @click="goToPrivacyPolicy">{{ t('settings.privacyPolicy') }}</span>
          <span class="legal-separator">|</span>
          <span class="legal-link" @click="goToLicenseAgreement">{{ t('settings.licenseAgreement') }}</span>
          <span class="legal-separator">|</span>
          <span class="legal-link" @click="goToAbout">{{ t('settings.aboutLeafy') }}</span>
        </div>
      </div>

      <p class="copyright">Copyright © {{ copyrightYear }} Yangshengzhou. All Rights Reserved.</p>
    </div>

    <!-- 添加模型对话框 -->
    <el-dialog v-model="showAddModelDialog" :title="t('settings.ai.addModelTitle')" width="480px">
      <el-form label-width="80px">
        <el-form-item :label="t('settings.ai.modelName')">
          <el-input v-model="newModelName" :placeholder="t('settings.ai.modelNamePlaceholder')" />
        </el-form-item>
        <el-form-item :label="t('settings.ai.modelPath')">
          <div class="dialog-path-row">
            <el-input v-model="newModelPath" :placeholder="t('settings.ai.modelPathPlaceholder')" style="flex: 1" />
            <el-button @click="browseModelFile">{{ t('settings.ai.browse') }}</el-button>
          </div>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showAddModelDialog = false">{{ t('common.cancel') }}</el-button>
        <el-button type="primary" @click="addCustomModel">{{ t('common.confirm') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onActivated, onMounted, onUnmounted, watch, inject, type Ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { open } from '@tauri-apps/plugin-dialog'
import { ElNotification, ElMessageBox } from 'element-plus'
import { Moon, Sunny } from '@element-plus/icons-vue'
import type { FunctionPanel, ModelDefinition, SMBShareInfo } from '@/types'
import { t, setLanguage, locale } from '@/i18n'
import { APP_VERSION, GITHUB_URL } from '@/utils/appInfo'
import { loadSettings, saveSettings, type ThemeMode } from '@/utils/settings'

const activePanel = inject<Ref<FunctionPanel>>('activePanel')
// 设置页在 KeepAlive 中常驻，只有当前显示时才弹通知，否则与 App.vue 的全局监听重复
const isSettingsActive = () => !activePanel || activePanel.value === 'settings'

const theme = ref<ThemeMode>(loadSettings().theme === 'dark' ? 'dark' : 'light')
const isDark = computed({
  get: () => theme.value === 'dark',
  set: (val: boolean) => { theme.value = val ? 'dark' : 'light' },
})

watch(theme, (val) => {
  document.documentElement.dataset.theme = val
  saveSettings({ theme: val })
})

const language = computed({
  get: () => locale.value,
  set: (val: any) => setLanguage(val),
})

const copyrightYear = new Date().getFullYear()

const closeAction = ref<'exit' | 'minimize' | 'tray'>(loadSettings().closeAction ?? 'exit')

watch(closeAction, (val) => {
  saveSettings({ closeAction: val })
})

// 桌面端 webview 不会自行打开外部链接，交给系统浏览器
async function openGithub() {
  try {
    await invoke('open_url', { url: GITHUB_URL })
  } catch (e) {
    ElNotification({ type: 'error', title: t('common.error'), message: t('app.openUrlFailed', { msg: String(e) }) })
  }
}

// ===== SMB 共享管理 =====
const connectedShares = ref<SMBShareInfo[]>([])

async function loadConnectedShares() {
  try {
    connectedShares.value = await invoke<SMBShareInfo[]>('smb_list_connected')
  } catch {
    connectedShares.value = []
  }
}

async function disconnectShare(share: SMBShareInfo) {
  try {
    await ElMessageBox.confirm(
      t('home.smb.disconnectConfirm', { path: share.unc_path }),
      t('home.smb.disconnect'),
      { type: 'warning' },
    )
  } catch {
    return // 用户取消
  }

  try {
    await invoke('smb_disconnect', { uncPath: share.unc_path })
    ElNotification({ type: 'success', title: t('common.success'), message: t('home.smb.disconnectSuccess', { path: share.unc_path }) })
    await loadConnectedShares()
  } catch (e: any) {
    ElNotification({ type: 'error', title: t('common.error'), message: String(e) })
  }
}

function goToPrivacyPolicy() {
  if (activePanel) {
    activePanel.value = 'privacy-policy'
  }
}

const logRetentionDays = ref(30)

// ===== AI 模型管理 =====
interface CustomModel {
  id: string
  name: string
}

const builtinModels = ref<ModelDefinition[]>([])
const customModels = ref<CustomModel[]>([])
const defaultModelId = ref<string>(loadSettings().defaultModelId || '')
const showAddModelDialog = ref(false)
const newModelName = ref('')
const newModelPath = ref('')

onActivated(async () => {
  try {
    const settings = await invoke<{ log_retention_days: number }>('get_app_settings')
    logRetentionDays.value = settings.log_retention_days
  } catch {
    // 读取失败时使用默认 30 天
  }

  try {
    builtinModels.value = await invoke<ModelDefinition[]>('list_builtin_models')
    if (!defaultModelId.value && builtinModels.value.length > 0) {
      defaultModelId.value = builtinModels.value[0].id
    }
  } catch {
    // 加载内置模型失败
  }

  customModels.value = loadSettings().customModels || []

  await loadConnectedShares()
})

function setDefaultModel(modelId: string) {
  defaultModelId.value = modelId
  saveSettings({ defaultModelId: modelId })
}

async function browseModelFile() {
  const selected = await open({
    title: t('settings.ai.modelPath'),
    filters: [{ name: 'ONNX Model', extensions: ['onnx'] }],
    multiple: false,
  })
  if (selected && typeof selected === 'string') {
    newModelPath.value = selected
    if (!newModelName.value) {
      const parts = selected.split(/[\\/]/)
      const fileName = parts[parts.length - 1]?.replace(/\.onnx$/, '') || ''
      newModelName.value = fileName
    }
  }
}

async function addCustomModel() {
  if (!newModelName.value.trim() || !newModelPath.value.trim()) {
    return
  }
  try {
    const valid = await invoke<boolean>('validate_model_path', { modelPath: newModelPath.value })
    if (!valid) {
      ElNotification({ type: 'error', title: t('common.error'), message: t('settings.ai.invalidModel') })
      return
    }
  } catch {
    ElNotification({ type: 'error', title: t('common.error'), message: t('settings.ai.invalidModel') })
    return
  }

  const newModel: CustomModel = {
    id: newModelPath.value,
    name: newModelName.value.trim(),
  }
  customModels.value = [...customModels.value, newModel]
  saveSettings({ customModels: customModels.value })
  showAddModelDialog.value = false
  newModelName.value = ''
  newModelPath.value = ''
  ElNotification({ type: 'success', title: t('common.success'), message: t('settings.ai.addSuccess') })
}

async function deleteCustomModel(model: CustomModel) {
  try {
    await ElMessageBox.confirm(
      t('settings.ai.confirmDelete', { name: model.name }),
      t('common.warning'),
      { type: 'warning' },
    )
    customModels.value = customModels.value.filter(m => m.id !== model.id)
    saveSettings({ customModels: customModels.value })
    if (defaultModelId.value === model.id && builtinModels.value.length > 0) {
      defaultModelId.value = builtinModels.value[0].id
      saveSettings({ defaultModelId: defaultModelId.value })
    }
    ElNotification({ type: 'success', title: t('common.success'), message: t('settings.ai.deleteSuccess') })
  } catch {
    // 用户取消
  }
}

async function saveLogRetention() {
  try {
    await invoke('set_log_retention', { days: logRetentionDays.value })
    ElNotification({ type: 'success', title: t('common.success'), message: t('settings.retentionSaved') })
  } catch (e: any) {
    ElNotification({ type: 'error', title: t('common.error'), message: t('settings.retentionSaveFailed', { e }) })
  }
}

function goToLicenseAgreement() {
  if (activePanel) {
    activePanel.value = 'license-agreement'
  }
}

function goToAbout() {
  if (activePanel) {
    activePanel.value = 'about'
  }
}

// ===== 更新检查 =====
const checkingUpdate = ref(false)
const downloadPercent = ref(0)
let unlisteners: Array<() => void> = []
let updateTimeout: number | null = null
let disposed = false

function finishUpdateCheck() {
  checkingUpdate.value = false
  if (updateTimeout) {
    clearTimeout(updateTimeout)
    updateTimeout = null
  }
}

// 检查请求本身最长 30 秒；一旦进入下载，后端最长允许 10 分钟
const CHECK_TIMEOUT_MS = 60_000
const DOWNLOAD_TIMEOUT_MS = 11 * 60_000

function armUpdateTimeout(ms: number) {
  if (updateTimeout) clearTimeout(updateTimeout)
  updateTimeout = window.setTimeout(() => {
    if (checkingUpdate.value) {
      finishUpdateCheck()
      ElNotification({ type: 'error', title: t('common.error'), message: t('app.updateCheckFailed', { msg: 'timeout' }) })
    }
  }, ms)
}

async function handleCheckUpdate() {
  if (checkingUpdate.value) return
  checkingUpdate.value = true
  downloadPercent.value = 0
  // 后端若始终不回报事件，界面也不会永久转圈
  armUpdateTimeout(CHECK_TIMEOUT_MS)
  try {
    await invoke('check_for_updates')
  } catch (e: any) {
    finishUpdateCheck()
    ElNotification({ type: 'error', title: t('common.error'), message: t('app.updateCheckFailed', { msg: String(e) }) })
  }
}

function setupUpdateListeners() {
  // 清理旧监听
  unlisteners.forEach(fn => fn())
  unlisteners = []
  // 监听注册是异步的：若在注册完成前组件已卸载，直接注销，避免泄漏
  const addUnlisten = (fn: Promise<() => void>) => {
    fn.then(un => {
      if (disposed) un()
      else unlisteners.push(un)
    })
  }

  addUnlisten(listen<{ version: string; releaseNotes?: string }>('update-available', (event) => {
    if (!isSettingsActive()) return
    const { version, releaseNotes } = event.payload
    ElNotification({
      type: 'info',
      title: t('app.updateFoundTitle'),
      message: t('app.updateFoundMessage', { version }),
      duration: 0,
    })
    if (releaseNotes) {
      console.log('Release notes:', releaseNotes)
    }
  }))

  addUnlisten(listen<{ percent: number }>('update-download-progress', (event) => {
    downloadPercent.value = event.payload.percent
    // 下载可能远超检查请求的超时时间，收到进度后改用下载时限
    if (checkingUpdate.value) armUpdateTimeout(DOWNLOAD_TIMEOUT_MS)
  }))

  addUnlisten(listen<{ version: string; releaseNotes?: string }>('update-downloaded', (event) => {
    finishUpdateCheck()
    downloadPercent.value = 100
    if (!isSettingsActive()) return
    const { version } = event.payload
    ElNotification({
      type: 'success',
      title: t('app.updateDownloadedTitle'),
      message: t('app.updateDownloadedMessage', { version }),
      duration: 0,
    })
  }))

  addUnlisten(listen<{ version: string; releaseNotes?: string }>('update-installed', (event) => {
    finishUpdateCheck()
    downloadPercent.value = 0
    const { version, releaseNotes } = event.payload
    if (version && isSettingsActive()) {
      ElNotification({
        type: 'success',
        title: t('app.updateInstalledTitle'),
        message: t('app.updateInstalledMessage', { version }),
      })
      if (releaseNotes) {
        console.log('Release notes:', releaseNotes)
      }
    }
  }))

  // 后端改用独立事件表示"已是最新版本"，不再靠空 payload 猜测
  addUnlisten(listen('update-not-available', () => {
    finishUpdateCheck()
    downloadPercent.value = 0
    if (!isSettingsActive()) return
    ElNotification({ type: 'success', title: t('common.success'), message: t('app.noUpdateAvailable') })
  }))

  addUnlisten(listen<{ message: string }>('update-error', (event) => {
    finishUpdateCheck()
    downloadPercent.value = 0
    if (!isSettingsActive()) return
    ElNotification({
      type: 'error',
      title: t('common.error'),
      message: t('app.updateCheckFailed', { msg: event.payload.message }),
    })
  }))
}

onMounted(() => {
  disposed = false
  setupUpdateListeners()
})

onUnmounted(() => {
  disposed = true
  unlisteners.forEach(fn => fn())
  unlisteners = []
  if (updateTimeout) {
    clearTimeout(updateTimeout)
    updateTimeout = null
  }
})
</script>

<style scoped>
.settings-page {
  height: 100%;
  background: var(--bg);
  overflow-y: auto;
}

.settings-content {
  padding: 24px;
}

.settings-group {
  margin-bottom: 24px;
}

.group-title {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
  margin-bottom: 12px;
  padding-bottom: 8px;
  border-bottom: 1px solid var(--panel-2);
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 0;
  gap: 16px;
}

.setting-label {
  font-size: 13px;
  color: var(--text-secondary);
  flex-shrink: 0;
  min-width: 120px;
}

.setting-input {
  width: 240px;
}

.about-content {
  text-align: center;
  padding: 24px 0;
}

.about-desc {
  font-size: 13px;
  color: var(--text-secondary);
  max-width: 520px;
  margin: 0 auto 16px;
  line-height: 1.8;
}

.feature-tags {
  display: flex;
  justify-content: center;
  gap: 8px;
  flex-wrap: wrap;
  margin-bottom: 16px;
}

.feature-tag {
  padding: 4px 10px;
  background: var(--panel-2);
  color: var(--text-secondary);
  border-radius: 6px;
  font-size: 12px;
}

.about-content h2 {
  font-size: 16px;
  color: var(--text);
  margin-bottom: 16px;
  font-weight: 500;
}

.about-content .version {
  font-size: 13px;
  color: var(--text-muted);
  margin-left: 8px;
}

.tech-stack {
  display: flex;
  justify-content: center;
  gap: 8px;
  flex-wrap: wrap;
  margin-bottom: 16px;
}

.tech-badge {
  padding: 6px 12px;
  background: var(--panel);
  color: var(--text);
  border-radius: 6px;
  font-size: 12px;
}

.license-row {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  margin-bottom: 4px;
}

.license {
  font-size: 12px;
  color: var(--text-muted);
  margin-bottom: 0;
}

.github-link {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  border-radius: 6px;
  color: var(--text-secondary);
  background: var(--panel-2);
  font-size: 12px;
  text-decoration: none;
  cursor: pointer;
  transition: color 0.2s, background 0.2s;
}

.github-link:hover {
  color: var(--text);
  background: var(--panel-2-hover);
}

.legal-links {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 16px 0;
}

.legal-link {
  color: var(--primary);
  font-size: 13px;
  cursor: pointer;
  transition: color 0.2s;
  user-select: none;
}

.legal-link:hover {
  color: var(--primary-hover);
}

.legal-separator {
  color: var(--text-muted);
  font-size: 13px;
}

.copyright {
  text-align: center;
  font-size: 12px;
  color: var(--text-muted);
  padding: 12px 0 4px;
}

/* ===== AI 模型 ===== */
.ai-section {
  margin-bottom: 16px;
}

.ai-section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
}

.ai-section-label {
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 8px;
}

.ai-section-header .ai-section-label {
  margin-bottom: 0;
}

.model-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.model-card {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 14px;
  background: var(--panel);
  border-radius: 8px;
  border: 1px solid var(--panel-2);
}

.model-card-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.model-card-name {
  font-size: 13px;
  color: var(--text);
  font-weight: 500;
}

.model-card-meta {
  font-size: 11px;
  color: var(--text-muted);
}

.model-card-path {
  font-size: 11px;
  color: var(--text-muted);
  font-family: monospace;
  max-width: 240px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.model-card-actions {
  display: flex;
  align-items: center;
  gap: 4px;
}

.ai-empty {
  font-size: 12px;
  color: var(--text-muted);
  padding: 8px 0;
}

.dialog-path-row {
  display: flex;
  gap: 8px;
  width: 100%;
}

/* ===== SMB 共享管理 ===== */
.smb-share-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.smb-share-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 14px;
  background: var(--panel);
  border-radius: 8px;
  border: 1px solid var(--panel-2);
}

.smb-share-info {
  display: flex;
  align-items: center;
  gap: 8px;
}

.smb-share-path {
  font-size: 13px;
  color: var(--text);
  font-family: monospace;
}

.smb-empty {
  font-size: 12px;
  color: var(--text-muted);
  padding: 8px 0;
}

/* ===== 检查更新 ===== */
.update-section {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 12px;
  margin-top: 12px;
}

.update-progress {
  font-size: 12px;
  color: var(--text-secondary);
}
</style>
