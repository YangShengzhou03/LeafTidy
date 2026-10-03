<template>
  <div class="page-container">
    <div class="page-header">
      <span class="page-title">{{ currentLogDate }} ({{ entries.length }} {{ t('logs.detail.count') }})</span>
      <div class="header-actions">
        <el-button @click="copyAllLogs" :disabled="entries.length === 0">{{ t('logs.action.copy') }}</el-button>
        <el-button @click="saveLogs" :disabled="entries.length === 0" type="primary">{{ t('logs.action.save') }}</el-button>
      </div>
    </div>
    <pre v-if="entries.length > 0" class="log-content">{{ logText }}</pre>
    <div v-else class="empty">{{ t('logs.empty') }}</div>
  </div>
</template>

<script setup lang="ts">
import { ref, inject, computed, onMounted, type Ref } from 'vue'
import { ElNotification } from 'element-plus'
import { t } from '@/i18n'
import { useLog } from '@/composables/useLog'
import { useFileOps } from '@/composables/useFileOps'
import type { LogEntry } from '@/types'

const currentLogDate = inject<Ref<string>>('currentLogDate')!
const { formatSize } = useFileOps()
const entries = ref<LogEntry[]>([])

const logText = computed(() =>
  entries.value
    .map(e => {
      let line = `[${e.timestamp}]`
      if (e.size > 0) line += ` (${formatSize(e.size)})`
      return `${line} ${e.action}`
    })
    .join('\n')
)

onMounted(async () => {
  if (currentLogDate.value) {
    const { queryLogsByDate } = useLog()
    entries.value = await queryLogsByDate(currentLogDate.value)
  }
})

function copyAllLogs() {
  navigator.clipboard.writeText(logText.value).then(() => {
    ElNotification({ type: 'success', title: t('logs.status.success'), message: t('logs.copiedAll'), duration: 1500 })
  })
}

function saveLogs() {
  const blob = new Blob([logText.value], { type: 'text/plain;charset=utf-8' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `leaf-tidy-log-${currentLogDate.value}.txt`
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  URL.revokeObjectURL(url)
}
</script>

<style scoped>
/* Page-specific styles (shared styles in components.css) */

.page-container {
  padding: 16px;
  display: flex;
  flex-direction: column;
}

.page-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}

.page-title {
  font-size: 13px;
  color: var(--text-secondary);
}

.header-actions {
  display: flex;
  gap: 8px;
}

.log-content {
  flex: 1;
  margin: 0;
  padding: 12px;
  background: var(--panel);
  border-radius: 6px;
  font-family: 'SF Mono', 'Consolas', 'Monaco', monospace;
  font-size: 12px;
  line-height: 1.6;
  color: var(--text);
  white-space: pre-wrap;
  word-break: break-all;
  user-select: text;
  overflow-y: auto;
}

.empty {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
  font-size: 13px;
}
</style>
