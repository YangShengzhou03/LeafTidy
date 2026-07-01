<template>
  <div class="page-container">
    <div class="page-header">
      <h2>附属文件清理</h2>
      <p class="desc">清理缩略图、临时文件等附属文件</p>
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
      <div class="action-bar">
        <el-button type="primary" :disabled="cleanupTypes.length === 0 || workDirs.length === 0" :loading="scanning" @click="startScan">
          开始扫描
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
            <span>{{ getCleanupTypeLabel(result.cleanup_type) }} - {{ result.files.length }} 个文件 - {{ formatSize(result.total_size) }}</span>
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
          <el-button type="danger" @click="cleanSelected">
            清理选中的 {{ selectedFiles.length }} 个文件
          </el-button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, inject, type Ref } from 'vue'
import { ElMessage } from 'element-plus'
import { invoke } from '@tauri-apps/api/core'
import { useFileOps } from '@/composables/useFileOps'
import { useLog, type FileResult } from '@/composables/useLog'
import type { WorkDirectory, CleanupResult, BatchOperationResult } from '@/types'

const workDirs = inject<Ref<WorkDirectory[]>>('workDirs')!

const { formatSize, openFile } = useFileOps()
const { logCleanupResults } = useLog()

const cleanupTypes = ref(['thumbnail', 'temp'])
const scanning = ref(false)
const scanResults = ref<CleanupResult[]>([])
const selectedFiles = ref<string[]>([])

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

async function startScan() {
  if (cleanupTypes.value.length === 0) {
    ElMessage.warning('请选择清理类型')
    return
  }
  if (workDirs.value.length === 0) {
    ElMessage.warning('请先选择待处理目录')
    return
  }

  scanning.value = true
  scanResults.value = []
  selectedFiles.value = []

  try {
    const paths = workDirs.value.map(d => d.path)
    const res = await invoke<CleanupResult[]>('scan_auxiliary_files', {
      paths,
      cleanupTypes: cleanupTypes.value,
    })
    scanResults.value = res

    if (res.length === 0 || totalFiles.value === 0) {
      ElMessage.success('没有发现需要清理的文件')
    } else {
      ElMessage.success(`发现 ${totalFiles.value} 个附属文件`)
    }
  } catch (e: any) {
    ElMessage.error(`扫描失败: ${e}`)
  } finally {
    scanning.value = false
  }
}

async function cleanSelected() {
  if (selectedFiles.value.length === 0) return

  try {
    const res = await invoke<BatchOperationResult>('cleanup_auxiliary_files', {
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
      ElMessage.success(`已清理 ${res.success_count} 个文件`)
      selectedFiles.value = []
      await startScan()
    }
    if (res.fail_count > 0) {
      ElMessage.warning(`${res.fail_count} 个文件清理失败`)
    }
  } catch (e: any) {
    ElMessage.error(`清理失败: ${e}`)
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