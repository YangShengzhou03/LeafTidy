<template>
  <div class="page-container">
    <div class="page-header">
      <h2>{{ t('gps.title') }}</h2>
      <p class="desc">{{ t('gps.desc') }}</p>
    </div>
    <div class="page-content">
      <div class="config-panel">
        <div class="config-row">
          <div class="config-item">
            <label>{{ t('gps.latLabel') }}</label>
            <el-input-number v-model="latitude" :min="-90" :max="90" :precision="6" :step="0.000001"
              controls-position="right" style="width: 200px" />
            <span class="range-hint">{{ t('gps.latRange') }}</span>
          </div>
          <div class="config-item">
            <label>{{ t('gps.lngLabel') }}</label>
            <el-input-number v-model="longitude" :min="-180" :max="180" :precision="6" :step="0.000001"
              controls-position="right" style="width: 200px" />
            <span class="range-hint">{{ t('gps.lngRange') }}</span>
          </div>
          <div class="config-item">
            <label>{{ t('gps.preset.label') }}</label>
            <div class="preset-row">
              <el-button v-for="p in presets" :key="p.key" @click="applyPreset(p)">{{ t('gps.preset.' + p.key) }}</el-button>
            </div>
          </div>
        </div>
        <div class="config-item location-item">
          <label>{{ t('gps.locationLabel') }}</label>
          <div class="location-row">
            <el-button :disabled="!coordsValid" @click="queryLocation">{{ t('gps.query') }}</el-button>
            <span class="location-result" v-if="locationText">{{ locationText }}</span>
          </div>
        </div>
      </div>

      <div class="progress-panel" v-if="processing">
        <div class="progress-header">
          <span class="progress-title">{{ t('gps.progressTitle') }}</span>
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
        <el-button v-if="!processing" type="primary" :disabled="workDirs.length === 0 || !coordsValid" @click="startWrite">
          {{ t('gps.start') }}
        </el-button>
        <el-button v-else type="danger" @click="stopWrite">
          {{ t('gps.stop') }}
        </el-button>
      </div>

      <div class="result-panel" v-if="results.length > 0">
        <div class="result-header">
          <span class="result-title">{{ t('gps.resultTitle') }}</span>
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

const latitude = ref<number>(0)
const longitude = ref<number>(0)
const locationText = ref('')

// 常用城市市中心坐标，点击自动填入经纬度，可再手动微调
const presets = [
  { key: 'jian', latitude: 27.113800, longitude: 114.979000 },
  { key: 'nanchang', latitude: 28.682000, longitude: 115.857900 },
  { key: 'hangzhou', latitude: 30.274100, longitude: 120.155100 },
]

function applyPreset(p: { key: string; latitude: number; longitude: number }) {
  latitude.value = p.latitude
  longitude.value = p.longitude
  locationText.value = ''
}

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

const coordsValid = computed(() => latitude.value !== null && longitude.value !== null)

async function queryLocation() {
  locationText.value = t('gps.querying')
  try {
    const loc = await invoke<{ province: string | null; city: string | null; district: string | null; place: string | null }>('query_location', {
      latitude: latitude.value,
      longitude: longitude.value
    })
    const parts = [loc.province, loc.city, loc.district, loc.place].filter(Boolean)
    locationText.value = parts.length > 0 ? parts.join(' ') : t('gps.notFound')
  } catch (e: any) {
    locationText.value = ''
    ElNotification({ type: 'error', title: t('photos.common.error'), message: t('gps.queryFailed', { e }) })
  }
}

const successCount = computed(() => results.value.filter(r => r.success).length)
const failCount = computed(() => results.value.filter(r => !r.success).length)
const successResults = computed(() => results.value.filter(r => r.success))
const failResults = computed(() => results.value.filter(r => !r.success))

function collectImagePaths(files: { path: string }[]): string[] {
  return files.filter(f => /\.(jpg|jpeg)$/i.test(f.path)).map(f => f.path)
}

async function stopWrite() {
  try {
    await invoke<CancelResult>('cancel_operation')
    ElNotification({ type: 'info', title: t('photos.common.tip'), message: t('gps.stopInProgress') })
  } catch (e: any) {
    ElNotification({ type: 'error', title: t('photos.common.error'), message: t('photos.common.stopFailed', { e }) })
  }
}

async function startWrite() {
  if (workDirs.value.length === 0) {
    ElNotification({ type: 'warning', title: t('photos.common.warning'), message: t('photos.common.selectDirs') })
    return
  }
  if (!outputDir.value) {
    ElNotification({ type: 'warning', title: t('photos.common.warning'), message: t('photos.common.selectOutput') })
    return
  }
  if (latitude.value === 0 && longitude.value === 0) {
    ElNotification({ type: 'warning', title: t('photos.common.warning'), message: t('gps.originWarning') })
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

    const res = await invoke<ImageProcessResult[]>('write_gps_async', {
      paths,
      targetDir: outputDir.value,
      latitude: latitude.value,
      longitude: longitude.value
    })
    results.value = res
    await logImageProcessResults('write_gps', res.map(r => ({ ...r })), {
      params: {
        坐标: `${latitude.value.toFixed(6)}, ${longitude.value.toFixed(6)}`,
        处理方式: '复制到输出目录',
      },
      targetDir: outputDir.value,
    })
    const success = res.filter(r => r.success).length
    const fail = res.filter(r => !r.success).length
    if (fail === 0) {
      ElNotification({ type: 'success', title: t('photos.common.success'), message: t('gps.doneAll', { n: success }) })
    } else {
      ElNotification({ type: 'warning', title: t('photos.common.warning'), message: t('gps.donePartial', { s: success, f: fail }) })
    }
  } catch (e: any) {
    const errMsg = String(e)
    if (errMsg.includes('取消')) {
      await logCancelledOperation('write_gps', paths.join(', '), `源文件数: ${paths.length}`)
      ElNotification({ type: 'info', title: t('photos.common.tip'), message: t('gps.cancelled') })
    } else {
      ElNotification({ type: 'error', title: t('photos.common.error'), message: t('gps.failed', { e: errMsg }) })
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

.config-row {
  display: flex;
  gap: 40px;
}

.config-item {
  margin-bottom: 0;
}

.config-item label {
  display: block;
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 12px;
  font-weight: 500;
}

.range-hint {
  display: block;
  font-size: 11px;
  color: var(--text-muted);
  margin-top: 4px;
}

.location-item {
  margin-top: 16px;
}

.preset-row {
  display: flex;
  gap: 6px;
}

.location-row {
  display: flex;
  align-items: center;
  gap: 10px;
}

.location-result {
  font-size: 12px;
  color: var(--text-secondary);
  max-width: 420px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
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
