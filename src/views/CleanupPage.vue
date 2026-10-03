<template>
  <div class="page-container">
    <div class="page-header">
      <h2>{{ t('lq.title') }}</h2>
      <p class="desc">{{ t('lq.desc') }}</p>
    </div>
    <div class="page-content">
      <div class="config-panel">
        <!-- 检测类型复选框组 -->
        <div class="config-item detection-types">
          <label>{{ t('lq.detectTypes') }}</label>
          <el-checkbox-group v-model="detectTypes" :disabled="scanning || deleting">
            <el-checkbox value="blur">
              <span class="checkbox-label">{{ t('lq.typeBlur') }}</span>
              <span class="checkbox-desc">{{ t('lq.typeBlurDesc') }}</span>
            </el-checkbox>
            <el-checkbox value="small">
              <span class="checkbox-label">{{ t('lq.typeSmall') }}</span>
              <span class="checkbox-desc">{{ t('lq.typeSmallDesc') }}</span>
            </el-checkbox>
            <el-checkbox value="screenshot">
              <span class="checkbox-label">{{ t('lq.typeScreenshot') }}</span>
              <span class="checkbox-desc">{{ t('lq.typeScreenshotDesc') }}</span>
            </el-checkbox>
            <el-checkbox value="closed_eye">
              <span class="checkbox-label">{{ t('lq.typeClosedEye') }}</span>
              <span class="checkbox-desc">{{ t('lq.typeClosedEyeDesc') }}</span>
            </el-checkbox>
          </el-checkbox-group>
        </div>

        <!-- 阈值设置行 -->
        <div class="config-row">
          <!-- 模糊阈值（仅勾选模糊时显示） -->
          <div class="config-item threshold-item" v-if="detectTypes.includes('blur')">
            <span class="model-display-label">{{ t('lq.blurThreshold') }}</span>
            <el-select v-model="blurThreshold" :disabled="scanning || deleting" class="threshold-select">
              <el-option
                v-for="opt in BLUR_THRESHOLD_OPTIONS"
                :key="opt.value"
                :value="opt.value"
                :label="`${t(opt.labelKey)}`"
              />
            </el-select>
          </div>

          <!-- 最小尺寸（仅勾选小尺寸时显示） -->
          <div class="config-item threshold-item" v-if="detectTypes.includes('small')">
            <span class="model-display-label">{{ t('lq.minDimension') }}</span>
            <el-select v-model="minDimension" :disabled="scanning || deleting" class="threshold-select">
              <el-option
                v-for="opt in MIN_DIMENSION_OPTIONS"
                :key="opt.value"
                :value="opt.value"
                :label="opt.label"
              />
            </el-select>
          </div>
        </div>
      </div>

      <!-- 扫描进度 -->
      <div class="progress-panel" v-if="scanning">
        <div class="progress-header">
          <span class="progress-title">{{ t('lq.scanning') }}</span>
          <span class="progress-percent">{{ scanProgress.percentage.toFixed(1) }}%</span>
        </div>
        <el-progress :percentage="scanProgress.percentage" :stroke-width="12" :show-text="false" class="progress-bar" />
        <div class="progress-stats">
          <span>{{ t('lq.total') }} {{ scanProgress.total }}</span>
          <span>{{ t('lq.scanned') }} {{ scanProgress.processed }}</span>
        </div>
        <div class="progress-current" v-if="scanProgress.current_item">
          <span class="current-label">{{ t('lq.current') }}</span>
          <span class="current-file">{{ scanProgress.current_item }}</span>
        </div>
      </div>

      <!-- 删除进度 -->
      <div class="progress-panel" v-if="deleting">
        <div class="progress-header">
          <span class="progress-title">{{ t('lq.deleting') }}</span>
          <span class="progress-percent">{{ deleteProgress.percentage.toFixed(1) }}%</span>
        </div>
        <el-progress :percentage="deleteProgress.percentage" :stroke-width="12" :show-text="false" class="progress-bar" />
        <div class="progress-stats">
          <span>{{ t('lq.total') }} {{ deleteProgress.total }}</span>
          <span>{{ t('lq.deleted') }} {{ deleteProgress.processed }}</span>
        </div>
      </div>

      <div class="action-bar">
        <el-button v-if="!scanning && !deleting" type="primary"
          :disabled="workDirs.length === 0 || isProcessing || detectTypes.length === 0" @click="startScan">
          {{ t('lq.startScan') }}
        </el-button>
        <el-button v-if="scanning || deleting" type="danger" @click="stopOperation">
          {{ scanning ? t('lq.stopScan') : t('lq.stopDelete') }}
        </el-button>
      </div>

      <!-- 扫描中：实时展示已发现的低质图片 -->
      <div class="result-panel live-results" v-if="scanning && liveResults.length > 0">
        <div class="result-header">
          <span class="result-title">{{ t('lq.resultTitle') }}</span>
          <span class="result-stats scanning-stats">
            {{ t('lq.liveFound', { n: liveResults.length }) }}
          </span>
        </div>
        <div class="image-grid">
          <div
            v-for="img in liveResults"
            :key="img.path"
            class="image-card"
            :class="{ selected: selectedImages.includes(img.path) }"
            @click="toggleSelect(img.path)"
          >
            <div class="image-preview">
              <img v-if="img.thumb" :src="img.thumb" class="preview-thumb" />
              <div v-else class="preview-placeholder">{{ img.name }}</div>
              <div class="select-overlay" v-if="selectedImages.includes(img.path)">
                <SuccessFilled class="check-icon" />
              </div>
            </div>
            <div class="image-info">
              <span class="image-name clickable" @click.stop="openFile(img.path)" :title="img.path">{{ img.name }}</span>
              <span class="image-variance" v-if="img.variance > 0">{{ t('lq.variance', { v: Math.round(img.variance) }) }}</span>
              <span class="image-reason">{{ img.reason ? `[${img.reason}]` : '' }}</span>
            </div>
          </div>
        </div>
      </div>

      <div class="result-panel" v-if="!scanning && scanResult && scanResult.images.length > 0">
        <div class="result-header">
          <span class="result-title">{{ t('lq.resultTitle') }}</span>
          <span class="result-stats">
            {{ t('lq.resultStats', { n: scanResult.images.length, total: scanResult.total_scanned }) }}
          </span>
        </div>

        <div class="selection-bar">
          <el-button link type="primary" @click="toggleSelectAll">
            {{ allSelected ? t('lq.clearSelection') : t('lq.selectAll') }}
          </el-button>
        </div>

        <div class="image-grid">
          <div
            v-for="img in scanResult.images"
            :key="img.path"
            class="image-card"
            :class="{ selected: selectedImages.includes(img.path) }"
            @click="toggleSelect(img.path)"
          >
            <div class="image-preview">
              <img v-if="img.thumb" :src="img.thumb" class="preview-thumb" />
              <div v-else class="preview-placeholder">{{ img.name }}</div>
              <div class="select-overlay" v-if="selectedImages.includes(img.path)">
                <SuccessFilled class="check-icon" />
              </div>
            </div>
            <div class="image-info">
              <span class="image-name clickable" @click.stop="openFile(img.path)" :title="img.path">{{ img.name }}</span>
              <span class="image-variance" v-if="img.variance > 0">{{ t('lq.variance', { v: Math.round(img.variance) }) }}</span>
              <span class="image-reason">{{ img.reason ? `[${img.reason}]` : '' }}</span>
            </div>
          </div>
        </div>

        <div class="batch-action" v-if="selectedImages.length > 0">
          <el-button type="danger" :disabled="deleting" @click="deleteSelected">
            {{ t('lq.deleteSelected', { n: selectedImages.length }) }}
          </el-button>
        </div>
      </div>

      <div class="result-panel" v-if="!scanning && scanResult && scanResult.images.length === 0 && scanned">
        <div class="empty-result">{{ t('lq.nothingFound') }}</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, inject, type Ref } from 'vue'
import { ElNotification, ElMessageBox } from 'element-plus'
import { invoke } from '@tauri-apps/api/core'
import { SuccessFilled } from '@element-plus/icons-vue'
import { t } from '@/i18n'
import { useFileOps } from '@/composables/useFileOps'
import { useLog, type FileResult } from '@/composables/useLog'
import { useProgressListener } from '@/composables/useProgressListener'
import { isCancelledError } from '@/utils/cancel'
import type { WorkDirectory, LowQualityImage, LowQualityScanResult, BatchOperationResult, TaskProgress, CancelResult } from '@/types'

const workDirs = inject<Ref<WorkDirectory[]>>('workDirs')!
const isProcessing = inject<Ref<boolean>>('isProcessing')!

const { openFile } = useFileOps()
const { logLowQualityResults, logCancelledOperation, logOperationStart, logEvent, logDirectoryOperation } = useLog()

const detectTypes = ref<string[]>(['blur'])
const blurThreshold = ref<number>(150)
const minDimension = ref<number>(400)

// 模糊阈值预设（方差阈值：低于此值判定为模糊）
const BLUR_THRESHOLD_OPTIONS = [
  { value: 80, labelKey: 'lq.thresholdGentle' },
  { value: 150, labelKey: 'lq.thresholdStandard' },
  { value: 300, labelKey: 'lq.thresholdStrict' },
]

// 最小尺寸预设（宽或高低于此值判定为尺寸过低）
const MIN_DIMENSION_OPTIONS = [
  { value: 200, label: '200px' },
  { value: 400, label: '400px' },
  { value: 800, label: '800px' },
  { value: 1080, label: '1080px' },
]
const scanning = ref(false)
const deleting = ref(false)
const scanResult = ref<LowQualityScanResult | null>(null)
const liveResults = ref<LowQualityImage[]>([])
const selectedImages = ref<string[]>([])
const scanned = ref(false)

const scanProgress = ref<TaskProgress>({
  total: 0,
  processed: 0,
  current_item: undefined,
  percentage: 0,
})
const deleteProgress = ref<TaskProgress>({
  total: 0,
  processed: 0,
  current_item: undefined,
  percentage: 0,
})

useProgressListener<TaskProgress>('lowquality-scan-progress', (payload) => {
  scanProgress.value = payload
})
useProgressListener<LowQualityImage>('lowquality-scan-result', (img) => {
  liveResults.value.push(img)
})
useProgressListener<TaskProgress>('lowquality-delete-progress', (payload) => {
  deleteProgress.value = payload
})



const allSelected = computed(() => {
  return scanResult.value
    ? selectedImages.value.length === scanResult.value.images.length
    : false
})

function toggleSelect(path: string) {
  const idx = selectedImages.value.indexOf(path)
  if (idx >= 0) {
    selectedImages.value.splice(idx, 1)
  } else {
    selectedImages.value.push(path)
  }
}

function toggleSelectAll() {
  if (allSelected.value) {
    selectedImages.value = []
  } else {
    selectedImages.value = scanResult.value?.images.map(img => img.path) ?? []
  }
}

async function stopOperation() {
  try {
    await invoke<CancelResult>('cancel_operation')
    ElNotification({ type: 'info', title: t('lq.tip'), message: t('lq.stopping') })
  } catch (e: any) {
    ElNotification({ type: 'error', title: t('lq.error'), message: t('lq.stopFailed', { e }) })
  }
}

async function startScan() {
  if (workDirs.value.length === 0) {
    ElNotification({ type: 'warning', title: t('lq.warning'), message: t('lq.selectDirFirst') })
    return
  }
  if (detectTypes.value.length === 0) {
    ElNotification({ type: 'warning', title: t('lq.warning'), message: t('lq.selectTypeFirst') })
    return
  }

  scanning.value = true
  isProcessing.value = true
  scanResult.value = null
  liveResults.value = []
  selectedImages.value = []
  scanned.value = false
  scanProgress.value = {
    total: 0,
    processed: 0,
    current_item: undefined,
    percentage: 0,
  }

  const paths = workDirs.value.map(d => d.path)

  const typeLabels: Record<string, string> = {
    blur: t('lq.typeBlur'),
    small: t('lq.typeSmall'),
    screenshot: t('lq.typeScreenshot'),
    closed_eye: t('lq.typeClosedEye'),
  }
  const typeNames = detectTypes.value.map(v => typeLabels[v] || v).join(', ')

  await logDirectoryOperation('扫描低质图片', paths, { 检测类型: typeNames })
  await logOperationStart(t('lq.log.scan'), {
    [t('log.field.detectType')]: typeNames,
    [t('log.field.blurThreshold')]: blurThreshold.value,
    [t('log.field.minSize')]: minDimension.value,
    [t('log.field.scanDir')]: paths.join(', '),
  })

  try {
    const res = await invoke<LowQualityScanResult>('scan_low_quality_images_async', {
      paths,
      detectTypes: detectTypes.value,
      varianceThreshold: blurThreshold.value,
      minDimension: minDimension.value,
    })
    scanResult.value = res
    scanned.value = true

    if (res.images.length === 0) {
      await logEvent(t('log.scanCompleteNone', { n: res.total_scanned }))
      ElNotification({ type: 'success', title: t('lq.success'), message: t('lq.nothingFound') })
    } else {
      await logEvent(t('log.scanCompleteFound', { n: res.images.length }))
      ElNotification({ type: 'success', title: t('lq.success'), message: t('lq.foundCount', { n: res.images.length }) })
    }
  } catch (e: any) {
    const errMsg = String(e)
    if (isCancelledError(e)) {
      const pathsStr = workDirs.value.map(d => d.path).join(', ')
      await logCancelledOperation('lowquality', pathsStr, `${t('lq.log.scan')} - ${t('common.cancel')}`)
      ElNotification({ type: 'info', title: t('lq.tip'), message: t('lq.scanCancelled') })
    } else {
      ElNotification({ type: 'error', title: t('lq.error'), message: t('lq.scanFailed', { e: errMsg }) })
    }
  } finally {
    scanning.value = false
    isProcessing.value = false
  }
}

async function deleteSelected() {
  if (selectedImages.value.length === 0) return

  // 确认对话框
  try {
    await ElMessageBox.confirm(
      t('lq.deleteSelected', { n: selectedImages.value.length }),
      t('lq.tip'),
      {
        confirmButtonText: t('common.confirm'),
        cancelButtonText: t('common.cancel'),
        type: 'warning',
      },
    )
  } catch {
    return
  }

  deleting.value = true
  isProcessing.value = true
  deleteProgress.value = {
    total: selectedImages.value.length,
    processed: 0,
    current_item: undefined,
    percentage: 0,
  }

  try {
    const res = await invoke<BatchOperationResult>('delete_low_quality_images_async', {
      files: selectedImages.value,
    })

    const results: FileResult[] = res.results.map((r, index) => ({
      source_path: selectedImages.value[index] || '',
      success: r.success,
      error: r.error,
    }))
    await logLowQualityResults(results)

    if (res.fail_count === 0) {
      ElNotification({ type: 'success', title: t('lq.success'), message: t('lq.deletedCount', { n: res.success_count }) })
    } else {
      ElNotification({ type: 'warning', title: t('lq.warning'), message: t('lq.deleteFailCount', { n: res.fail_count }) })
    }

    // 删除后重新扫描（保留阈值）
    selectedImages.value = []
    await startScan()
  } catch (e: any) {
    const errMsg = String(e)
    if (isCancelledError(e)) {
      await logCancelledOperation('lowquality', selectedImages.value.join(', '), `${t('lq.log.delete')} - ${t('common.cancel')}`)
      ElNotification({ type: 'info', title: t('lq.tip'), message: t('lq.deleteCancelled') })
    } else {
      ElNotification({ type: 'error', title: t('lq.error'), message: t('lq.deleteFailed', { e: errMsg }) })
    }
  } finally {
    deleting.value = false
    isProcessing.value = false
  }
}
</script>

<style scoped>
/* Page-specific overrides (shared styles in components.css) */
.config-item label {
  margin-bottom: 8px;
  display: block;
}

.detection-types {
  margin-bottom: 4px;
}

.threshold-item {
  display: flex;
  align-items: center;
  gap: 12px;
}

.model-display-label {
  font-size: 13px;
  color: var(--text-secondary);
  flex-shrink: 0;
}

.threshold-select {
  width: 180px;
}

.checkbox-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--text);
  margin-right: 4px;
}

.checkbox-desc {
  font-size: 11px;
  color: var(--text-muted);
}

.selection-bar {
  display: flex;
  justify-content: flex-end;
  margin-bottom: 12px;
}

.image-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
  gap: 12px;
}

.image-card {
  position: relative;
  border-radius: 8px;
  background: var(--panel-2);
  padding: 8px;
  cursor: pointer;
  transition: all 0.15s;
  border: 2px solid transparent;
}

.image-card:hover {
  border-color: rgba(var(--primary-rgb), 0.3);
}

.image-card.selected {
  border-color: var(--primary);
}

.image-preview {
  position: relative;
  width: 100%;
  aspect-ratio: 1;
  border-radius: 6px;
  overflow: hidden;
  background: var(--panel);
}

.preview-thumb {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.preview-placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 11px;
  color: var(--text-muted);
  text-align: center;
  padding: 4px;
}

.select-overlay {
  position: absolute;
  top: 4px;
  right: 4px;
  width: 20px;
  height: 20px;
  border-radius: 50%;
  background: var(--primary);
  display: flex;
  align-items: center;
  justify-content: center;
}

.check-icon {
  color: white;
  font-size: 12px;
}

.image-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
  margin-top: 6px;
}

.image-name {
  font-size: 12px;
  color: var(--text);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.image-variance {
  font-size: 11px;
  color: var(--text-muted);
}

.image-reason {
  font-size: 11px;
  color: var(--primary);
}

.batch-action {
  display: flex;
  justify-content: flex-end;
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px solid var(--panel-2);
}

.empty-result {
  padding: 48px 0;
  text-align: center;
  font-size: 14px;
  color: var(--text-muted);
}
</style>
