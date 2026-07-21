<template>
  <div class="page-container">
    <div class="page-content" v-if="logDetail">
      <div class="detail-card">
        <div class="card-header">基本信息</div>
        <div class="card-body">
          <div class="info-row">
            <span class="info-label">执行时间</span>
            <span class="info-value">{{ logDetail.timestamp }}</span>
          </div>
          <div class="info-row">
            <span class="info-label">操作类型</span>
            <span class="info-value">{{ getOperationTypeLabel(logDetail.operation_type) }}</span>
          </div>
          <div class="info-row">
            <span class="info-label">执行状态</span>
            <span class="info-value">
              <span :class="['status-tag', logDetail.status]">{{ getStatusLabel(logDetail.status) }}</span>
            </span>
          </div>
          <div class="info-row">
            <span class="info-label">处理路径</span>
            <span class="info-value path-value">{{ logDetail.source_path }}</span>
          </div>
          <div class="info-row" v-if="logDetail.target_path">
            <span class="info-label">目标路径</span>
            <span class="info-value path-value">{{ logDetail.target_path }}</span>
          </div>
        </div>
      </div>

      <div class="detail-card" v-if="logDetail.detail">
        <div class="card-header">详细操作记录</div>
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
      <p>未找到日志详情</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, inject, onMounted, type Ref } from 'vue'
import { Warning } from '@element-plus/icons-vue'
import type { LogEntry } from '@/types'

const currentLogId = inject<Ref<string>>('currentLogId')!
const logDetail = ref<LogEntry | null>(null)

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
    cancelled: '已取消',
  }
  return labels[status] || status
}

onMounted(async () => {
  if (currentLogId.value) {
    // 从日志列表中查询该日志详情
    // 这里需要通过 invoke 获取日志详情
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
      console.error('获取日志详情失败:', e)
      logDetail.value = null
    }
  }
})
</script>

<style scoped>
.page-container {
  height: 100%;
  background: #18191C;
  padding: 24px;
  overflow-y: auto;
}

.page-content {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.detail-card {
  background: #1F2023;
  border-radius: 8px;
  overflow: hidden;
}

.card-header {
  padding: 16px 20px;
  font-size: 14px;
  font-weight: 500;
  color: #E0E6ED;
  background: #2A2B30;
  border-bottom: 1px solid #3A3B40;
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
  color: #8A94A6;
  min-width: 80px;
  flex-shrink: 0;
}

.info-value {
  font-size: 13px;
  color: #E0E6ED;
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
  background: rgba(82, 196, 26, 0.15);
  color: #52C41A;
}

.status-tag.fail {
  background: rgba(232, 17, 35, 0.15);
  color: #E81123;
}

.status-tag.cancelled {
  background: rgba(250, 173, 20, 0.15);
  color: #FAAD14;
}

.log-reader {
  background: #18191C;
  border-radius: 6px;
  border: 1px solid #3A3B40;
  max-height: 600px;
  overflow: hidden;
}

.log-content {
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 12px;
  color: #E0E6ED;
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
  color: #8A94A6;
}

.empty-state p {
  font-size: 14px;
  margin: 0;
}
</style>