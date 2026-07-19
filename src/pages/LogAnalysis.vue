<template>
  <div class="log-analysis-page">
    <div class="page-header">
      <h1 class="page-title">日志分析</h1>
      <p class="page-subtitle">查看系统运行日志和错误信息</p>
    </div>

    <div class="divider"></div>

    <div class="section">
      <div class="section-header">
        <h2 class="section-title">日志文件</h2>
        <div class="toolbar">
          <el-button @click="refreshLogs">刷新</el-button>
          <el-button @click="deleteAllLogs" type="danger">清空所有日志</el-button>
        </div>
      </div>

      <div class="log-files">
        <div class="log-file" v-for="logFile in logFiles" :key="logFile.name">
          <div class="log-file-info">
            <div class="log-file-icon">
              <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#533afd" stroke-width="1.5">
                <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
                <polyline points="14 2 14 8 20 8"/>
              </svg>
            </div>
            <div class="log-file-details">
              <div class="log-file-name">{{ logFile.name }}</div>
              <div class="log-file-meta">
                <span>{{ logFile.date }}</span>
                <span>{{ logFile.size }}</span>
              </div>
            </div>
          </div>
          <div class="log-file-actions">
            <el-button size="small" @click="viewLog(logFile.name)">查看</el-button>
            <el-button size="small" @click="downloadLog(logFile.name)">下载</el-button>
            <el-button size="small" type="danger" @click="deleteLog(logFile.name)">删除</el-button>
          </div>
        </div>
      </div>

      <div v-if="logFiles.length === 0" class="empty-state">
        <div class="empty-icon">
          <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="#64748d" stroke-width="1.5">
            <path d="M20 12v6a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h6"/>
            <polyline points="14 2 14 8 20 8"/>
            <line x1="12" y1="18" x2="12" y2="12"/>
            <line x1="9" y1="15" x2="15" y2="15"/>
          </svg>
        </div>
        <div class="empty-text">暂无日志文件</div>
      </div>
    </div>

    <div class="divider"></div>

    <div class="section">
      <div class="section-header">
        <h2 class="section-title">日志内容</h2>
        <div class="toolbar">
          <el-input v-model="searchKeyword" placeholder="搜索关键词" style="width: 300px" clearable />
          <el-select v-model="logLevel" placeholder="日志级别" style="width: 150px">
            <el-option label="全部" value="" />
            <el-option label="INFO" value="INFO" />
            <el-option label="WARN" value="WARN" />
            <el-option label="ERROR" value="ERROR" />
            <el-option label="DEBUG" value="DEBUG" />
          </el-select>
        </div>
      </div>

      <div class="log-viewer">
        <div class="log-content">
          <div v-if="currentLogContent" class="log-lines">
            <div
              v-for="(line, index) in filteredLogLines"
              :key="index"
              class="log-line"
              :class="getLogLineClass(line)"
            >
              <span class="log-line-number">{{ index + 1 }}</span>
              <span class="log-line-content">{{ line }}</span>
            </div>
          </div>
          <div v-else class="empty-state">
            <div class="empty-icon">
              <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="#64748d" stroke-width="1.5">
                <circle cx="11" cy="11" r="8"/>
                <path d="M21 21l-4.35-4.35"/>
              </svg>
            </div>
            <div class="empty-text">请选择一个日志文件查看</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { invoke } from '@tauri-apps/api/core'

interface LogFile {
  name: string
  date: string
  size: string
  path: string
}

const logFiles = ref<LogFile[]>([])
const selectedLogFiles = ref<string[]>([])
const currentLogContent = ref<string>('')
const searchKeyword = ref('')
const logLevel = ref('')

const filteredLogLines = computed(() => {
  if (!currentLogContent.value) return []

  let lines = currentLogContent.value.split('\n')

  // 按日志级别筛选
  if (logLevel.value) {
    lines = lines.filter(line => line.includes(`[${logLevel.value}]`))
  }

  // 按关键词搜索
  if (searchKeyword.value) {
    lines = lines.filter(line => line.toLowerCase().includes(searchKeyword.value.toLowerCase()))
  }

  return lines
})

const getLogLineClass = (line: string) => {
  if (line.includes('[ERROR]')) return 'log-error'
  if (line.includes('[WARN]')) return 'log-warn'
  if (line.includes('[INFO]')) return 'log-info'
  if (line.includes('[DEBUG]')) return 'log-debug'
  return ''
}

const refreshLogs = async () => {
  try {
    // 调用后端API获取日志文件列表
    const logs = await invoke<LogFile[]>('get_log_files')
    logFiles.value = logs
    ElMessage.success('日志列表已刷新')
  } catch (error) {
    console.error('获取日志列表失败:', error)
    ElMessage.error('获取日志列表失败')
  }
}

const viewLog = async (fileName: string) => {
  try {
    // 调用后端API读取日志文件
    const content = await invoke<string>('read_log_file', { fileName })
    currentLogContent.value = content
    ElMessage.success(`已加载日志: ${fileName}`)
  } catch (error) {
    console.error('读取日志文件失败:', error)
    ElMessage.error('读取日志文件失败')
  }
}

const downloadLog = async (fileName: string) => {
  try {
    // 调用后端API读取日志文件
    const content = await invoke<string>('read_log_file', { fileName })

    const blob = new Blob([content], { type: 'text/plain;charset=utf-8' })
    const url = URL.createObjectURL(blob)
    const link = document.createElement('a')
    link.href = url
    link.download = fileName
    link.click()
    URL.revokeObjectURL(url)
    ElMessage.success(`已下载: ${fileName}`)
  } catch (error) {
    console.error('下载日志失败:', error)
    ElMessage.error('下载日志失败')
  }
}

const deleteLog = async (fileName: string) => {
  try {
    await ElMessageBox.confirm('确定要删除此日志文件吗?', '确认删除', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })

    await invoke('delete_log_file', { fileName })
    ElMessage.success(`已删除: ${fileName}`)
    refreshLogs()

    // 如果当前查看的是被删除的日志，清空显示
    if (currentLogContent.value) {
      currentLogContent.value = ''
    }
  } catch (error) {
    if (error !== 'cancel') {
      console.error('删除日志失败:', error)
      ElMessage.error('删除日志失败')
    }
  }
}

const deleteAllLogs = async () => {
  try {
    await ElMessageBox.confirm('确定要清空所有日志文件吗? 此操作不可恢复!', '确认清空', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })

    await invoke('delete_all_logs')
    ElMessage.success('已清空所有日志')
    logFiles.value = []
    currentLogContent.value = ''
  } catch (error) {
    if (error !== 'cancel') {
      console.error('清空日志失败:', error)
      ElMessage.error('清空日志失败')
    }
  }
}

onMounted(() => {
  refreshLogs()
})
</script>

<style scoped>
.log-analysis-page {
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

.divider {
  height: 1px;
  background: #e5edf5;
  margin: 52px 0;
}

.section {
  margin-bottom: 32px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.section-title {
  font-size: 32px;
  font-weight: 300;
  color: #061b31;
  margin: 0;
  letter-spacing: -0.64px;
}

.toolbar {
  display: flex;
  gap: 12px;
}

.log-files {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 16px;
}

.log-file {
  background: #ffffff;
  border: 1px solid #e5edf5;
  border-radius: 4px;
  padding: 16px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  transition: all 0.2s;
}

.log-file:hover {
  border-color: #b9b9f9;
}

.log-file-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

.log-file-icon {
  font-size: 32px;
}

.log-file-details {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.log-file-name {
  font-size: 16px;
  font-weight: 400;
  color: #061b31;
}

.log-file-meta {
  display: flex;
  gap: 12px;
  font-size: 12px;
  color: #64748d;
}

.log-file-actions {
  display: flex;
  gap: 8px;
}

.log-viewer {
  background: #ffffff;
  border: 1px solid #e5edf5;
  border-radius: 4px;
  padding: 16px;
  min-height: 500px;
}

.log-content {
  font-family: 'Courier New', monospace;
  font-size: 13px;
  line-height: 1.6;
}

.log-lines {
  display: flex;
  flex-direction: column;
}

.log-line {
  display: flex;
  gap: 12px;
  padding: 2px 0;
}

.log-line-number {
  color: #64748d;
  font-size: 12px;
  min-width: 40px;
  text-align: right;
  user-select: none;
}

.log-line-content {
  flex: 1;
  color: #061b31;
  word-break: break-all;
}

.log-error {
  background: #fef2f2;
}

.log-error .log-line-content {
  color: #ef4444;
}

.log-warn {
  background: #fffbeb;
}

.log-warn .log-line-content {
  color: #f59e0b;
}

.log-info {
  background: #f0f9ff;
}

.log-info .log-line-content {
  color: #0ea5e9;
}

.log-debug {
  background: #f8fafd;
}

.log-debug .log-line-content {
  color: #64748d;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 48px;
  color: #64748d;
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 16px;
}

.empty-text {
  font-size: 16px;
  color: #50617a;
}
</style>