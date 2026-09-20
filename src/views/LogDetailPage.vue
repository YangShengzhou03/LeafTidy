<template>
  <div class="page-container">
    <div class="page-content" v-if="logDetail">
      <div class="detail-card">
        <div class="card-header">{{ t('logs.detail.basic') }}</div>
        <div class="card-body">
          <div class="info-row">
            <span class="info-label">{{ t('logs.detail.time') }}</span>
            <span class="info-value">{{ logDetail.timestamp }}</span>
          </div>
          <div class="info-row">
            <span class="info-label">{{ t('logs.detail.opType') }}</span>
            <span class="info-value">{{ getOperationTypeLabel(logDetail.operation_type) }}</span>
          </div>
          <div class="info-row">
            <span class="info-label">{{ t('logs.detail.status') }}</span>
            <span class="info-value">
              <span :class="['status-tag', logDetail.status]">{{ getStatusLabel(logDetail.status) }}</span>
            </span>
          </div>
          <div class="info-row">
            <span class="info-label">{{ t('logs.col.sourcePath') }}</span>
            <span class="info-value path-value">{{ logDetail.source_path }}</span>
          </div>
          <div class="info-row" v-if="logDetail.target_path">
            <span class="info-label">{{ t('logs.col.targetPath') }}</span>
            <span class="info-value path-value">{{ logDetail.target_path }}</span>
          </div>
        </div>
      </div>

      <div class="detail-card" v-if="logDetail.detail">
        <div class="card-header">{{ t('logs.detail.records') }}</div>
        <div class="card-body">
          <div class="log-reader">
            <pre class="log-content">{{ logDetail.detail }}</pre>
          </div>
        </div>
      </div>
    </div>
    <div class="empty-state" v-else>
      <el-icon :size="48">
        <Warning />
      </el-icon>
      <p>{{ t('logs.detail.notFound') }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, inject, onMounted, type Ref } from 'vue'
import { Warning } from '@element-plus/icons-vue'
import { t } from '@/i18n'
import type { LogEntry } from '@/types'

const currentLogId = inject<Ref<string>>('currentLogId')!
const logDetail = ref<LogEntry | null>(null)

function getOperationTypeLabel(type: string): string {
  const keys: Record<string, string> = {
    organize: 'logs.op.organize',
    rename: 'logs.op.rename',
    duplicate_clean: 'logs.op.duplicate_clean',
    cleanup: 'logs.op.cleanup',
    move: 'logs.op.move',
    copy: 'logs.op.copy',
    delete: 'logs.op.delete',
  }
  const key = keys[type]
  return key ? t(key) : type
}

function getStatusLabel(status: string): string {
  const keys: Record<string, string> = {
    success: 'logs.status.success',
    fail: 'logs.status.fail',
    cancelled: 'logs.status.cancelled',
  }
  const key = keys[status]
  return key ? t(key) : status
}

onMounted(async () => {
  if (currentLogId.value) {
    try {
      const { invoke } = await import('@tauri-apps/api/core')
      const logs = await invoke<LogEntry[]>('query_logs', {
        operationType: null,
        startDate: null,
        endDate: null,
        limit: 1000,
      })
      logDetail.value = logs.find(log => log.id === currentLogId.value) || null
    } catch (e) {
      console.error('Failed to load log details:', e)
      logDetail.value = null
    }
  }
})
</script>

<style scoped>
.page-container {
  height: 100%;
  background: var(--bg);
  padding: 24px;
  overflow-y: auto;
}

.page-content {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.detail-card {
  background: var(--panel);
  border-radius: 8px;
  overflow: hidden;
}

.card-header {
  padding: 16px 20px;
  font-size: 14px;
  font-weight: 500;
  color: var(--text);
  background: var(--panel-2);
  border-bottom: 1px solid var(--border);
}

.card-body {
  padding: 20px;
}

.info-row {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  margin-bottom: 16px;
}

.info-row:last-child {
  margin-bottom: 0;
}

.info-label {
  font-size: 13px;
  color: var(--text-muted);
  min-width: 80px;
  flex-shrink: 0;
}

.info-value {
  font-size: 13px;
  color: var(--text);
  word-break: break-all;
}

.path-value {
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 12px;
}

.status-tag {
  display: inline-block;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
}

.status-tag.success {
  background: rgba(var(--success-rgb), 0.15);
  color: var(--success);
}

.status-tag.fail {
  background: rgba(var(--danger-rgb), 0.15);
  color: var(--danger);
}

.status-tag.cancelled {
  background: rgba(var(--warning-rgb), 0.15);
  color: var(--warning);
}

.log-reader {
  background: var(--bg);
  border-radius: 6px;
  border: 1px solid var(--border);
  max-height: 600px;
  overflow: hidden;
}

.log-content {
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 12px;
  color: var(--text);
  padding: 16px;
  margin: 0;
  white-space: pre-wrap;
  word-break: break-all;
  max-height: 600px;
  overflow-y: auto;
  line-height: 1.6;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
  height: calc(100% - 80px);
  color: var(--text-muted);
}

.empty-state p {
  font-size: 14px;
  margin: 0;
}
</style>