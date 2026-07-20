<template>
  <div class="log-files-page">
    <div class="page-header">
      <h1 class="page-title">日志文件</h1>
      <p class="page-subtitle">管理系统运行日志文件</p>
    </div>

    <div class="log-files-grid">
      <div class="log-file-card" v-for="logFile in logFiles" :key="logFile.name">
        <div class="log-file-icon">
          <el-icon :size="48" color="#533afd">
            <Document />
          </el-icon>
        </div>
        <div class="log-file-content">
          <div class="log-file-name" :title="logFile.name">{{ logFile.name }}</div>
          <div class="log-file-meta">
            <span class="log-file-date">{{ logFile.date }}</span>
            <span class="log-file-size">{{ logFile.size }}</span>
          </div>
        </div>
        <div class="log-file-actions">
          <el-button type="primary" @click="viewLog(logFile)">查看</el-button>
          <el-button @click="downloadLog(logFile)">下载</el-button>
          <el-button type="danger" @click="deleteLog(logFile)">删除</el-button>
        </div>
      </div>
    </div>

    <div v-if="logFiles.length === 0" class="empty-state">
      <div class="empty-icon">
        <el-icon :size="64" color="#64748d">
          <DocumentAdd />
        </el-icon>
      </div>
      <div class="empty-text">暂无日志文件</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Document, DocumentAdd } from '@element-plus/icons-vue'
import { invoke } from '@tauri-apps/api/core'

interface LogFile {
  name: string
  date: string
  size: string
  path: string
}

const router = useRouter()
const logFiles = ref<LogFile[]>([])

const loadLogFiles = async () => {
  try {
    const logs = await invoke<LogFile[]>('get_log_files')
    logFiles.value = logs
  } catch (error) {
    console.error('获取日志列表失败:', error)
    ElMessage.error('获取日志列表失败')
  }
}

const viewLog = (logFile: LogFile) => {
  router.push({
    path: '/log-viewer',
    query: { file: logFile.name }
  })
}

const downloadLog = async (logFile: LogFile) => {
  try {
    const content = await invoke<string>('read_log_file', { fileName: logFile.name })

    const blob = new Blob([content], { type: 'text/plain;charset=utf-8' })
    const url = URL.createObjectURL(blob)
    const link = document.createElement('a')
    link.href = url
    link.download = logFile.name
    link.click()
    URL.revokeObjectURL(url)
    ElMessage.success(`已下载: ${logFile.name}`)
  } catch (error) {
    console.error('下载日志失败:', error)
    ElMessage.error('下载日志失败')
  }
}

const deleteLog = async (logFile: LogFile) => {
  try {
    await ElMessageBox.confirm('确定要删除此日志文件吗?', '确认删除', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })

    await invoke('delete_log_file', { fileName: logFile.name })
    ElMessage.success(`已删除: ${logFile.name}`)
    loadLogFiles()
  } catch (error) {
    if (error !== 'cancel') {
      console.error('删除日志失败:', error)
      ElMessage.error('删除日志失败')
    }
  }
}

onMounted(() => {
  loadLogFiles()
})
</script>

<style scoped>
.log-files-page {
  max-width: 1320px;
  margin: 0 auto;
}

.page-header {
  margin-bottom: 32px;
}

.page-title {
  font-size: 48px;
  font-weight: 300;
  color: #061b31;
  margin: 0 0 8px 0;
  letter-spacing: -0.96px;
}

.page-subtitle {
  font-size: 22px;
  color: #64748d;
  margin: 0;
  font-weight: 300;
  letter-spacing: -0.22px;
}

.log-files-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(480px, 1fr));
  gap: 24px;
}

.log-file-card {
  background: #ffffff;
  border: 1px solid #e5edf5;
  border-radius: 4px;
  padding: 24px;
  display: flex;
  align-items: center;
  gap: 16px;
  transition: all 0.2s;
  min-width: 0;
}

.log-file-card:hover {
  border-color: #b9b9f9;
  box-shadow: 0 4px 12px rgba(83, 58, 253, 0.1);
}

.log-file-icon {
  flex-shrink: 0;
}

.log-file-content {
  flex: 1;
  min-width: 0;
  overflow: hidden;
}

.log-file-name {
  font-size: 18px;
  font-weight: 400;
  color: #061b31;
  margin-bottom: 8px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.log-file-meta {
  display: flex;
  gap: 16px;
  font-size: 14px;
  color: #64748d;
}

.log-file-date,
.log-file-size {
  font-weight: 300;
}

.log-file-actions {
  display: flex;
  gap: 8px;
  flex-shrink: 0;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px 24px;
  text-align: center;
}

.empty-icon {
  margin-bottom: 24px;
}

.empty-text {
  font-size: 18px;
  color: #64748d;
  font-weight: 300;
}
</style>