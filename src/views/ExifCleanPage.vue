<template>
  <div class="page-container">
    <div class="page-header">
      <h2>{{ t('exif.title') }}</h2>
      <p class="desc">{{ t('exif.desc') }}</p>
    </div>
    <div class="page-content">
      <div class="config-panel">
        <el-checkbox-group v-model="stripItems" class="privacy-list">
            <el-checkbox value="exif">{{ t('exif.stripExif') }}</el-checkbox>
            <el-checkbox value="gps">{{ t('exif.stripGps') }}</el-checkbox>
            <el-checkbox value="camera">{{ t('exif.stripCamera') }}</el-checkbox>
            <el-checkbox value="xmp">{{ t('exif.stripXmp') }}</el-checkbox>
            <el-checkbox value="photoshop">{{ t('exif.stripPhotoshop') }}</el-checkbox>
          </el-checkbox-group>
      </div>

      <div class="progress-panel" v-if="processing">
        <div class="progress-header">
          <span class="progress-title">{{ t('exif.progressTitle') }}</span>
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
        <el-button v-if="!processing" type="primary" :disabled="workDirs.length === 0 || isProcessing" @click="startClean">
          {{ t('exif.start') }}
        </el-button>
        <el-button v-else type="danger" @click="stopClean">
          {{ t('exif.stop') }}
        </el-button>
      </div>

      <div class="result-panel" v-if="results.length > 0">
        <div class="result-header">
          <span class="result-title">{{ t('exif.resultTitle') }}</span>
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
import { ref, inject, type Ref } from 'vue'
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

const processing = ref(false)
const results = ref<ImageProcessResult[]>([])
const stripItems = ref<string[]>(['exif', 'gps', 'camera', 'xmp', 'photoshop'])
const progress = ref<TaskProgress>({
  total: 0,
  processed: 0,
  current_item: undefined,
  percentage: 0
})

useProgressListener<TaskProgress>('image-process-progress', (payload) => {
  progress.value = payload
})

const { successCount, failCount, successResults, failResults } = useOperationResults(results)

async function stopClean() {
  try {
    await invoke<CancelResult>('cancel_operation')
    ElNotification({ type: 'info', title: t('photos.common.tip'), message: t('exif.stopInProgress') })
  } catch (e: any) {
    ElNotification({ type: 'error', title: t('photos.common.error'), message: t('photos.common.stopFailed', { e }) })
  }
}

async function startClean() {
  if (workDirs.value.length === 0) {
    ElNotification({ type: 'warning', title: t('photos.common.warning'), message: t('photos.common.selectDirs') })
    return
  }
  if (!outputDir.value) {
    ElNotification({ type: 'warning', title: t('photos.common.warning'), message: t('photos.common.selectOutput') })
    return
  }

  processing.value = true
  isProcessing.value = true
  if (stripItems.value.length === 0) {
    ElNotification({ type: 'warning', title: t('photos.common.warning'), message: t('exif.noneSelected') })
    processing.value = false
    isProcessing.value = false
    return
  }
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
    await logDirectoryOperation('EXIF脱敏-读取目录', sourceDirs)
    for (const dir of workDirs.value) {
      const files = await invoke<{ path: string }[]>('scan_directory', { path: dir.path })
      paths.push(...collectImagePaths(files))
    }

    if (paths.length === 0) {
      ElNotification({ type: 'info', title: t('photos.common.tip'), message: t('photos.common.noJpg') })
      return
    }

    await logOperationStart(t('exif.log.clean'), {
      [t('log.field.cleanupTarget')]: stripItems.value.join('、'),
      [t('log.field.outputDir')]: outputDir.value,
      [t('log.field.fileCount')]: paths.length,
    })

    const res = await invoke<ImageProcessResult[]>('strip_exif_async', {
      paths,
      targetDir: outputDir.value,
      options: {
        exif: stripItems.value.includes('exif'),
        gps: stripItems.value.includes('gps'),
        camera: stripItems.value.includes('camera'),
        xmp: stripItems.value.includes('xmp'),
        photoshop: stripItems.value.includes('photoshop'),
      }
    })
    results.value = res
    await logImageProcessResults('exif_clean', res.map(r => ({ ...r })), {
      params: {
        [t('log.field.cleanupTarget')]: stripItems.value.join('、'),
        [t('log.field.outputDir')]: outputDir.value,
      },
      targetDir: outputDir.value,
    })
    const success = res.filter(r => r.success).length
    const fail = res.filter(r => !r.success).length
    if (fail === 0) {
      ElNotification({ type: 'success', title: t('photos.common.success'), message: t('exif.doneAll', { n: success }) })
    } else {
      ElNotification({ type: 'warning', title: t('photos.common.warning'), message: t('exif.donePartial', { s: success, f: fail }) })
    }
  } catch (e: any) {
    const errMsg = String(e)
    if (isCancelledError(e)) {
      await logCancelledOperation('exif_clean', paths.join(', '), t('log.sourceFileCount', { n: paths.length }))
      ElNotification({ type: 'info', title: t('photos.common.tip'), message: t('exif.cancelled') })
    } else {
      ElNotification({ type: 'error', title: t('photos.common.error'), message: t('exif.failed', { e: errMsg }) })
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

.config-panel {
  padding: 16px 20px;
}

.privacy-list {
  display: flex;
  flex-wrap: wrap;
  gap: 4px 20px;
}

.privacy-list .el-checkbox {
  margin-right: 0;
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
