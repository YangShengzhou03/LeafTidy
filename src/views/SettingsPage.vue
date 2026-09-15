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
            <el-option label="简体中文" value="zh-CN" />
            <el-option label="繁體中文" value="zh-TW" />
            <el-option label="English" value="en" />
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
          <h2>{{ t('settings.appName') }} <span class="version">v1.0.0</span></h2>
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
            <a class="github-link" :href="GITHUB_URL" target="_blank" rel="noopener" title="GitHub">
              <svg viewBox="0 0 16 16" width="22" height="22" aria-hidden="true">
                <path fill="currentColor"
                  d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27s1.36.09 2 .27c1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.01 8.01 0 0 0 16 8c0-4.42-3.58-8-8-8Z" />
              </svg>
            </a>
            <p class="license">MIT License</p>
          </div>
        </div>
      </div>

      <div class="settings-group">
        <div class="group-title">{{ t('settings.group.log') }}</div>
        <div class="setting-item">
          <span class="setting-label">{{ t('settings.logRetention') }}</span>
          <el-select v-model="logRetentionDays" class="setting-input" @change="saveLogRetention">
            <el-option :label="t('settings.days', { n: 7 })" :value="7" />
            <el-option :label="t('settings.days', { n: 30 })" :value="30" />
            <el-option :label="t('settings.days', { n: 90 })" :value="90" />
            <el-option :label="t('settings.days', { n: 365 })" :value="365" />
          </el-select>
        </div>
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
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { inject } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElNotification } from 'element-plus'
import { Moon, Sunny } from '@element-plus/icons-vue'
import type { Ref } from 'vue'
import type { FunctionPanel } from '@/types'
import { t, setLanguage, locale } from '@/i18n'

const activePanel = inject<Ref<FunctionPanel>>('activePanel')

const SETTINGS_KEY = 'leaf-tidy-settings'
const GITHUB_URL = 'https://github.com/YangShengzhou03'

function loadSettings(): Record<string, any> {
  try {
    return JSON.parse(localStorage.getItem(SETTINGS_KEY) || '{}')
  } catch {
    return {}
  }
}

function saveSettings(patch: Record<string, any>) {
  const saved = loadSettings()
  Object.assign(saved, patch)
  localStorage.setItem(SETTINGS_KEY, JSON.stringify(saved))
}

const theme = ref(loadSettings().theme === 'light' ? 'light' : 'dark')
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

const closeAction = ref<'exit' | 'minimize' | 'tray'>(loadSettings().closeAction === 'minimize' ? 'minimize' : loadSettings().closeAction === 'tray' ? 'tray' : 'exit')

watch(closeAction, (val) => {
  saveSettings({ closeAction: val })
})

function goToPrivacyPolicy() {
  if (activePanel) {
    activePanel.value = 'privacy-policy'
  }
}

const logRetentionDays = ref(30)

onMounted(async () => {
  try {
    const settings = await invoke<{ log_retention_days: number }>('get_app_settings')
    logRetentionDays.value = settings.log_retention_days
  } catch {
    // 读取失败时使用默认 30 天
  }
})

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
  justify-content: center;
  width: 28px;
  height: 28px;
  border-radius: 8px;
  color: var(--text-secondary);
  transition: color 0.2s, background 0.2s;
}

.github-link:hover {
  color: var(--text);
  background: var(--panel-2);
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
</style>
