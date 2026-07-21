<template>
  <div class="settings-page">
    <div class="settings-content">
      <div class="settings-group">
        <div class="group-title">关于</div>
        <div class="about-content">
          <h2>轻羽归档 LeafTidy <span class="version">v1.0.0</span></h2>
          <div class="tech-stack">
            <span class="tech-badge">Tauri</span>
            <span class="tech-badge">Rust</span>
            <span class="tech-badge">Vue 3</span>
            <span class="tech-badge">TypeScript</span>
            <span class="tech-badge">Element Plus</span>
          </div>
          <p class="license">MIT License</p>
        </div>
      </div>

      <div class="settings-group">
        <div class="group-title">法律信息</div>
        <div class="legal-links">
          <span class="legal-link" @click="goToPrivacyPolicy">隐私政策</span>
          <span class="legal-separator">|</span>
          <span class="legal-link" @click="goToLicenseAgreement">许可协议</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { inject } from 'vue'
import type { Ref } from 'vue'
import type { FunctionPanel } from '@/types'

const activePanel = inject<Ref<FunctionPanel>>('activePanel')

function goToPrivacyPolicy() {
  if (activePanel) {
    activePanel.value = 'privacy-policy'
  }
}

function goToLicenseAgreement() {
  if (activePanel) {
    activePanel.value = 'license-agreement'
  }
}
</script>

<style scoped>
.settings-page {
  height: 100%;
  background: #18191C;
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
  color: #C8D0DC;
  margin-bottom: 12px;
  padding-bottom: 8px;
  border-bottom: 1px solid #2A2B30;
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
  color: #C8D0DC;
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

.about-content h2 {
  font-size: 16px;
  color: #E0E6ED;
  margin-bottom: 16px;
  font-weight: 500;
}

.about-content .version {
  font-size: 13px;
  color: #8A94A6;
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
  background: #1F2023;
  color: #E0E6ED;
  border-radius: 6px;
  font-size: 12px;
}

.license {
  font-size: 12px;
  color: #8A94A6;
}

.legal-links {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 16px 0;
}

.legal-link {
  color: #3A86FF;
  font-size: 13px;
  cursor: pointer;
  transition: color 0.2s;
  user-select: none;
}

.legal-link:hover {
  color: #5A9FFF;
}

.legal-separator {
  color: #8A94A6;
  font-size: 13px;
}
</style>