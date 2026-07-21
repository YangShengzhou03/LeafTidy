<template>
  <div class="page-container">
    <div class="page-header">
      <h2>重复文件清理</h2>
      <p class="desc">识别并清理重复文件，释放宝贵的磁盘空间</p>
    </div>
    <div class="page-content">
      <div class="config-panel">
        <div class="config-item">
          <label>检测方式</label>
          <div class="option-grid">
            <div v-for="opt in detectOptions" :key="opt.value" class="option-chip"
              :class="{ active: detectMode === opt.value }" @click="detectMode = opt.value">
              <el-icon>
                <component :is="opt.icon" />
              </el-icon>
              <span class="option-name">{{ opt.label }}</span>
              <span class="option-desc">{{ opt.desc }}</span>
            </div>
          </div>
        </div>
        <div class="config-item">
          <label>处理方式</label>
          <div class="option-grid">
            <div v-for="opt in handleOptions" :key="opt.value" class="option-chip"
              :class="{ active: handleMode === opt.value, danger: opt.danger }" @click="handleMode = opt.value">
              <el-icon>
                <component :is="opt.icon" />
              </el-icon>
              <span class="option-name">{{ opt.label }}</span>
            </div>
          </div>
          <div v-if="handleMode === 'move' && !outputDir" class="move-dir-warning">
            <el-icon>
              <Warning />
            </el-icon>
            <span>请先在首页设置输出目录</span>
          </div>
        </div>
      </div>

      <!-- 扫描进度 -->
      <div class="progress-panel" v-if="scanning">
        <div class="progress-header">
          <span class="progress-title">正在扫描重复文件</span>
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

      <div class="action-bar">
        <el-button v-if="!scanning" type="primary" :disabled="workDirs.length === 0" @click="startScan">
          开始检测
        </el-button>
        <el-button v-else type="danger" @click="stopScan">
          终止检测
        </el-button>
        <el-button v-if="scanResult && scanResult.duplicate_groups.length > 0 && !scanning"
          @click="selectAllDuplicates">
          全选重复项
        </el-button>
        <el-button v-if="selectedFiles.length > 0 && !scanning" @click="selectedFiles = []">
          取消选择
        </el-button>
      </div>
      <div class="result-panel" v-if="scanResult">
        <div class="result-header">
          <span class="result-title">检测结果</span>
          <span class="result-stats">
            共扫描 <strong>{{ scanResult.total_files }}</strong> 个文件，
            发现 <strong>{{ scanResult.total_duplicates }}</strong> 个重复，
            浪费空间 <strong>{{ formatSize(scanResult.wasted_space) }}</strong>
          </span>
        </div>
        <div class="duplicate-groups" v-if="scanResult.duplicate_groups.length > 0">
          <div class="group-item" v-for="(group, index) in scanResult.duplicate_groups" :key="index">
            <div class="group-header">
              <div class="group-info">
                <span class="group-md5" :title="group.md5">MD5: {{ group.md5.slice(0, 16) }}</span>
                <span class="group-size">{{ formatSize(group.size) }}</span>
                <span class="group-count">{{ group.files.length }} 个文件</span>
              </div>
              <div class="group-actions">
                <el-button size="small" type="primary" plain @click="selectGroupDuplicates(group)">
                  选择重复
                </el-button>
                <el-button size="small" type="danger" @click="confirmCleanGroup(group)">
                  清理重复
                </el-button>
              </div>
            </div>
            <div class="group-files">
              <div class="file-item" v-for="file in group.files" :key="file.path"
                :class="{ original: file.is_original }">
                <el-icon v-if="file.is_original" class="original-icon">
                  <Star />
                </el-icon>
                <el-icon v-else>
                  <Document />
                </el-icon>
                <span class="file-name clickable" @click="openFile(file.path)" :title="file.path">{{ file.name }}</span>
                <span class="file-modified">{{ file.modified }}</span>
                <span class="file-tag original-tag" v-if="file.is_original">保留</span>
                <el-checkbox v-else v-model="selectedFiles" :value="file.path" />
              </div>
            </div>
          </div>
        </div>
        <div class="empty-result" v-else>
          <el-icon class="empty-icon">
            <CircleCheck />
          </el-icon>
          <span>没有发现重复文件</span>
        </div>
        <div class="batch-action" v-if="selectedFiles.length > 0">
          <span class="selected-info">已选择 {{ selectedFiles.length }} 个文件</span>
          <el-button type="danger" @click="confirmCleanSelected">
            清理选中的文件
          </el-button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, inject, type Ref, onMounted, onUnmounted } from 'vue'
import { ElMessage, ElMessageBox, ElNotification } from 'element-plus'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { Search, ScaleToOriginal, Check, Delete, FolderOpened, Close, Document, Star, CircleCheck, Warning } from '@element-plus/icons-vue'
import { useFileOps } from '@/composables/useFileOps'
import { useLog, type FileResult } from '@/composables/useLog'
import type { WorkDirectory, DuplicateScanResult, DuplicateGroup, BatchOperationResult, TaskProgress, CancelResult } from '@/types'

const workDirs = inject<Ref<WorkDirectory[]>>('workDirs')!
const outputDir = inject<Ref<string>>('outputDir')!

const { formatSize, openFile } = useFileOps()
const { logDuplicateCleanResults, logCancelledOperation } = useLog()

const detectMode = ref('both')
const handleMode = ref('trash')
const scanning = ref(false)
const scanResult = ref<DuplicateScanResult | null>(null)
const selectedFiles = ref<string[]>([])
const scanProgress = ref<TaskProgress>({
  total: 0,
  processed: 0,
  current_item: undefined,
  percentage: 0
})

let unlistenProgress: UnlistenFn | null = null

const detectOptions = [
  { value: 'md5', label: 'MD5校验', desc: '精确匹配', icon: Search },
  { value: 'size', label: '文件大小', desc: '快速检测', icon: ScaleToOriginal },
  { value: 'both', label: '双重校验', desc: '最准确', icon: Check },
]

const handleOptions = [
  { value: 'trash', label: '移至回收站', icon: Delete },
  { value: 'move', label: '移动到目录', icon: FolderOpened },
  { value: 'delete', label: '彻底删除', icon: Close, danger: true },
]

onMounted(async () => {
  unlistenProgress = await listen<TaskProgress>('duplicate-progress', (event) => {
    scanProgress.value = event.payload
  })
})

onUnmounted(() => {
  if (unlistenProgress) {
    unlistenProgress()
  }
})

async function stopScan() {
  try {
    await invoke<CancelResult>('cancel_operation')
    ElNotification({ type: 'info', title: '提示', message: '正在终止扫描...' })
  } catch (e: any) {
    ElNotification({ type: 'error', title: '错误', message: `终止失败: ${e}` })
  }
}

async function startScan() {
  if (workDirs.value.length === 0) {
    ElNotification({ type: 'warning', title: '警告', message: '请先选择待处理目录' })
    return
  }

  scanning.value = true
  scanResult.value = null
  selectedFiles.value = []
  scanProgress.value = {
    total: 0,
    processed: 0,
    current_item: undefined,
    percentage: 0
  }

  try {
    const paths = workDirs.value.map(d => d.path)
    const res = await invoke<DuplicateScanResult>('find_duplicates_async', {
      paths,
      detectMode: detectMode.value,
    })
    scanResult.value = res

    if (res.duplicate_groups.length === 0) {
      ElNotification({ type: 'success', title: '成功', message: '没有发现重复文件' })
    } else {
      ElNotification({ type: 'success', title: '成功', message: `发现 ${res.total_duplicates} 个重复文件，浪费空间 ${formatSize(res.wasted_space)}` })
    }
  } catch (e: any) {
    const errMsg = String(e)
    if (errMsg.includes('取消')) {
      const pathsStr = workDirs.value.map(d => d.path).join(', ')
      await logCancelledOperation('duplicate_clean', pathsStr, '扫描重复文件 - 用户终止')
      ElNotification({ type: 'info', title: '提示', message: '扫描已取消' })
    } else {
      ElNotification({ type: 'error', title: '错误', message: `检测失败: ${errMsg}` })
    }
  } finally {
    scanning.value = false
  }
}

function selectGroupDuplicates(group: DuplicateGroup) {
  const duplicates = group.files.filter(f => !f.is_original).map(f => f.path)
  for (const path of duplicates) {
    if (!selectedFiles.value.includes(path)) {
      selectedFiles.value.push(path)
    }
  }
}

function selectAllDuplicates() {
  if (!scanResult.value) return
  selectedFiles.value = []
  for (const group of scanResult.value.duplicate_groups) {
    for (const file of group.files) {
      if (!file.is_original && !selectedFiles.value.includes(file.path)) {
        selectedFiles.value.push(file.path)
      }
    }
  }
}

async function confirmCleanGroup(group: DuplicateGroup) {
  const duplicates = group.files.filter(f => !f.is_original)
  if (duplicates.length === 0) return

  const actionText = handleMode.value === 'trash' ? '移至回收站' :
    handleMode.value === 'delete' ? '彻底删除' : '移动'
  const totalSize = formatSize(group.size * duplicates.length)

  try {
    await ElMessageBox.confirm(
      `确定要${actionText} ${duplicates.length} 个重复文件吗？共 ${totalSize}`,
      '确认清理',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      }
    )
    await cleanDuplicates(duplicates.map(f => f.path))
  } catch {
    // 用户取消
  }
}

async function confirmCleanSelected() {
  if (selectedFiles.value.length === 0) return

  const actionText = handleMode.value === 'trash' ? '移至回收站' :
    handleMode.value === 'delete' ? '彻底删除' : '移动'

  try {
    await ElMessageBox.confirm(
      `确定要${actionText}选中的 ${selectedFiles.value.length} 个文件吗？`,
      '确认清理',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      }
    )
    await cleanDuplicates(selectedFiles.value)
    selectedFiles.value = []
  } catch {
    // 用户取消
  }
}

async function cleanDuplicates(paths: string[]) {
  if (paths.length === 0) return

  if (handleMode.value === 'move') {
    if (!outputDir.value) {
      ElNotification({ type: 'warning', title: '警告', message: '请先在首页设置输出目录' })
      return
    }
    // 移动模式
    try {
      const res = await invoke<BatchOperationResult>('move_files_batch', {
        sources: paths,
        target: outputDir.value,
      })
      // 写入日志
      const results: FileResult[] = paths.map((path, index) => ({
        source_path: path,
        target_path: index < res.success_count ? outputDir.value : undefined,
        success: index < res.success_count,
        error: index >= res.success_count ? '移动失败' : undefined,
      }))
      await logDuplicateCleanResults(results)
      if (res.success_count > 0) {
        ElNotification({ type: 'success', title: '成功', message: `已移动 ${res.success_count} 个文件` })
        await startScan()
      }
      if (res.fail_count > 0) {
        ElNotification({ type: 'warning', title: '警告', message: `${res.fail_count} 个文件移动失败` })
      }
    } catch (e: any) {
      const errMsg = String(e)
      if (errMsg.includes('取消')) {
        await logCancelledOperation('duplicate_clean', paths.join(', '), '移动重复文件 - 用户终止')
        ElNotification({ type: 'info', title: '提示', message: '移动操作已取消' })
      } else {
        ElNotification({ type: 'error', title: '错误', message: `移动失败: ${errMsg}` })
      }
    }
  } else {
    // 回收站或删除模式
    const keepOriginal = handleMode.value === 'trash'
    try {
      const res = await invoke<BatchOperationResult>('clean_duplicates', {
        paths,
        keepOriginal,
      })
      // 写入日志
      const results: FileResult[] = paths.map((path, index) => ({
        source_path: path,
        success: index < res.success_count,
        error: index >= res.success_count ? '清理失败' : undefined,
      }))
      await logDuplicateCleanResults(results)
      if (res.success_count > 0) {
        ElNotification({ type: 'success', title: '成功', message: `已清理 ${res.success_count} 个文件` })
        await startScan()
      }
      if (res.fail_count > 0) {
        ElNotification({ type: 'warning', title: '警告', message: `${res.fail_count} 个文件清理失败` })
      }
    } catch (e: any) {
      const errMsg = String(e)
      if (errMsg.includes('取消')) {
        await logCancelledOperation('duplicate_clean', paths.join(', '), '清理重复文件 - 用户终止')
        ElNotification({ type: 'info', title: '提示', message: '清理操作已取消' })
      } else {
        ElNotification({ type: 'error', title: '错误', message: `清理失败: ${errMsg}` })
      }
    }
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
  margin-bottom: 12px;
  font-weight: 500;
}

.option-grid {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.option-chip {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  background: #2A2B30;
  border-radius: 6px;
  border: 1px solid #3A3B40;
  cursor: pointer;
  transition: all 0.15s;
  user-select: none;
}

.option-chip:hover {
  background: #353639;
}

.option-chip.active {
  background: rgba(58, 134, 255, 0.15);
  border-color: #3A86FF;
}

.option-chip .el-icon {
  font-size: 16px;
  color: #C8D0DC;
}

.option-chip.active .el-icon {
  color: #3A86FF;
}

.option-name {
  font-size: 13px;
  color: #C8D0DC;
}

.option-chip.active .option-name {
  color: #3A86FF;
}

.option-desc {
  font-size: 11px;
  color: #8A94A6;
}

/* 危险选项样式 */
.option-chip.danger:hover {
  background: rgba(232, 17, 35, 0.15);
  border-color: rgba(232, 17, 35, 0.3);
}

.option-chip.danger:hover .el-icon,
.option-chip.danger:hover .option-name {
  color: #E81123;
}

.option-chip.danger.active {
  background: rgba(232, 17, 35, 0.15);
  border-color: #E81123;
}

.option-chip.danger.active .el-icon,
.option-chip.danger.active .option-name {
  color: #E81123;
}

.move-dir-warning {
  margin-top: 12px;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 12px;
  background: rgba(232, 17, 35, 0.15);
  border-radius: 6px;
  color: #E81123;
  font-size: 13px;
}

.move-dir-warning .el-icon {
  font-size: 16px;
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
  gap: 12px;
  margin-bottom: 20px;
}

/* 结果面板 */
.result-panel {
  background: #1F2023;
  border-radius: 8px;
  padding: 20px;
}

.result-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 1px solid #2A2B30;
}

.result-title {
  font-size: 14px;
  font-weight: 500;
  color: #E0E6ED;
}

.result-stats {
  font-size: 12px;
  color: #8A94A6;
}

.result-stats strong {
  color: #3A86FF;
}

/* 重复文件组 */
.duplicate-groups {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.group-item {
  background: #2A2B30;
  border-radius: 8px;
  overflow: hidden;
}

.group-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: #353639;
  border-bottom: 1px solid #3A3B40;
}

.group-info {
  display: flex;
  align-items: center;
  gap: 16px;
}

.group-md5 {
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 12px;
  color: #8A94A6;
}

.group-size {
  font-size: 12px;
  color: #3A86FF;
  font-weight: 500;
}

.group-count {
  font-size: 12px;
  color: #C8D0DC;
}

.group-actions {
  display: flex;
  gap: 8px;
}

.group-files {
  padding: 8px 0;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  transition: background 0.15s;
}

.file-item:hover {
  background: rgba(58, 134, 255, 0.05);
}

.file-item.original {
  background: rgba(58, 134, 255, 0.1);
}

.file-item .el-icon {
  font-size: 16px;
  color: #8A94A6;
}

.file-item.original .el-icon {
  color: #3A86FF;
}

.file-name {
  flex: 1;
  font-size: 13px;
  color: #C8D0DC;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.file-name.clickable {
  cursor: pointer;
}

.file-name.clickable:hover {
  color: #3A86FF;
}

.file-modified {
  font-size: 11px;
  color: #8A94A6;
  min-width: 140px;
}

.file-tag {
  font-size: 11px;
  padding: 2px 8px;
  border-radius: 4px;
  background: #3A3B40;
  color: #8A94A6;
}

.file-tag.original-tag {
  background: rgba(58, 134, 255, 0.2);
  color: #3A86FF;
}

.original-icon {
  color: #3A86FF !important;
}

/* 空结果 */
.empty-result {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px;
  color: #8A94A6;
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 12px;
  color: #3A86FF;
}

/* 批量操作 */
.batch-action {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 20px;
  padding-top: 16px;
  border-top: 1px solid #2A2B30;
}

.selected-info {
  font-size: 13px;
  color: #8A94A6;
}
</style>
