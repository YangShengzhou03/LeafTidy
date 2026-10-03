<template>
  <div class="page-container">
    <div class="page-header">
      <h2>{{ t('organize.title') }}</h2>
      <p class="desc">{{ t('organize.desc') }}</p>
    </div>
    <div class="page-content">
      <div class="config-panel">
        <div class="config-item">
          <label>{{ t('organize.timeSourceLabel') }}</label>
          <el-select v-model="timeSource" :placeholder="t('organize.timeSourcePlaceholder')" style="width: 150px">
            <el-option :label="t('organize.timeSource.modified')" value="modified" />
            <el-option :label="t('organize.timeSource.created')" value="created" />
            <el-option :label="t('organize.timeSource.taken')" value="taken" />
          </el-select>
          <span class="hint">{{ t('organize.timeSourceHint') }}</span>
        </div>
        <div class="config-item">
          <label>{{ t('organize.tagsLabel') }}</label>
          <div class="tag-selector">
            <div class="tag-grid">
              <div v-for="tag in availableTags" :key="tag.value" class="tag-chip" @click="addTag(tag.value)">
                <el-icon>
                  <component :is="tag.icon" />
                </el-icon>
                <span>{{ tag.label }}</span>
              </div>
            </div>
          </div>
        </div>
        <div class="config-item" v-if="selectedTags.length > 0">
          <label>{{ t('organize.folderLevelsLabel') }}</label>
          <div class="path-builder">
            <div class="path-segments">
              <div v-for="(tag, index) in selectedTags" :key="index" class="path-segment" @click="removeTag(index)">
                <span class="segment-order">{{ index + 1 }}</span>
                <span class="segment-name">{{ getTagLabel(tag) }}</span>
                <el-icon class="segment-remove">
                  <Close />
                </el-icon>
              </div>
            </div>
            <div class="path-preview">
              <el-icon>
                <FolderOpened />
              </el-icon>
              <span class="preview-text">{{ previewPath }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- 进度显示区域 -->
      <div class="progress-panel" v-if="organizing">
        <div class="progress-header">
          <span class="progress-title">{{ t('organize.progress.title') }}</span>
          <span class="progress-percent">{{ progress.percentage.toFixed(1) }}%</span>
        </div>
        <el-progress :percentage="progress.percentage" :stroke-width="12" :show-text="false" class="progress-bar" />
        <div class="progress-stats">
          <span>{{ t('organize.progress.total', { n: progress.total }) }}</span>
          <span>{{ t('organize.progress.processed', { n: progress.processed }) }}</span>
          <span style="color: var(--success);">{{ t('organize.progress.success', { n: progress.success_count }) }}</span>
          <span style="color: var(--danger);">{{ t('organize.progress.fail', { n: progress.fail_count }) }}</span>
        </div>
        <div class="progress-current" v-if="progress.current_file">
          <span class="current-label">{{ t('organize.progress.current') }}</span>
          <span class="current-file">{{ progress.current_file }}</span>
        </div>
      </div>

      <div class="action-bar">
        <el-button v-if="!organizing" type="primary" @click="startOrganize" :disabled="!canOrganize || isProcessing">
          {{ t('organize.action.start') }}
        </el-button>
        <el-button v-else type="danger" @click="stopOrganize">
          {{ t('organize.action.stop') }}
        </el-button>
      </div>
      <div class="result-panel" v-if="results.length > 0">
        <div class="result-header">
          <span class="result-title">{{ t('organize.result.title') }}</span>
          <span class="result-stats">
            {{ t('organize.result.stats', { success: successCount, fail: failCount }) }}
          </span>
        </div>
        <!-- 失败结果 -->
        <div class="result-section" v-if="failResults.length > 0">
          <div class="section-label fail-label">
            <el-icon>
              <CircleCloseFilled />
            </el-icon>
            <span>{{ t('organize.result.failCount', { n: failCount }) }}</span>
          </div>
          <div class="result-list">
            <div v-for="result in failResults" :key="result.source_path" class="result-item fail">
              <el-icon>
                <CircleCloseFilled />
              </el-icon>
              <span class="result-name clickable" @click="openFile(result.source_path)" :title="result.source_path">{{
                getFileName(result.source_path) }}</span>
              <span class="result-error-inline">{{ result.error }}</span>
            </div>
          </div>
        </div>
        <!-- 成功结果 -->
        <div class="result-section" v-if="successResults.length > 0">
          <div class="section-label success-label">
            <el-icon>
              <SuccessFilled />
            </el-icon>
            <span>{{ t('organize.result.successCount', { n: successCount }) }}</span>
          </div>
          <div class="result-list">
            <div v-for="result in successResults" :key="result.source_path" class="result-item success">
              <el-icon>
                <SuccessFilled />
              </el-icon>
              <span class="result-name clickable" @click="openFile(result.source_path)" :title="result.source_path">{{
                getFileName(result.source_path) }}</span>
              <span class="result-arrow">→</span>
              <span class="result-target clickable" @click="openFile(result.target_path)" :title="result.target_path">{{
                getRelativePath(result.target_path, outputDir) }}</span>
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
import { FolderOpened, Close, SuccessFilled, CircleCloseFilled, Calendar, Clock, MapLocation, Camera, VideoCamera, Files, Coin, Location, Document } from '@element-plus/icons-vue'
import { useFileOps } from '@/composables/useFileOps'
import { useOperationResults } from '@/composables/useOperationResults'
import { useLog, type OrganizeLogOptions, type FileResult } from '@/composables/useLog'
import { useProgressListener } from '@/composables/useProgressListener'
import { isCancelledError } from '@/utils/cancel'
import { t } from '@/i18n'
import type { WorkDirectory, OrganizeResult, OrganizeProgress, CancelResult } from '@/types'

const workDirs = inject<Ref<WorkDirectory[]>>('workDirs')!
const outputDir = inject<Ref<string>>('outputDir')!
const isProcessing = inject<Ref<boolean>>('isProcessing')!

const { getFileName, openFile, getRelativePath } = useFileOps()
const { logOrganizeResults, logCancelledOperation, logDirectoryOperation } = useLog()

const selectedTags = ref<string[]>([])
const timeSource = ref<string>('modified')
const organizing = ref<boolean>(false)
const results = ref<OrganizeResult[]>([])
const progress = ref<OrganizeProgress>({
  total: 0,
  processed: 0,
  success_count: 0,
  fail_count: 0,
  current_file: undefined,
  percentage: 0
})

useProgressListener<OrganizeProgress>('organize-progress', (payload) => {
  progress.value = payload
})

const STORAGE_KEY = 'leaf-tidy-organize-state'

function loadState() {
  try {
    const saved = localStorage.getItem(STORAGE_KEY)
    if (saved) {
      const state = JSON.parse(saved)
      if (state.selectedTags) selectedTags.value = state.selectedTags
      if (state.timeSource) timeSource.value = state.timeSource
    }
  } catch {
    // 状态加载失败时使用默认值
  }
}

function saveState() {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify({
      selectedTags: selectedTags.value,
      timeSource: timeSource.value,
    }))
  } catch {
    // 保存失败时忽略
  }
}

onMounted(() => {
  loadState()
})

onUnmounted(() => {
  saveState()
})

const availableTags = computed(() => [
  { value: 'type', label: t('organize.tag.type'), icon: Document },
  { value: 'year', label: t('organize.tag.year'), icon: Calendar },
  { value: 'month', label: t('organize.tag.month'), icon: Clock },
  { value: 'day', label: t('organize.tag.day'), icon: Calendar },
  { value: 'date', label: t('organize.tag.date'), icon: Calendar },
  { value: 'province', label: t('organize.tag.province'), icon: MapLocation },
  { value: 'city', label: t('organize.tag.city'), icon: Location },
  { value: 'district', label: t('organize.tag.district'), icon: Location },
  { value: 'place', label: t('organize.tag.place'), icon: Location },
  { value: 'make', label: t('organize.tag.make'), icon: Camera },
  { value: 'model', label: t('organize.tag.model'), icon: VideoCamera },
  { value: 'ext', label: t('organize.tag.ext'), icon: Files },
  { value: 'size', label: t('organize.tag.size'), icon: Coin },
])

function getTagLabel(value: string): string {
  const tag = availableTags.value.find(item => item.value === value)
  return tag ? tag.label : value
}

function addTag(value: string) {
  if (!selectedTags.value.includes(value)) {
    selectedTags.value.push(value)
  }
}

function removeTag(index: number) {
  selectedTags.value.splice(index, 1)
}

const previewPath = computed(() => {
  const examples: Record<string, string> = {
    type: t('organize.example.type'),
    year: '2024',
    month: '01',
    day: '15',
    date: '2024-01-15',
    province: t('organize.example.province'),
    city: t('organize.example.city'),
    district: t('organize.example.district'),
    place: t('organize.example.place'),
    make: 'Canon',
    model: 'EOS_R5',
    ext: 'jpg',
    size: '1-10MB',
  }
  return selectedTags.value.map(tag => examples[tag] || tag).join('/')
})

const canOrganize = computed(() => {
  return selectedTags.value.length > 0 && workDirs.value.length > 0 && outputDir.value
})

const { successCount, failCount, successResults, failResults } = useOperationResults(results)

async function stopOrganize() {
  try {
    await invoke<CancelResult>('cancel_operation')
    ElNotification({ type: 'info', title: t('organize.tipTitle'), message: t('organize.notify.stopping') })
  } catch (e: any) {
    ElNotification({ type: 'error', title: t('organize.errorTitle'), message: t('organize.notify.stopFailed', { msg: e }) })
  }
}

async function startOrganize() {
  if (!canOrganize.value) {
    ElNotification({ type: 'warning', title: t('organize.warnTitle'), message: t('organize.notify.missingConfig') })
    return
  }

  organizing.value = true
  isProcessing.value = true
  results.value = []
  progress.value = {
    total: 0,
    processed: 0,
    success_count: 0,
    fail_count: 0,
    current_file: undefined,
    percentage: 0
  }

  try {
    const sourceDirs = workDirs.value.map(d => d.path)
    await logDirectoryOperation('文件整理', sourceDirs, { 目标目录: outputDir.value })
    const res = await invoke<OrganizeResult[]>('organize_files_async', {
      sourceDirs,
      targetDir: outputDir.value,
      rule: {
        tags: selectedTags.value,
        time_source: timeSource.value,
      },
    })
    results.value = res

    const logOptions: OrganizeLogOptions = {
      timeSource: timeSource.value,
      tags: selectedTags.value,
      sourceDirs,
      targetDir: outputDir.value,
    }

    const fileResults: FileResult[] = res.map(r => ({
      source_path: r.source_path,
      target_path: r.target_path,
      success: r.success,
      error: r.error,
      metadata: r.metadata ? {
        modified: r.metadata.modified,
        created: r.metadata.created,
        taken: r.metadata.taken,
        gps_latitude: r.metadata.gps_latitude,
        gps_longitude: r.metadata.gps_longitude,
        gps_province: r.metadata.gps_province,
        gps_city: r.metadata.gps_city,
        gps_district: r.metadata.gps_district,
        gps_place: r.metadata.gps_place,
        camera_make: r.metadata.camera_make,
        camera_model: r.metadata.camera_model,
        size: r.metadata.size,
        format: r.metadata.format,
      } : undefined,
      process_time_ms: r.process_time_ms,
    }))

    await logOrganizeResults(fileResults, logOptions)

    if (res.length > 0) {
      const success = res.filter(r => r.success).length
      const fail = res.filter(r => !r.success).length
      if (fail === 0) {
        ElNotification({ type: 'success', title: t('organize.successTitle'), message: t('organize.notify.doneAll', { n: success }) })
      } else {
        ElNotification({ type: 'warning', title: t('organize.warnTitle'), message: t('organize.notify.donePartial', { success, fail }) })
      }
    } else {
      ElNotification({ type: 'info', title: t('organize.tipTitle'), message: t('organize.notify.noFiles') })
    }
  } catch (e: any) {
    const errMsg = String(e)
    if (isCancelledError(e)) {
      const sourceDirsStr = workDirs.value.map(d => d.path).join(', ')
      await logCancelledOperation('organize', sourceDirsStr, `源目录: ${sourceDirsStr}`)
      ElNotification({ type: 'info', title: t('organize.tipTitle'), message: t('organize.notify.cancelled') })
    } else {
      ElNotification({ type: 'error', title: t('organize.errorTitle'), message: t('organize.notify.failed', { msg: errMsg }) })
    }
  } finally {
    organizing.value = false
    isProcessing.value = false
  }
}
</script>

<style scoped>
/* Page-specific overrides (shared styles in components.css) */
.result-list {
  gap: 6px;
  max-height: 200px;
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
  max-width: 200px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex-shrink: 0;
}

.result-name.clickable {
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

.result-target.clickable {
  cursor: pointer;
  transition: opacity 0.15s;
}

.result-target.clickable:hover {
  opacity: 0.7;
}

.result-item .result-error-inline {
  font-size: 12px;
  color: var(--danger);
  flex-shrink: 0;
  margin-left: auto;
}
</style>