<template>
  <div class="page-container">
    <div class="page-header">
      <div>
        <h2>{{ t('logs.title') }}</h2>
        <p class="desc">{{ t('logs.desc') }}</p>
      </div>
    </div>
    <div class="page-content">
      <div class="log-list">
        <div class="log-row log-head">
          <span>{{ t('logs.col.date') }}</span>
          <span>{{ t('logs.col.count') }}</span>
          <span>{{ t('logs.col.size') }}</span>
          <span>{{ t('logs.col.operation') }}</span>
        </div>
        <div v-for="row in dailyLogs" :key="row.date" class="log-row">
          <span class="log-date">{{ row.date }}</span>
          <span class="log-meta">{{ row.count }}</span>
          <span class="log-meta">{{ formatSize(row.totalSize) }}</span>
          <span class="row-actions">
            <el-button link @click="viewLogDetail(row)">{{ t('logs.action.view') }}</el-button>
            <el-button link type="danger" @click="deleteLog(row)">{{ t('logs.action.delete') }}</el-button>
          </span>
        </div>
        <div v-if="dailyLogs.length === 0" class="empty-state">{{ t('logs.empty') }}</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, inject, onMounted, type Ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessageBox, ElNotification } from 'element-plus'
import { t } from '@/i18n'
import { useLog } from '@/composables/useLog'
import { useFileOps } from '@/composables/useFileOps'
import type { LogDaySummary, FunctionPanel } from '@/types'

const activePanel = inject<Ref<FunctionPanel>>('activePanel')!
const currentLogDate = inject<Ref<string>>('currentLogDate')!
const { formatSize } = useFileOps()
const dailyLogs = ref<LogDaySummary[]>([])

async function loadLogs() {
  try {
    const { queryDailySummary } = useLog()
    dailyLogs.value = await queryDailySummary()
  } catch {
    dailyLogs.value = []
  }
}

function viewLogDetail(row: LogDaySummary) {
  currentLogDate.value = row.date
  activePanel.value = 'log-detail'
}

async function deleteLog(row: LogDaySummary) {
  try {
    await ElMessageBox.confirm(
      t('logs.confirm.deleteMessage'),
      t('logs.confirm.deleteTitle'),
      {
        confirmButtonText: t('logs.confirm.ok'),
        cancelButtonText: t('logs.confirm.cancel'),
        type: 'warning',
      }
    )
    await invoke('delete_logs_by_date', { date: row.date })
    ElNotification({ type: 'success', title: t('logs.status.success'), message: t('logs.deleted') })
    dailyLogs.value = dailyLogs.value.filter(l => l.date !== row.date)
  } catch (e) {
    if (e !== 'cancel' && e !== 'close') {
      ElNotification({ type: 'error', title: t('logs.confirm.deleteTitle'), message: t('app.operationFailed') })
    }
  }
}

onMounted(loadLogs)
</script>

<style scoped>
/* Page-specific styles (shared styles in components.css) */

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 24px;
}

.page-content {
  background: var(--panel);
  border-radius: 8px;
  padding: 8px 20px;
}

.log-list {
  display: flex;
  flex-direction: column;
}

.log-row {
  display: grid;
  /* 列宽全部固定，保证表头与数据行逐列对齐（auto/1fr 会在行间各自计算导致错位） */
  grid-template-columns: minmax(0, 1fr) 100px 120px 140px;
  align-items: center;
  gap: 12px;
  padding: 12px 4px;
  border-bottom: 1px solid var(--border);
  transition: background 0.15s;
}

.log-row:last-of-type {
  border-bottom: none;
}

.log-row:not(.log-head):hover {
  background: var(--panel-2);
}

.log-head {
  font-size: 12px;
  color: var(--text-muted);
  padding: 10px 4px;
}

.log-date {
  font-size: 13px;
  color: var(--text);
  font-weight: 500;
}

.log-meta {
  font-size: 13px;
  color: var(--text-secondary);
}

.row-actions {
  display: flex;
  justify-content: flex-start;
  gap: 4px;
}
</style>
