<template>
  <div class="page-container">
    <div class="page-header">
      <h2>{{ t('rename.title') }}</h2>
      <p class="desc">{{ t('rename.desc') }}</p>
    </div>
    <div class="page-content">
      <div class="config-panel">
        <div class="config-row">
          <div class="config-item">
            <label>{{ t('rename.timeSourceLabel') }}</label>
            <el-select v-model="timeSource" :placeholder="t('rename.timeSourcePlaceholder')" style="width: 150px">
              <el-option :label="t('rename.timeSourceModified')" value="modified" />
              <el-option :label="t('rename.timeSourceCreated')" value="created" />
              <el-option :label="t('rename.timeSourceTaken')" value="taken" />
            </el-select>
            <span class="hint">{{ t('rename.timeSourceHint') }}</span>
          </div>
          <div class="config-item">
            <label>{{ t('rename.startIndexLabel') }}</label>
            <el-input-number v-model="startIndex" :min="1" :max="9999" controls-position="right" style="width: 120px" />
          </div>
        </div>
        <div class="config-item">
          <label>{{ t('rename.tagsLabel') }}</label>
          <div class="tag-selector">
            <div class="tag-grid">
              <div v-for="tag in availableTags" :key="tag.value" class="tag-chip" @click="addTag(tag.value)">
                <el-icon>
                  <component :is="tag.icon" />
                </el-icon>
                <span>{{ t(tag.label) }}</span>
              </div>
            </div>
            <div class="separator-grid">
              <div v-for="sep in separators" :key="sep.value" class="sep-chip" @click="addSeparator(sep.value)">
                <span>{{ t(sep.label) }}</span>
              </div>
            </div>
          </div>
        </div>
        <div class="config-item" v-if="templateParts.length > 0">
          <label>{{ t('rename.currentTemplateLabel') }}</label>
          <div class="template-builder">
            <div class="template-segments">
              <div v-for="(part, index) in templateParts" :key="index" class="template-part"
                :class="{ separator: part.type === 'separator' }" @click="removePart(index)">
                <span v-if="part.type === 'separator'" class="part-sep">{{ part.value }}</span>
                <span v-else class="part-tag">{{ t(part.label) }}</span>
                <el-icon class="part-remove">
                  <Close />
                </el-icon>
              </div>
            </div>
            <div class="template-preview">
              <el-icon>
                <Document />
              </el-icon>
              <span class="preview-text">{{ previewName }}<span class="ext-hint">.jpg</span></span>
            </div>
          </div>
        </div>
      </div>

      <!-- 进度显示区域 -->
      <div class="progress-panel" v-if="renaming">
        <div class="progress-header">
          <span class="progress-title">{{ t('rename.progressTitle') }}</span>
          <span class="progress-percent">{{ progress.percentage.toFixed(1) }}%</span>
        </div>
        <el-progress :percentage="progress.percentage" :stroke-width="12" :show-text="false" class="progress-bar" />
        <div class="progress-stats">
          <span>{{ t('rename.progressTotal', { n: progress.total }) }}</span>
          <span>{{ t('rename.progressProcessed', { n: progress.processed }) }}</span>
        </div>
        <div class="progress-current" v-if="progress.current_item">
          <span class="current-label">{{ t('rename.progressCurrentLabel') }}</span>
          <span class="current-file">{{ progress.current_item }}</span>
        </div>
      </div>

      <div class="action-bar">
        <el-button v-if="!renaming" type="primary" :disabled="templateParts.length === 0 || workDirs.length === 0"
          @click="startRename">
          {{ t('rename.startButton') }}
        </el-button>
        <el-button v-else type="danger" @click="stopRename">
          {{ t('rename.stopButton') }}
        </el-button>
      </div>
      <div class="result-panel" v-if="results.length > 0">
        <div class="result-header">
          <span class="result-title">{{ t('rename.resultTitle') }}</span>
          <span class="result-stats">
            {{ t('rename.resultStats', { success: successCount, fail: failCount }) }}
          </span>
        </div>
        <div class="result-section" v-if="failResults.length > 0">
          <div class="section-label fail-label">
            <el-icon>
              <CircleCloseFilled />
            </el-icon>
            <span>{{ t('rename.failCountLabel', { n: failCount }) }}</span>
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
            <span>{{ t('rename.successCountLabel', { n: successCount }) }}</span>
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
                result.new_name }}</span>
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
import { Document, Calendar, Clock, Camera, VideoCamera, Sort, Close, SuccessFilled, CircleCloseFilled, MapLocation, Location, Files, Coin } from '@element-plus/icons-vue'
import { useFileOps } from '@/composables/useFileOps'
import { t } from '@/i18n'
import { useLog, type RenameLogOptions, type FileResult } from '@/composables/useLog'
import type { WorkDirectory, RenameResult, RenameRule, TaskProgress, CancelResult } from '@/types'

interface TemplatePart {
  type: 'tag' | 'separator'
  value: string
  label: string
}

const workDirs = inject<Ref<WorkDirectory[]>>('workDirs')!
const outputDir = inject<Ref<string>>('outputDir')!
const isProcessing = inject<Ref<boolean>>('isProcessing')!

const { getFileName, openFile } = useFileOps()
const { logRenameResults, logCancelledOperation } = useLog()

const templateParts = ref<TemplatePart[]>([])
const startIndex = ref(1)
const timeSource = ref('modified')
const renaming = ref(false)
const results = ref<RenameResult[]>([])
const progress = ref<TaskProgress>({
  total: 0,
  processed: 0,
  current_item: undefined,
  percentage: 0
})

let unlistenProgress: UnlistenFn | null = null

const STORAGE_KEY = 'leaf-tidy-rename-state'

function loadState() {
  try {
    const saved = localStorage.getItem(STORAGE_KEY)
    if (saved) {
      const state = JSON.parse(saved)
      if (state.templateParts) templateParts.value = state.templateParts
      if (state.startIndex) startIndex.value = state.startIndex
      if (state.timeSource) timeSource.value = state.timeSource
    }
  } catch {
    // 状态加载失败时使用默认值
  }
}

function saveState() {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify({
      templateParts: templateParts.value,
      startIndex: startIndex.value,
      timeSource: timeSource.value,
    }))
  } catch {
    // 保存失败时忽略
  }
}

onMounted(async () => {
  loadState()
  unlistenProgress = await listen<TaskProgress>('rename-progress', (event) => {
    progress.value = event.payload
  })
})

onUnmounted(() => {
  saveState()
  if (unlistenProgress) {
    unlistenProgress()
  }
})

const availableTags = [
  { value: 'date', label: 'rename.tag.date', icon: Calendar },
  { value: 'time', label: 'rename.tag.time', icon: Clock },
  { value: 'year', label: 'rename.tag.year', icon: Calendar },
  { value: 'month', label: 'rename.tag.month', icon: Clock },
  { value: 'day', label: 'rename.tag.day', icon: Calendar },
  { value: 'type', label: 'rename.tag.type', icon: Document },
  { value: 'name', label: 'rename.tag.name', icon: Document },
  { value: 'ext', label: 'rename.tag.ext', icon: Files },
  { value: 'index', label: 'rename.tag.index', icon: Sort },
  { value: 'province', label: 'rename.tag.province', icon: MapLocation },
  { value: 'city', label: 'rename.tag.city', icon: Location },
  { value: 'district', label: 'rename.tag.district', icon: Location },
  { value: 'place', label: 'rename.tag.place', icon: Location },
  { value: 'make', label: 'rename.tag.make', icon: Camera },
  { value: 'model', label: 'rename.tag.model', icon: VideoCamera },
  { value: 'exact_size', label: 'rename.tag.exact_size', icon: Coin },
]

const separators = [
  { value: '_', label: '_' },
  { value: '-', label: '-' },
  { value: ' ', label: 'rename.tag.space' },
  { value: '.', label: '.' },
]

function addTag(value: string) {
  const tag = availableTags.find(t => t.value === value)
  if (tag) {
    templateParts.value.push({
      type: 'tag',
      value: tag.value,
      label: tag.label,
    })
  }
}

function addSeparator(value: string) {
  const sep = separators.find(s => s.value === value)
  if (sep) {
    templateParts.value.push({
      type: 'separator',
      value: sep.value,
      label: sep.label,
    })
  }
}

function removePart(index: number) {
  templateParts.value.splice(index, 1)
}

const previewName = computed(() => {
  const examples: Record<string, string> = {
    date: '20240115',
    time: '143000',
    year: '2024',
    month: '01',
    day: '15',
    type: t('rename.example.type'),
    name: 'photo',
    ext: 'jpg',
    index: String(startIndex.value).padStart(3, '0'),
    province: t('rename.example.province'),
    city: t('rename.example.city'),
    district: t('rename.example.district'),
    place: t('rename.example.place'),
    make: 'Canon',
    model: 'EOSR5',
    exact_size: '2.5MB',
  }
  let result = templateParts.value.map(part => {
    if (part.type === 'separator') {
      return part.value
    }
    return examples[part.value] || part.value
  }).join('')
  return result || t('rename.previewEmpty')
})

const successCount = computed(() => results.value.filter(r => r.success).length)
const failCount = computed(() => results.value.filter(r => !r.success).length)
const successResults = computed(() => results.value.filter(r => r.success))
const failResults = computed(() => results.value.filter(r => !r.success))

async function stopRename() {
  try {
    await invoke<CancelResult>('cancel_operation')
    ElNotification({ type: 'info', title: t('rename.notif.infoTitle'), message: t('rename.stopInProgress') })
  } catch (e: any) {
    ElNotification({ type: 'error', title: t('rename.notif.errorTitle'), message: t('rename.stopFailed', { msg: e }) })
  }
}

async function startRename() {
  if (templateParts.value.length === 0) {
    ElNotification({ type: 'warning', title: t('rename.notif.warningTitle'), message: t('rename.warnNoTemplate') })
    return
  }
  if (workDirs.value.length === 0) {
    ElNotification({ type: 'warning', title: t('rename.notif.warningTitle'), message: t('rename.warnNoWorkDir') })
    return
  }
  if (!outputDir.value) {
    ElNotification({ type: 'warning', title: t('rename.notif.warningTitle'), message: t('rename.warnNoOutputDir') })
    return
  }

  renaming.value = true
  isProcessing.value = true
  results.value = []
  progress.value = {
    total: 0,
    processed: 0,
    current_item: undefined,
    percentage: 0
  }

  const paths: string[] = []
  try {
    for (const dir of workDirs.value) {
      const files = await invoke<{ path: string }[]>('scan_directory', { path: dir.path })
      paths.push(...files.map(f => f.path))
    }

    const rule: RenameRule = {
      template_parts: templateParts.value.map(p => ({
        part_type: p.type,
        value: p.value,
      })),
      time_source: timeSource.value,
      start_index: startIndex.value,
    }

    const res = await invoke<RenameResult[]>('batch_rename_async', {
      paths,
      targetDir: outputDir.value,
      rule
    })
    results.value = res
    const logOptions: RenameLogOptions = {
      timeSource: timeSource.value,
      template: templateParts.value.map(p => p.label).join(''),
      startIndex: startIndex.value,
      sourcePaths: paths,
      targetDir: outputDir.value,
    }
    const fileResults: FileResult[] = res.map(r => ({
      source_path: r.source_path,
      target_path: r.target_path,
      new_name: r.new_name,
      success: r.success,
      error: r.error,
    }))
    await logRenameResults(fileResults, logOptions)
    if (res.length > 0) {
      const success = res.filter(r => r.success).length
      const fail = res.filter(r => !r.success).length
      if (fail === 0) {
        ElNotification({ type: 'success', title: t('rename.notif.successTitle'), message: t('rename.allSuccess', { n: success }) })
      } else {
        ElNotification({ type: 'warning', title: t('rename.notif.warningTitle'), message: t('rename.partialSuccess', { success, fail }) })
      }
    } else {
      ElNotification({ type: 'info', title: t('rename.notif.infoTitle'), message: t('rename.noFiles') })
    }
  } catch (e: any) {
    const errMsg = String(e)
    if (errMsg.includes('取消')) {
      const sourcePathsStr = paths.join(', ')
      await logCancelledOperation('rename', sourcePathsStr, `源文件数: ${paths.length}`)
      ElNotification({ type: 'info', title: t('rename.notif.infoTitle'), message: t('rename.cancelled') })
    } else {
      ElNotification({ type: 'error', title: t('rename.notif.errorTitle'), message: t('rename.failed', { msg: errMsg }) })
    }
  } finally {
    renaming.value = false
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

.config-panel {
  background: var(--panel);
  border-radius: 8px;
  padding: 20px;
  margin-bottom: 20px;
}

.config-row {
  display: flex;
  gap: 40px;
  margin-bottom: 20px;
}

.config-row .config-item {
  margin-bottom: 0;
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

.tag-selector {
  background: var(--panel-2);
  border-radius: 8px;
  padding: 16px;
}

.tag-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 10px;
  margin-bottom: 12px;
}

.tag-chip {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 10px 12px;
  background: var(--panel);
  border-radius: 6px;
  border: 1px solid var(--border);
  cursor: pointer;
  transition: background 0.15s;
}

.tag-chip:hover {
  background: var(--panel-2-hover);
}

.tag-chip .el-icon {
  font-size: 14px;
  color: var(--text-secondary);
}

.tag-chip span {
  font-size: 13px;
  color: var(--text-secondary);
}

.separator-grid {
  display: flex;
  gap: 10px;
}

.sep-chip {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 8px 16px;
  background: var(--panel);
  border-radius: 6px;
  border: 1px solid var(--border);
  cursor: pointer;
  transition: background 0.15s;
}

.sep-chip:hover {
  background: var(--panel-2-hover);
}

.sep-chip span {
  font-size: 13px;
  color: var(--text-secondary);
}

.template-builder {
  background: var(--panel-2);
  border-radius: 8px;
  padding: 16px;
}

.template-segments {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-bottom: 12px;
}

.template-part {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  background: var(--primary);
  border-radius: 6px;
  cursor: pointer;
  user-select: none;
  transition: opacity 0.15s;
}

.template-part.separator {
  background: var(--border-strong);
}

.template-part:hover {
  opacity: 0.8;
}

.part-tag {
  font-size: 13px;
  color: var(--white);
}

.part-sep {
  font-size: 13px;
  color: var(--text);
}

.part-remove {
  font-size: 14px;
  color: rgba(var(--white-rgb), 0.7);
}

.template-preview {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px;
  background: var(--panel);
  border-radius: 6px;
  border: 1px dashed var(--border);
}

.template-preview .el-icon {
  font-size: 16px;
  color: var(--text-muted);
}

.preview-text {
  font-size: 13px;
  color: var(--primary);
  word-break: break-all;
}

.ext-hint {
  color: var(--text-muted);
  font-size: 13px;
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
  margin-bottom: 20px;
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
