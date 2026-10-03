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
        <el-button v-if="!processing" type="primary" :disabled="workDirs.length === 0 || !coordsValid || isProcessing" @click="startWrite">
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
            <div v-for="result in failResults" :key="result.source_path" class="result-item fail">
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
            <div v-for="result in successResults" :key="result.source_path" class="result-item success">
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
import { ref, computed, inject, type Ref } from 'vue'
import { ElNotification } from 'element-plus'
import { invoke } from '@tauri-apps/api/core'
import { SuccessFilled, CircleCloseFilled } from '@element-plus/icons-vue'
import { t } from '@/i18n'
import { useFileOps } from '@/composables/useFileOps'
import { useOperationResults } from '@/composables/useOperationResults'
import { useLog } from '@/composables/useLog'
import { useProgressListener } from '@/composables/useProgressListener'
import { isCancelledError } from '@/utils/cancel'
import type { WorkDirectory, ImageProcessResult, TaskProgress, CancelResult } from '@/types'

const workDirs = inject<Ref<WorkDirectory[]>>('workDirs')!
const outputDir = inject<Ref<string>>('outputDir')!
const isProcessing = inject<Ref<boolean>>('isProcessing')!

const { getFileName, openFile, collectImagePaths } = useFileOps()
const { logImageProcessResults, logCancelledOperation, logOperationStart, logDirectoryOperation } = useLog()

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

useProgressListener<TaskProgress>('image-process-progress', (payload) => {
  progress.value = payload
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

const { successCount, failCount, successResults, failResults } = useOperationResults(results)

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
    const sourceDirs = workDirs.value.map(d => d.path)
    await logDirectoryOperation('写入GPS-读取目录', sourceDirs)
    for (const dir of workDirs.value) {
      const files = await invoke<{ path: string }[]>('scan_directory', { path: dir.path })
      paths.push(...collectImagePaths(files))
    }

    if (paths.length === 0) {
      ElNotification({ type: 'info', title: t('photos.common.tip'), message: t('photos.common.noJpg') })
      return
    }

    await logOperationStart(t('gps.log.write'), {
      [t('log.field.coordinate')]: `${latitude.value.toFixed(6)}, ${longitude.value.toFixed(6)}`,
      [t('log.field.outputDir')]: outputDir.value,
      [t('log.field.fileCount')]: paths.length,
    })

    const res = await invoke<ImageProcessResult[]>('write_gps_async', {
      paths,
      targetDir: outputDir.value,
      latitude: latitude.value,
      longitude: longitude.value
    })
    results.value = res
    await logImageProcessResults('write_gps', res.map(r => ({ ...r })), {
      params: {
        [t('log.field.coordinate')]: `${latitude.value.toFixed(6)}, ${longitude.value.toFixed(6)}`,
        [t('log.field.outputDir')]: outputDir.value,
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
    if (isCancelledError(e)) {
      await logCancelledOperation('write_gps', paths.join(', '), t('log.sourceFileCount', { n: paths.length }))
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
/* Page-specific overrides (shared styles in components.css) */
.page-content {
  max-width: 900px;
}

.config-row {
  gap: 40px;
}

.config-item {
  margin-bottom: 0;
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

.action-bar {
  align-items: center;
  gap: 12px;
}

.action-hint {
  font-size: 13px;
  color: var(--text-muted);
  margin-right: auto;
}

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
  max-width: 260px;
}

.result-name.clickable,
.result-target.clickable {
  cursor: pointer;
  transition: color 0.15s;
}

.result-error-inline {
  font-size: 12px;
  color: var(--danger);
  flex-shrink: 0;
  margin-left: auto;
}
</style>
