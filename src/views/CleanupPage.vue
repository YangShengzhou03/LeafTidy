<template>
  <div class="page-container">
    <div class="page-header">
      <h2>附属文件清理</h2>
      <p class="desc">清理系统生成的缩略图、临时文件等垃圾文件</p>
    </div>
    <div class="page-content">
      <div class="config-panel">
        <div class="config-item">
          <label>清理类型</label>
          <el-checkbox-group v-model="cleanupTypes">
            <el-checkbox value="thumbnail">缩略图缓存</el-checkbox>
            <el-checkbox value="temp">临时文件</el-checkbox>
            <el-checkbox value="ds_store">.DS_Store</el-checkbox>
            <el-checkbox value="thumbs">Thumbs.db</el-checkbox>
            <el-checkbox value="desktop.ini">desktop.ini</el-checkbox>
          </el-checkbox-group>
        </div>
      </div>

      <!-- 扫描进度 -->
      <div class="progress-panel" v-if="scanning">
        <div class="progress-header">
          <span class="progress-title">正在扫描文件</span>
          <span class="progress-percent">{{ scanProgress.percentage.toFixed(1) }}%</span>
        </div>
        <el-progress :percentage="scanProgress.percentage" :stroke-width="12" :show-text="false" class="progress-bar" />
        <div class="progress-stats">
          <span>总数: {{ scanProgress.total }}</span>
          <span>已扫描: {{ scanProgress.processed }}</span>
        </div>
        <div class="progress-current" v-if="scanProgress.current_item">
          <span class="current-label">当前:</span>
          <span class="current-file">{{ scanProgress.current_item }}</span>
        </div>
      </div>

      <!-- 清理进度 -->
      <div class="progress-panel" v-if="cleaning">
        <div class="progress-header">
          <span class="progress-title">正在清理文件</span>
          <span class="progress-percent">{{ cleanProgress.percentage.toFixed(1) }}%</span>
        </div>
        <el-progress :percentage="cleanProgress.percentage" :stroke-width="12" :show-text="false"
          class="progress-bar" />
        <div class="progress-stats">
          <span>总数: {{ cleanProgress.total }}</span>
          <span>已清理: {{ cleanProgress.processed }}</span>
        </div>
      </div>

      <div class="action-bar">
        <el-button v-if="!scanning && !cleaning" type="primary"
          :disabled="cleanupTypes.length === 0 || workDirs.length === 0" @click="startScan">
          开始扫描
        </el-button>
        <el-button v-if="scanning || cleaning" type="danger" @click="stopOperation">
          {{ scanning ? '终止扫描' : '终止清理' }}
        </el-button>
      </div>
      <div class="result-panel" v-if="scanResults.length > 0">
        <div class="result-header">
          <span class="result-title">扫描结果</span>
          <span class="result-stats">
            共发现 {{ totalFiles }} 个文件，占用 {{ formatSize(totalSize) }}
          </span>
        </div>
        <div class="result-section" v-for="result in scanResults" :key="result.cleanup_type">
          <div class="section-label">
            <span>{{ getCleanupTypeLabel(result.cleanup_type) }} - {{ result.files.length }} 个文件 - {{
              formatSize(result.total_size) }}</span>
          </div>
          <div class="result-list">
            <div class="file-item" v-for="file in result.files" :key="file.path">
              <el-checkbox v-model="selectedFiles" :value="file.path" />
              <span class="file-name clickable" @click="openFile(file.path)" :title="file.path">{{ file.name }}</span>
              <span class="file-size">{{ formatSize(file.size) }}</span>
            </div>
          </div>
        </div>
        <div class="batch-action" v-if="selectedFiles.length > 0">
          <el-button type="danger" :disabled="cleaning" @click="cleanSelected">
            清理选中的 {{ selectedFiles.length }} 个文件
          </el-button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, inject, type Ref, onMounted, onUnmounted } from 'vue'
import { ElMessage, ElNotification } from 'element-plus'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useFileOps } from '@/composables/useFileOps'
import { useLog, type FileResult } from '@/composables/useLog'
import type { WorkDirectory, CleanupResult, BatchOperationResult, TaskProgress, CancelResult } from '@/types'

const workDirs = inject<Ref<WorkDirectory[]>>('workDirs')!

const { formatSize, openFile } = useFileOps()
const { logCleanupResults, logCancelledOperation } = useLog()

const cleanupTypes = ref(['thumbnail', 'temp'])
const scanning = ref(false)
const cleaning = ref(false)
const scanResults = ref<CleanupResult[]>([])
const selectedFiles = ref<string[]>([])
const scanProgress = ref<TaskProgress>({
  total: 0,
  processed: 0,
  current_item: undefined,
  percentage: 0
})
const cleanProgress = ref<TaskProgress>({
  total: 0,
  processed: 0,
  current_item: undefined,
  percentage: 0
})

let unlistenScanProgress: UnlistenFn | null = null
let unlistenCleanProgress: UnlistenFn | null = null

const cleanupTypeLabels: Record<string, string> = {
  thumbnail: '缩略图缓存',
  temp: '临时文件',
  ds_store: '.DS_Store',
  thumbs: 'Thumbs.db',
  'desktop.ini': 'desktop.ini',
}

function getCleanupTypeLabel(type: string): string {
  return cleanupTypeLabels[type] || type
}

const totalFiles = computed(() => {
  return scanResults.value.reduce((sum, r) => sum + r.files.length, 0)
})

const totalSize = computed(() => {
  return scanResults.value.reduce((sum, r) => sum + r.total_size, 0)
})

onMounted(async () => {
  unlistenScanProgress = await listen<TaskProgress>('cleanup-scan-progress', (event) => {
    scanProgress.value = event.payload
  })
  unlistenCleanProgress = await listen<TaskProgress>('cleanup-clean-progress', (event) => {
    cleanProgress.value = event.payload
  })
})

onUnmounted(() => {
  if (unlistenScanProgress) {
    unlistenScanProgress()
  }
  if (unlistenCleanProgress) {
    unlistenCleanProgress()
  }
})

async function stopOperation() {
  try {
    await invoke<CancelResult>('cancel_operation')
    ElNotification({ type: 'info', title: '提示', message: '正在终止操作...' })
  } catch (e: any) {
    ElNotification({ type: 'error', title: '错误', message: `终止失败: ${e}` })
  }
}

async function startScan() {
  if (cleanupTypes.value.length === 0) {
    ElNotification({ type: 'warning', title: '警告', message: '请选择清理类型' })
    return
  }
  if (workDirs.value.length === 0) {
    ElNotification({ type: 'warning', title: '警告', message: '请先选择待处理目录' })
    return
  }

  scanning.value = true
  scanResults.value = []
  selectedFiles.value = []
  scanProgress.value = {
    total: 0,
    processed: 0,
    current_item: undefined,
    percentage: 0
  }

  try {
    const paths = workDirs.value.map(d => d.path)
    const res = await invoke<CleanupResult[]>('scan_auxiliary_files_async', {
      paths,
      cleanupTypes: cleanupTypes.value,
    })
    scanResults.value = res

    if (res.length === 0 || totalFiles.value === 0) {
      ElNotification({ type: 'success', title: '成功', message: '没有发现需要清理的文件' })
    } else {
      ElNotification({ type: 'success', title: '成功', message: `发现 ${totalFiles.value} 个附属文件` })
    }
  } catch (e: any) {
    const errMsg = String(e)
    if (errMsg.includes('取消')) {
      const pathsStr = workDirs.value.map(d => d.path).join(', ')
      await logCancelledOperation('cleanup', pathsStr, '扫描附属文件 - 用户终止')
      ElNotification({ type: 'info', title: '提示', message: '扫描操作已取消' })
    } else {
      ElNotification({ type: 'error', title: '错误', message: `扫描失败: ${errMsg}` })
    }
  } finally {
    scanning.value = false
  }
}

async function cleanSelected() {
  if (selectedFiles.value.length === 0) return

  cleaning.value = true
  cleanProgress.value = {
    total: selectedFiles.value.length,
    processed: 0,
    current_item: undefined,
    percentage: 0
  }

  try {
    const res = await invoke<BatchOperationResult>('cleanup_auxiliary_files_async', {
      files: selectedFiles.value,
    })

    // 写入日志
    const results: FileResult[] = selectedFiles.value.map((path, index) => ({
      source_path: path,
      success: index < res.success_count,
      error: index >= res.success_count ? '清理失败' : undefined,
    }))
    await logCleanupResults(results)

    if (res.success_count > 0) {
      ElNotification({ type: 'success', title: '成功', message: `已清理 ${res.success_count} 个文件` })
      selectedFiles.value = []
      await startScan()
    }
    if (res.fail_count > 0) {
      ElNotification({ type: 'warning', title: '警告', message: `${res.fail_count} 个文件清理失败` })
    }
  } catch (e: any) {
    const errMsg = String(e)
    if (errMsg.includes('取消')) {
      await logCancelledOperation('cleanup', selectedFiles.value.join(', '), '清理附属文件 - 用户终止')
      ElNotification({ type: 'info', title: '提示', message: '清理操作已取消' })
    } else {
      ElNotification({ type: 'error', title: '错误', message: `清理失败: ${errMsg}` })
    }
  } finally {
    cleaning.value = false
  }
}
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

.config-panel {
  background: #1F2023;
  border-radius: 8px;
  padding: 20px;
  margin-bottom: 20px;
}

.config-item {
  margin-bottom: 20px;
}

.config-item:last-child {
  margin-bottom: 0;
}

.config-item label {
  display: block;
  font-size: 13px;
  color: #C8D0DC;
  margin-bottom: 8px;
  font-weight: 500;
}

.config-item .el-checkbox-group {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.progress-panel {
  background: #1F2023;
  border-radius: 8px;
  padding: 20px;
  margin-bottom: 20px;
}

.progress-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.progress-title {
  font-size: 14px;
  font-weight: 500;
  color: #E0E6ED;
}

.progress-percent {
  font-size: 16px;
  font-weight: 600;
  color: #3A86FF;
}

.progress-bar {
  margin-bottom: 12px;
}

.progress-stats {
  display: flex;
  gap: 16px;
  font-size: 12px;
  color: #8A94A6;
}

.progress-current {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 12px;
  padding: 8px 12px;
  background: #2A2B30;
  border-radius: 6px;
}

.current-label {
  font-size: 12px;
  color: #8A94A6;
  flex-shrink: 0;
}

.current-file {
  font-size: 12px;
  color: #E0E6ED;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.action-bar {
  display: flex;
  justify-content: flex-end;
  margin-bottom: 20px;
}

.result-panel {
  background: #1F2023;
  border-radius: 8px;
  padding: 16px;
}

.result-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.result-title {
  font-size: 13px;
  font-weight: 500;
  color: #C8D0DC;
}

.result-stats {
  font-size: 12px;
  color: #8A94A6;
}

.result-section {
  margin-bottom: 16px;
}

.result-section:last-child {
  margin-bottom: 0;
}

.section-label {
  font-size: 12px;
  color: #8A94A6;
  margin-bottom: 8px;
}

.result-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
  max-height: 200px;
  overflow-y: auto;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  background: #2A2B30;
  border-radius: 6px;
}

.file-name {
  flex: 1;
  font-size: 12px;
  color: #E0E6ED;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.file-name.clickable {
  cursor: pointer;
  transition: color 0.15s;
}

.file-name.clickable:hover {
  color: #3A86FF;
}

.file-size {
  font-size: 12px;
  color: #8A94A6;
}

.batch-action {
  display: flex;
  justify-content: flex-end;
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px solid #2A2B30;
}
</style>
