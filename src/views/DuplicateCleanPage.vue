<template>
  <div class="page-container">
    <div class="page-header">
      <h2>{{ t('dup.title') }}</h2>
      <p class="desc">{{ t('dup.desc') }}</p>
    </div>
    <div class="page-content">
      <div class="config-panel">
        <div class="config-item">
          <label>{{ t('dup.detectMode') }}</label>
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
          <label>{{ t('dup.handleMode') }}</label>
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
            <span>{{ t('dup.setOutDirFirst') }}</span>
          </div>
        </div>
      </div>

      <!-- 扫描进度 -->
      <div class="progress-panel" v-if="scanning">
        <div class="progress-header">
          <span class="progress-title">{{ t('dup.scanning') }}</span>
          <span class="progress-percent">{{ scanProgress.percentage.toFixed(1) }}%</span>
        </div>
        <el-progress :percentage="scanProgress.percentage" :stroke-width="12" :show-text="false" class="progress-bar" />
        <div class="progress-stats">
          <span>{{ t('dup.total') }} {{ scanProgress.total }}</span>
          <span>{{ t('dup.scanned') }} {{ scanProgress.processed }}</span>
        </div>
        <div class="progress-current" v-if="scanProgress.current_item">
          <span class="current-label">{{ t('dup.current') }}</span>
          <span class="current-file">{{ scanProgress.current_item }}</span>
        </div>
      </div>

      <div class="action-bar">
        <el-button v-if="!scanning" type="primary" :disabled="workDirs.length === 0" @click="startScan">
          {{ t('dup.startScan') }}
        </el-button>
        <el-button v-else type="danger" @click="stopScan">
          {{ t('dup.stopScan') }}
        </el-button>
        <el-button v-if="scanResult && scanResult.duplicate_groups.length > 0 && !scanning"
          @click="selectAllDuplicates">
          {{ t('dup.selectAll') }}
        </el-button>
        <el-button v-if="selectedFiles.length > 0 && !scanning" @click="selectedFiles = []">
          {{ t('dup.clearSelection') }}
        </el-button>
      </div>
      <div class="result-panel" v-if="scanResult">
        <div class="result-header">
          <span class="result-title">{{ t('dup.resultTitle') }}</span>
          <span class="result-stats">
            {{ t('dup.statScanned') }}<strong>{{ scanResult.total_files }}</strong>{{ t('dup.statFound') }}<strong>{{ scanResult.total_duplicates }}</strong>{{ t('dup.statWasted') }}<strong>{{ formatSize(scanResult.wasted_space) }}</strong>
          </span>
        </div>
        <div class="duplicate-groups" v-if="scanResult.duplicate_groups.length > 0">
          <div class="group-item" v-for="(group, index) in scanResult.duplicate_groups" :key="index">
            <div class="group-header">
              <div class="group-info">
                <span class="group-md5" :title="group.md5">MD5: {{ group.md5.slice(0, 16) }}</span>
                <span class="group-size">{{ formatSize(group.size) }}</span>
                <span class="group-count">{{ t('dup.fileCount', { n: group.files.length }) }}</span>
              </div>
              <div class="group-actions">
                <el-button size="small" type="primary" plain @click="selectGroupDuplicates(group)">
                  {{ t('dup.selectDup') }}
                </el-button>
                <el-button size="small" type="danger" @click="confirmCleanGroup(group)">
                  {{ t('dup.cleanDup') }}
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
                <span class="file-tag original-tag" v-if="file.is_original">{{ t('dup.keep') }}</span>
                <el-checkbox v-else v-model="selectedFiles" :value="file.path" />
              </div>
            </div>
          </div>
        </div>
        <div class="empty-result" v-else>
          <el-icon class="empty-icon">
            <CircleCheck />
          </el-icon>
          <span>{{ t('dup.noDuplicates') }}</span>
        </div>
        <div class="batch-action" v-if="selectedFiles.length > 0">
          <span class="selected-info">{{ t('dup.selectedCount', { n: selectedFiles.length }) }}</span>
          <el-button type="danger" @click="confirmCleanSelected">
            {{ t('dup.cleanSelected') }}
          </el-button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, inject, type Ref, onMounted, onUnmounted } from 'vue'
import { ElMessageBox, ElNotification } from 'element-plus'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { Search, ScaleToOriginal, Check, Delete, FolderOpened, Close, Document, Star, CircleCheck, Warning } from '@element-plus/icons-vue'
import { t } from '@/i18n'
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

const detectOptions = computed(() => [
  { value: 'md5', label: t('dup.optMd5'), desc: t('dup.optMd5Desc'), icon: Search },
  { value: 'size', label: t('dup.optSize'), desc: t('dup.optSizeDesc'), icon: ScaleToOriginal },
  { value: 'both', label: t('dup.optBoth'), desc: t('dup.optBothDesc'), icon: Check },
])

const handleOptions = computed(() => [
  { value: 'trash', label: t('dup.optTrash'), icon: Delete },
  { value: 'move', label: t('dup.optMove'), icon: FolderOpened },
  { value: 'delete', label: t('dup.optDelete'), icon: Close, danger: true },
])

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
    ElNotification({ type: 'info', title: t('dup.tip'), message: t('dup.stopping') })
  } catch (e: any) {
    ElNotification({ type: 'error', title: t('dup.error'), message: t('dup.stopFailed', { e }) })
  }
}

async function startScan() {
  if (workDirs.value.length === 0) {
    ElNotification({ type: 'warning', title: t('dup.warning'), message: t('dup.selectDirFirst') })
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
      ElNotification({ type: 'success', title: t('dup.success'), message: t('dup.noDuplicates') })
    } else {
      ElNotification({ type: 'success', title: t('dup.success'), message: t('dup.foundResult', { n: res.total_duplicates, size: formatSize(res.wasted_space) }) })
    }
  } catch (e: any) {
    const errMsg = String(e)
    if (errMsg.includes('取消')) {
      const pathsStr = workDirs.value.map(d => d.path).join(', ')
      await logCancelledOperation('duplicate_clean', pathsStr, '扫描重复文件 - 用户终止')
      ElNotification({ type: 'info', title: t('dup.tip'), message: t('dup.scanCancelled') })
    } else {
      ElNotification({ type: 'error', title: t('dup.error'), message: t('dup.detectFailed', { e: errMsg }) })
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

  const actionText = handleMode.value === 'trash' ? t('dup.optTrash') :
    handleMode.value === 'delete' ? t('dup.optDelete') : t('dup.actionMove')
  const totalSize = formatSize(group.size * duplicates.length)

  try {
    await ElMessageBox.confirm(
      t('dup.confirmGroup', { action: actionText, n: duplicates.length, size: totalSize }),
      t('dup.confirmTitle'),
      {
        confirmButtonText: t('dup.ok'),
        cancelButtonText: t('dup.cancel'),
        type: 'warning',
      }
    )
    await cleanDuplicates(duplicates.map(f => f.path))
  } catch {
    // 用户取消确认框
  }
}

async function confirmCleanSelected() {
  if (selectedFiles.value.length === 0) return

  const actionText = handleMode.value === 'trash' ? t('dup.optTrash') :
    handleMode.value === 'delete' ? t('dup.optDelete') : t('dup.actionMove')

  try {
    await ElMessageBox.confirm(
      t('dup.confirmSelected', { action: actionText, n: selectedFiles.value.length }),
      t('dup.confirmTitle'),
      {
        confirmButtonText: t('dup.ok'),
        cancelButtonText: t('dup.cancel'),
        type: 'warning',
      }
    )
    await cleanDuplicates(selectedFiles.value)
    selectedFiles.value = []
  } catch {
    // 用户取消确认框
  }
}

async function cleanDuplicates(paths: string[]) {
  if (paths.length === 0) return

  if (handleMode.value === 'move') {
    if (!outputDir.value) {
      ElNotification({ type: 'warning', title: t('dup.warning'), message: t('dup.setOutDirFirst') })
      return
    }
    try {
      const res = await invoke<BatchOperationResult>('move_files_batch', {
        sources: paths,
        target: outputDir.value,
      })
      const results: FileResult[] = paths.map((path, index) => ({
        source_path: path,
        target_path: index < res.success_count ? outputDir.value : undefined,
        success: index < res.success_count,
        error: index >= res.success_count ? '移动失败' : undefined,
      }))
      await logDuplicateCleanResults(results)
      if (res.success_count > 0) {
        ElNotification({ type: 'success', title: t('dup.success'), message: t('dup.movedCount', { n: res.success_count }) })
        await startScan()
      }
      if (res.fail_count > 0) {
        ElNotification({ type: 'warning', title: t('dup.warning'), message: t('dup.moveFailCount', { n: res.fail_count }) })
      }
    } catch (e: any) {
      const errMsg = String(e)
      if (errMsg.includes('取消')) {
        await logCancelledOperation('duplicate_clean', paths.join(', '), '移动重复文件 - 用户终止')
        ElNotification({ type: 'info', title: t('dup.tip'), message: t('dup.moveCancelled') })
      } else {
        ElNotification({ type: 'error', title: t('dup.error'), message: t('dup.moveFailed', { e: errMsg }) })
      }
    }
  } else {
    const keepOriginal = handleMode.value === 'trash'
    try {
      const res = await invoke<BatchOperationResult>('clean_duplicates', {
        paths,
        keepOriginal,
      })
      const results: FileResult[] = paths.map((path, index) => ({
        source_path: path,
        success: index < res.success_count,
        error: index >= res.success_count ? '清理失败' : undefined,
      }))
      await logDuplicateCleanResults(results)
      if (res.success_count > 0) {
        ElNotification({ type: 'success', title: t('dup.success'), message: t('dup.cleanedCount', { n: res.success_count }) })
        await startScan()
      }
      if (res.fail_count > 0) {
        ElNotification({ type: 'warning', title: t('dup.warning'), message: t('dup.cleanFailCount', { n: res.fail_count }) })
      }
    } catch (e: any) {
      const errMsg = String(e)
      if (errMsg.includes('取消')) {
        await logCancelledOperation('duplicate_clean', paths.join(', '), '清理重复文件 - 用户终止')
        ElNotification({ type: 'info', title: t('dup.tip'), message: t('dup.cleanCancelled') })
      } else {
        ElNotification({ type: 'error', title: t('dup.error'), message: t('dup.cleanFailed', { e: errMsg }) })
      }
    }
  }
}
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

.config-panel {
  background: var(--panel);
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
  color: var(--text-secondary);
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
  background: var(--panel-2);
  border-radius: 6px;
  border: 1px solid var(--border);
  cursor: pointer;
  transition: all 0.15s;
  user-select: none;
}

.option-chip:hover {
  background: var(--panel-2-hover);
}

.option-chip.active {
  background: rgba(var(--primary-rgb), 0.15);
  border-color: var(--primary);
}

.option-chip .el-icon {
  font-size: 16px;
  color: var(--text-secondary);
}

.option-chip.active .el-icon {
  color: var(--primary);
}

.option-name {
  font-size: 13px;
  color: var(--text-secondary);
}

.option-chip.active .option-name {
  color: var(--primary);
}

.option-desc {
  font-size: 11px;
  color: var(--text-muted);
}

/* 危险选项样式 */
.option-chip.danger:hover {
  background: rgba(var(--danger-rgb), 0.15);
  border-color: rgba(var(--danger-rgb), 0.3);
}

.option-chip.danger:hover .el-icon,
.option-chip.danger:hover .option-name {
  color: var(--danger);
}

.option-chip.danger.active {
  background: rgba(var(--danger-rgb), 0.15);
  border-color: var(--danger);
}

.option-chip.danger.active .el-icon,
.option-chip.danger.active .option-name {
  color: var(--danger);
}

.move-dir-warning {
  margin-top: 12px;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 12px;
  background: rgba(var(--danger-rgb), 0.15);
  border-radius: 6px;
  color: var(--danger);
  font-size: 13px;
}

.move-dir-warning .el-icon {
  font-size: 16px;
}

.progress-panel {
  background: var(--panel);
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
  color: var(--text);
}

.progress-percent {
  font-size: 16px;
  font-weight: 600;
  color: var(--primary);
}

.progress-bar {
  margin-bottom: 12px;
}

.progress-stats {
  display: flex;
  gap: 16px;
  font-size: 12px;
  color: var(--text-muted);
}

.progress-current {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 12px;
  padding: 8px 12px;
  background: var(--panel-2);
  border-radius: 6px;
}

.current-label {
  font-size: 12px;
  color: var(--text-muted);
  flex-shrink: 0;
}

.current-file {
  font-size: 12px;
  color: var(--text);
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
  background: var(--panel);
  border-radius: 8px;
  padding: 20px;
}

.result-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--panel-2);
}

.result-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--text);
}

.result-stats {
  font-size: 12px;
  color: var(--text-muted);
}

.result-stats strong {
  color: var(--primary);
}

/* 重复文件组 */
.duplicate-groups {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.group-item {
  background: var(--panel-2);
  border-radius: 8px;
  overflow: hidden;
}

.group-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: var(--panel-2-hover);
  border-bottom: 1px solid var(--border);
}

.group-info {
  display: flex;
  align-items: center;
  gap: 16px;
}

.group-md5 {
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 12px;
  color: var(--text-muted);
}

.group-size {
  font-size: 12px;
  color: var(--primary);
  font-weight: 500;
}

.group-count {
  font-size: 12px;
  color: var(--text-secondary);
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
  background: rgba(var(--primary-rgb), 0.05);
}

.file-item.original {
  background: rgba(var(--primary-rgb), 0.1);
}

.file-item .el-icon {
  font-size: 16px;
  color: var(--text-muted);
}

.file-item.original .el-icon {
  color: var(--primary);
}

.file-name {
  flex: 1;
  font-size: 13px;
  color: var(--text-secondary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.file-name.clickable {
  cursor: pointer;
}

.file-name.clickable:hover {
  color: var(--primary);
}

.file-modified {
  font-size: 11px;
  color: var(--text-muted);
  min-width: 140px;
}

.file-tag {
  font-size: 11px;
  padding: 2px 8px;
  border-radius: 4px;
  background: var(--border);
  color: var(--text-muted);
}

.file-tag.original-tag {
  background: rgba(var(--primary-rgb), 0.2);
  color: var(--primary);
}

.original-icon {
  color: var(--primary) !important;
}

/* 空结果 */
.empty-result {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px;
  color: var(--text-muted);
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 12px;
  color: var(--primary);
}

/* 批量操作 */
.batch-action {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 20px;
  padding-top: 16px;
  border-top: 1px solid var(--panel-2);
}

.selected-info {
  font-size: 13px;
  color: var(--text-muted);
}
</style>
