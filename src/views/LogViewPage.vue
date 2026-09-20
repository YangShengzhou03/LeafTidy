<template>
  <div class="page-container">
    <div class="page-header">
      <h2>{{ t('logs.title') }}</h2>
      <p class="desc">{{ t('logs.desc') }}</p>
    </div>
    <div class="page-content">
      <div class="filter-bar">
        <div class="filter-left">
          <el-select v-model="filterType" :placeholder="t('logs.filter.all')" clearable style="width: 160px">
            <el-option :label="t('logs.filter.all')" value="" />
            <el-option :label="t('logs.op.organize')" value="organize" />
            <el-option :label="t('logs.op.rename')" value="rename" />
            <el-option :label="t('logs.op.duplicate_clean')" value="duplicate_clean" />
            <el-option :label="t('logs.op.cleanup')" value="cleanup" />
          </el-select>
          <el-button @click="loadLogs">{{ t('logs.query') }}</el-button>
        </div>
        <el-button type="danger" @click="clearAllLogs" v-show="logs.length > 0">{{ t('logs.clear') }}</el-button>
      </div>
      <div class="log-list">
        <el-table :data="logs" style="width: 100%" :empty-text="t('logs.empty')">
          <el-table-column prop="timestamp" :label="t('logs.col.time')" width="180" />
          <el-table-column prop="operation_type" :label="t('logs.col.type')" width="100">
            <template #default="{ row }">
              {{ getOperationTypeLabel(row.operation_type) }}
            </template>
          </el-table-column>
          <el-table-column prop="source_path" :label="t('logs.col.sourcePath')" min-width="200" show-overflow-tooltip />
          <el-table-column prop="target_path" :label="t('logs.col.targetPath')" min-width="200" show-overflow-tooltip>
            <template #default="{ row }">
              {{ row.target_path || '-' }}
            </template>
          </el-table-column>
          <el-table-column prop="status" :label="t('logs.col.status')" width="80">
            <template #default="{ row }">
              <span :class="['status-tag', row.status]">{{ getStatusLabel(row.status) }}</span>
            </template>
          </el-table-column>
          <el-table-column :label="t('logs.col.actions')" width="160" fixed="right">
            <template #default="{ row }">
              <div class="op-buttons">
                <el-button text @click="viewLogDetail(row)">{{ t('logs.action.view') }}</el-button>
                <el-button text type="danger" @click="deleteLog(row)">{{ t('logs.action.delete') }}</el-button>
              </div>
            </template>
          </el-table-column>
        </el-table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, inject, onMounted, type Ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessageBox, ElNotification } from 'element-plus'
import { t } from '@/i18n'
import type { LogEntry, FunctionPanel } from '@/types'

const activePanel = inject<Ref<FunctionPanel>>('activePanel')!
const currentLogId = inject<Ref<string>>('currentLogId')!
const filterType = ref('')
const logs = ref<LogEntry[]>([])

async function loadLogs() {
  try {
    logs.value = await invoke<LogEntry[]>('query_logs', {
      operationType: filterType.value || null,
      startDate: null,
      endDate: null,
      limit: 200,
    })
  } catch {
    logs.value = []
  }
}

function viewLogDetail(log: LogEntry) {
  currentLogId.value = log.id
  activePanel.value = 'log-detail'
}

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

async function clearAllLogs() {
  try {
    await ElMessageBox.confirm(
      t('logs.confirm.clearMessage'),
      t('logs.confirm.clearTitle'),
      {
        confirmButtonText: t('logs.confirm.ok'),
        cancelButtonText: t('logs.confirm.cancel'),
        type: 'warning',
      }
    )
    await invoke<number>('clear_all_logs')
    ElNotification({ type: 'success', title: t('logs.status.success'), message: t('logs.cleared') })
    logs.value = []
  } catch (e) {
    if (e !== 'cancel' && e !== 'close') {
      console.error('Failed to clear logs:', e)
    }
  }
}

async function deleteLog(log: LogEntry) {
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
    const success = await invoke<boolean>('delete_log', { logId: log.id })
    if (success) {
      ElNotification({ type: 'success', title: t('logs.status.success'), message: t('logs.deleted') })
      logs.value = logs.value.filter(l => l.id !== log.id)
    }
  } catch (e) {
    if (e !== 'cancel' && e !== 'close') {
      console.error('Failed to delete log:', e)
    }
  }
}

onMounted(loadLogs)
</script>

<style scoped>
.page-container {
  height: 100%;
  background: var(--bg);
  padding: 24px;
  overflow-y: auto;
}

.page-header {
  margin-bottom: 24px;
}

.page-header h2 {
  font-size: 16px;
  font-weight: 500;
  color: var(--text);
  margin-bottom: 6px;
}

.page-header .desc {
  font-size: 13px;
  color: var(--text-muted);
}

.filter-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 16px;
}

.filter-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.op-buttons {
  display: flex;
  gap: 8px;
}

.log-list {
  background: var(--panel);
  border-radius: 8px;
  padding: 16px;
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
</style>
