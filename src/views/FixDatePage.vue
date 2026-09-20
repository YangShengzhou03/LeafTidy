<template>
  <div class="page-container">
    <div class="page-header">
      <h2>{{ t('fixDate.title') }}</h2>
      <p class="desc">{{ t('fixDate.desc') }}</p>
    </div>
    <div class="page-content">
      <div class="config-panel">
        <div class="config-item">
          <label>{{ t('fixDate.sourceLabel') }}</label>
          <el-select v-model="dateSource" :placeholder="t('fixDate.sourcePlaceholder')" style="width: 220px">
            <el-option :label="t('fixDate.sourceFilename')" value="filename" />
            <el-option :label="t('fixDate.sourceSpecified')" value="specified" />
          </el-select>
          <span class="hint" v-if="dateSource === 'filename'">{{ t('fixDate.hintFilename') }}</span>
          <span class="hint" v-else>{{ t('fixDate.hintSpecified') }}</span>
        </div>
        <div class="config-item" v-if="dateSource === 'specified'">
          <label>{{ t('fixDate.timeLabel') }}</label>
          <el-date-picker v-model="specifiedTime" type="datetime" :placeholder="t('fixDate.timePlaceholder')"
            format="YYYY-MM-DD HH:mm:ss" value-format="YYYY:MM:DD HH:mm:ss" style="width: 220px" />
        </div>
      </div>

      <div class="progress-panel" v-if="processing">
        <div class="progress-header">
          <span class="progress-title">{{ t('fixDate.progressTitle') }}</span>
          <span class="progress-percent">{{ progress.percentage.toFixed(1) }}%</span>
        </div>
        <el-progress :percentage="progress.percentage" :stroke-width="12" :show-text="false" class="progress-bar" />
        <div class="progress-stats">
          <span>{{ t('photos.common.total', { n: progress.total }) }}</span>
          <span>{{ t('photos.common.processed', { n: progress.processed }) }}</span>
        </div>
        <div class="progress-current" v-if="progress.current_item">
          <span class="current-label">{{ t('photos.common.current') }}</span>
          <span class="current-file">{{ progress.current_item }}</span>
        </div>
      </div>

      <div class="action-bar">
        <span v-if="workDirs.length === 0 && !processing" class="action-hint">{{ t('photos.common.selectDirsHint') }}</span>
        <el-button v-if="!processing" type="primary" :disabled="workDirs.length === 0" @click="startFix">
          {{ t('fixDate.start') }}
        </el-button>
        <el-button v-else type="danger" @click="stopFix">
          {{ t('fixDate.stop') }}
        </el-button>
      </div>

      <div class="result-panel" v-if="results.length > 0">
        <div class="result-header">
          <span class="result-title">{{ t('fixDate.resultTitle') }}</span>
          <span class="result-stats">
            {{ t('photos.common.resultStats', { s: successCount, f: failCount }) }}
          </span>
        </div>
        <div class="result-section" v-if="failResults.length > 0">
          <div class="section-label fail-label">
            <el-icon>
              <CircleCloseFilled />
            </el-icon>
            <span>{{ t('photos.common.failedCount', { n: failCount }) }}</span>
          </div>
          <div class="result-list">
            <div v-for="(result, index) in failResults" :key="'fail-' + index" class="result-item fail">
              <el-icon>
                <CircleCloseFilled />
              </el-icon>
              <span class="result-name" :title="result.source_path">{{ getFileName(result.source_path) }}</span>
              <span class="result-error-inline">{{ result.error }}</span>
            </div>
          </div>
        </div>
        <div class="result-section" v-if="successResults.length > 0">
          <div class="section-label success-label">
            <el-icon>
              <SuccessFilled />
            </el-icon>
            <span>{{ t('photos.common.successCount', { n: successCount }) }}</span>
          </div>
          <div class="result-list">
            <div v-for="(result, index) in successResults" :key="'success-' + index" class="result-item success">
              <el-icon>
                <SuccessFilled />
              </el-icon>
              <span class="result-name clickable" @click="openFile(result.source_path)" :title="result.source_path">{{
                getFileName(result.source_path) }}</span>
              <span class="result-arrow">→</span>
              <span class="result-target clickable" @click="openFile(result.target_path)" :title="result.target_path">{{
                getFileName(result.target_path) }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, inject, type Ref, onMounted, onUnmounted } from 'vue'
import { ElNotification } from 'element-plus'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { SuccessFilled, CircleCloseFilled } from '@element-plus/icons-vue'
import { t } from '@/i18n'
import { useFileOps } from '@/composables/useFileOps'
import { useLog } from '@/composables/useLog'
import type { WorkDirectory, ImageProcessResult, TaskProgress, CancelResult } from '@/types'

const workDirs = inject<Ref<WorkDirectory[]>>('workDirs')!
const outputDir = inject<Ref<string>>('outputDir')!
const isProcessing = inject<Ref<boolean>>('isProcessing')!

const { getFileName, openFile } = useFileOps()
const { logImageProcessResults, logCancelledOperation } = useLog()

const dateSource = ref('filename')
const specifiedTime = ref('')
const processing = ref(false)
const results = ref<ImageProcessResult[]>([])
const progress = ref<TaskProgress>({
  total: 0,
  processed: 0,
  current_item: undefined,
  percentage: 0
})

let unlistenProgress: UnlistenFn | null = null

onMounted(async () => {
  unlistenProgress = await listen<TaskProgress>('image-process-progress', (event) => {
    progress.value = event.payload
  })
})

onUnmounted(() => {
  if (unlistenProgress) {
    unlistenProgress()
  }
})

const successCount = computed(() => results.value.filter(r => r.success).length)
const failCount = computed(() => results.value.filter(r => !r.success).length)
const successResults = computed(() => results.value.filter(r => r.success))
const failResults = computed(() => results.value.filter(r => !r.success))

function collectImagePaths(files: { path: string }[]): string[] {
  return files.filter(f => /\.(jpg|jpeg)$/i.test(f.path)).map(f => f.path)
}

async function stopFix() {
  try {
    await invoke<CancelResult>('cancel_operation')
    ElNotification({ type: 'info', title: t('photos.common.tip'), message: t('fixDate.stopInProgress') })
  } catch (e: any) {
    ElNotification({ type: 'error', title: t('photos.common.error'), message: t('photos.common.stopFailed', { e }) })
  }
}

async function startFix() {
  if (workDirs.value.length === 0) {
    ElNotification({ type: 'warning', title: t('photos.common.warning'), message: t('photos.common.selectDirs') })
    return
  }
  if (!outputDir.value) {
    ElNotification({ type: 'warning', title: t('photos.common.warning'), message: t('photos.common.selectOutput') })
    return
  }
  if (dateSource.value === 'specified' && !specifiedTime.value) {
    ElNotification({ type: 'warning', title: t('photos.common.warning'), message: t('fixDate.selectTime') })
    return
  }

  processing.value = true
  isProcessing.value = true
  results.value = []
  progress.value = {
    total: 0,
    processed: 0,
    current_item: undefined,
    percentage: 0
  }

  let paths: string[] = []
  try {
    for (const dir of workDirs.value) {
      const files = await invoke<{ path: string }[]>('scan_directory', { path: dir.path })
      paths.push(...collectImagePaths(files))
    }

    if (paths.length === 0) {
      ElNotification({ type: 'info', title: t('photos.common.tip'), message: t('photos.common.noJpg') })
      return
    }

    const res = await invoke<ImageProcessResult[]>('fix_date_taken_async', {
      paths,
      targetDir: outputDir.value,
      dateSource: dateSource.value,
      specifiedTime: dateSource.value === 'specified' ? specifiedTime.value : null
    })
    results.value = res
    await logImageProcessResults('fix_date', res.map(r => ({ ...r })), {
      params: {
        时间来源: dateSource.value === 'specified' ? `指定时间 ${specifiedTime.value}` : '从文件名识别',
        处理方式: '复制到输出目录',
      },
      targetDir: outputDir.value,
    })
    const success = res.filter(r => r.success).length
    const fail = res.filter(r => !r.success).length
    if (fail === 0) {
      ElNotification({ type: 'success', title: t('photos.common.success'), message: t('fixDate.doneAll', { n: success }) })
    } else {
      ElNotification({ type: 'warning', title: t('photos.common.warning'), message: t('fixDate.donePartial', { s: success, f: fail }) })
    }
  } catch (e: any) {
    const errMsg = String(e)
    if (errMsg.includes('取消')) {
      await logCancelledOperation('fix_date', paths.join(', '), `源文件数: ${paths.length}`)
      ElNotification({ type: 'info', title: t('photos.common.tip'), message: t('fixDate.cancelled') })
    } else {
      ElNotification({ type: 'error', title: t('photos.common.error'), message: t('fixDate.failed', { e: errMsg }) })
    }
  } finally {
    processing.value = false
    isProcessing.value = false
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

.page-content {
  max-width: 900px;
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

.config-item .hint {
  font-size: 12px;
  color: var(--text-muted);
  margin-left: 12px;
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
  align-items: center;
  justify-content: flex-end;
  gap: 12px;
  margin-bottom: 20px;
}

.action-hint {
  font-size: 13px;
  color: var(--text-muted);
  margin-right: auto;
}

.result-panel {
  background: var(--panel);
  border-radius: 8px;
  padding: 16px;
}

.result-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.result-section {
  margin-bottom: 16px;
}

.result-section:last-child {
  margin-bottom: 0;
}

.section-label {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-bottom: 8px;
  padding: 6px 10px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
}

.section-label .el-icon {
  font-size: 14px;
}

.fail-label {
  background: rgba(var(--danger-rgb), 0.15);
  color: var(--danger);
}

.success-label {
  background: rgba(var(--success-rgb), 0.15);
  color: var(--success);
}

.result-title {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
}

.result-stats {
  font-size: 12px;
  color: var(--text-muted);
}

.result-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
  max-height: 200px;
  overflow-y: auto;
}

.result-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  background: var(--panel-2);
  border-radius: 6px;
}

.result-item .el-icon {
  font-size: 14px;
  flex-shrink: 0;
}

.result-item.success .el-icon {
  color: var(--success);
}

.result-item.fail .el-icon {
  color: var(--danger);
}

.result-name {
  font-size: 12px;
  color: var(--text);
  max-width: 260px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex-shrink: 0;
}

.result-name.clickable,
.result-target.clickable {
  cursor: pointer;
  transition: color 0.15s;
}

.result-name.clickable:hover {
  color: var(--primary);
}

.result-arrow {
  font-size: 12px;
  color: var(--text-muted);
  flex-shrink: 0;
}

.result-target {
  font-size: 12px;
  color: var(--primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
}

.result-target.clickable:hover {
  opacity: 0.7;
}

.result-error-inline {
  font-size: 12px;
  color: var(--danger);
  flex-shrink: 0;
  margin-left: auto;
}
</style>
