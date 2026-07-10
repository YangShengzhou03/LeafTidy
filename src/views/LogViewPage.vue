<template>
  <div class="page-container">
    <div class="page-header">
      <h2>操作日志</h2>
      <p class="desc">查看所有操作记录，支持按类型筛选和详情查看</p>
    </div>
    <div class="page-content">
      <div class="filter-bar">
        <div class="filter-left">
          <el-select v-model="filterType" placeholder="全部类型" clearable style="width: 160px">
            <el-option label="全部类型" value="" />
            <el-option label="文件整理" value="organize" />
            <el-option label="批量重命名" value="rename" />
            <el-option label="重复清理" value="duplicate_clean" />
            <el-option label="附属清理" value="cleanup" />
          </el-select>
          <el-button @click="loadLogs">查询</el-button>
        </div>
        <el-button type="danger" @click="clearAllLogs" v-show="logs.length > 0">清空</el-button>
      </div>
      <div class="log-list">
        <el-table :data="logs" style="width: 100%" empty-text="暂无日志记录">
          <el-table-column prop="timestamp" label="时间" width="180" />
          <el-table-column prop="operation_type" label="类型" width="100">
            <template #default="{ row }">
              {{ getOperationTypeLabel(row.operation_type) }}
            </template>
          </el-table-column>
          <el-table-column prop="source_path" label="处理路径" min-width="200" show-overflow-tooltip />
          <el-table-column prop="target_path" label="目标路径" min-width="200" show-overflow-tooltip>
            <template #default="{ row }">
              {{ row.target_path || '-' }}
            </template>
          </el-table-column>
          <el-table-column prop="status" label="状态" width="80">
            <template #default="{ row }">
              <span :class="['status-tag', row.status]">{{ getStatusLabel(row.status) }}</span>
            </template>
          </el-table-column>
          <el-table-column label="操作" width="160" fixed="right">
            <template #default="{ row }">
              <div class="op-buttons">
                <el-button text @click="viewLogDetail(row)">查看</el-button>
                <el-button text type="danger" @click="deleteLog(row)">删除</el-button>
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
import { ElMessageBox, ElMessage } from 'element-plus'
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
  const labels: Record<string, string> = {
    organize: '文件整理',
    rename: '批量重命名',
    duplicate_clean: '重复清理',
    cleanup: '附属清理',
    move: '移动',
    copy: '复制',
    delete: '删除',
  }
  return labels[type] || type
}

function getStatusLabel(status: string): string {
  const labels: Record<string, string> = {
    success: '成功',
    fail: '失败',
  }
  return labels[status] || status
}

async function clearAllLogs() {
  try {
    await ElMessageBox.confirm(
      '确定要清空所有日志吗？此操作不可恢复。',
      '确认清空',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      }
    )
    await invoke<number>('clear_all_logs')
    ElMessage.success('日志已清空')
    logs.value = []
  } catch (e) {
    if (e !== 'cancel' && e !== 'close') {
      console.error('清空日志失败:', e)
    }
  }
}

async function deleteLog(log: LogEntry) {
  try {
    await ElMessageBox.confirm(
      '确定要删除这条日志吗？',
      '确认删除',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      }
    )
    const success = await invoke<boolean>('delete_log', { logId: log.id })
    if (success) {
      ElMessage.success('日志已删除')
      logs.value = logs.value.filter(l => l.id !== log.id)
    }
  } catch (e) {
    if (e !== 'cancel' && e !== 'close') {
      console.error('删除日志失败:', e)
    }
  }
}

onMounted(loadLogs)
</script>

<style scoped>
.page-container {
  height: 100%;
  background: #18191C;
  padding: 24px;
  overflow-y: auto;
}

.page-header {
  margin-bottom: 24px;
}

.page-header h2 {
  font-size: 16px;
  font-weight: 500;
  color: #E0E6ED;
  margin-bottom: 6px;
}

.page-header .desc {
  font-size: 13px;
  color: #8A94A6;
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
  background: #1F2023;
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
  background: rgba(82, 196, 26, 0.15);
  color: #52C41A;
}

.status-tag.fail {
  background: rgba(232, 17, 35, 0.15);
  color: #E81123;
}
</style>
