<template>
  <div class="page-container">
    <div class="page-header">
      <h2>{{ t('ai.title') }}</h2>
      <p class="desc">{{ t('ai.desc') }}</p>
    </div>
    <div class="page-content">
      <!-- 当前模型与置信度阈值 -->
      <div class="config-panel">
        <div class="config-row">
          <div class="config-item threshold-item">
            <span class="model-display-label">{{ t('ai.mode') }}</span>
            <el-select v-model="mode" :disabled="classifying" class="threshold-select">
              <el-option :value="'hybrid'" :label="t('ai.mode.hybrid')" />
              <el-option :value="'content'" :label="t('ai.mode.content')" />
              <el-option :value="'face'" :label="t('ai.mode.face')" />
            </el-select>
          </div>
          <template v-if="mode !== 'face'">
            <div class="config-item model-display">
              <span class="model-display-label">{{ t('ai.model.current') }}</span>
              <span class="model-display-name">{{ currentModelName }}</span>
              <span class="model-display-hint">{{ t('ai.model.hint') }}</span>
            </div>
            <div class="config-item threshold-item">
              <span class="model-display-label">{{ t('ai.confidenceThreshold') }}</span>
              <el-select v-model="CONFIDENCE_THRESHOLD" :disabled="classifying" class="threshold-select">
                <el-option
                  v-for="opt in THRESHOLD_OPTIONS"
                  :key="opt.value"
                  :value="opt.value"
                  :label="`${t(opt.labelKey)} (${Math.round(opt.value * 100)}%)`"
                />
              </el-select>
            </div>
          </template>
          <template v-if="mode === 'face'">
            <div class="config-item model-display">
              <span class="model-display-label">{{ t('ai.model.current') }}</span>
              <span class="model-display-name">YuNet + SFace</span>
              <span class="model-display-hint">{{ t('ai.face.modelHint') }}</span>
            </div>
          </template>
        </div>
      </div>

      <!-- 进度显示 -->
      <div class="progress-panel" v-if="classifying">
        <div class="progress-header">
          <span class="progress-title">{{ progressTitle }}</span>
          <span class="progress-percent">{{ progress.percentage.toFixed(1) }}%</span>
        </div>
        <el-progress :percentage="progress.percentage" :stroke-width="12" :show-text="false" class="progress-bar" />
        <div class="progress-stats">
          <span>{{ t('ai.progress.scanned', { n: progress.processed }) }}</span>
          <span v-if="progress.current_category" class="current-cat">
            {{ progress.current_category }}
          </span>
        </div>
        <div class="progress-current" v-if="progress.current_item">
          <span class="current-label">{{ t('photos.common.current') }}</span>
          <span class="current-file">{{ getFileName(progress.current_item) }}</span>
        </div>
      </div>

      <!-- 操作按钮 -->
      <div class="action-bar">
        <el-button v-if="!classifying" type="primary" @click="onStart" :disabled="!canStart || isProcessing">
          {{ startBtnLabel }}
        </el-button>
        <el-button v-else type="danger" @click="stopClassify">
          {{ mode === 'content' ? t('ai.action.stop') : t('ai.face.stop') }}
        </el-button>
      </div>

      <!-- 人物分组结果（人脸模式 / 混合模式） -->
      <div class="result-panel" v-if="(mode === 'face' || mode === 'hybrid') && (personGroups.length > 0 || faceAnalyzed)">
        <div class="result-header">
          <span class="result-title">{{ t('ai.face.resultTitle') }}</span>
          <el-button v-if="personGroups.length > 0" type="primary" size="default" :disabled="archiving" @click="archiveByPerson">
            {{ t('ai.face.archive') }}
          </el-button>
        </div>

        <div class="face-empty" v-if="personGroups.length === 0">{{ t('ai.face.noFaces') }}</div>

        <div v-for="group in personGroups" :key="group.id" class="person-group">
          <div class="person-header">
            <span class="person-name" :title="group.name">{{ group.name }}</span>
            <span class="person-count">{{ t('ai.face.photoCount', { n: group.photos.length }) }}</span>
            <div class="person-actions">
              <el-button link type="primary" @click="renamePerson(group)">{{ t('ai.face.rename') }}</el-button>
              <el-button link type="primary" @click="openMergeDialog(group)">{{ t('ai.face.merge') }}</el-button>
            </div>
          </div>
          <div class="face-list">
            <div v-for="photo in group.photos" :key="photo.path" class="face-item clickable" @click="openFile(photo.path)" :title="photo.path">
              <img v-if="photo.thumb" :src="photo.thumb" class="face-thumb" />
              <span class="face-name">{{ photo.name }}</span>
              <span class="face-sim">{{ (photo.similarity * 100).toFixed(0) }}%</span>
            </div>
          </div>
        </div>
      </div>

      <!-- 合并人物对话框 -->
      <el-dialog v-model="mergeDialogVisible" :title="t('ai.face.mergeTitle')" width="400px" :append-to-body="true">
        <div class="merge-row">
          <span class="merge-label">{{ t('ai.face.mergeTarget') }}</span>
          <el-select v-model="mergeTargetId" style="flex: 1">
            <el-option v-for="g in mergeCandidates" :key="g.id" :value="g.id" :label="g.name" />
          </el-select>
        </div>
        <template #footer>
          <el-button @click="mergeDialogVisible = false">{{ t('common.cancel') }}</el-button>
          <el-button type="primary" @click="confirmMerge" :disabled="!mergeTargetId">{{ t('common.confirm') }}</el-button>
        </template>
      </el-dialog>

      <!-- 分类结果（内容模式 / 混合模式） -->
      <div class="result-panel" v-if="(mode === 'content' || mode === 'hybrid') && results.length > 0">
        <div class="result-header">
          <span class="result-title">{{ t('ai.result.title') }}</span>
          <span class="result-stats">{{ t('ai.result.total', { n: results.length }) }}</span>
        </div>

        <!-- 分类汇总（echarts 圆环图 + 图例，前 7 类，其余合并为"其他"） -->
        <div class="category-summary" v-if="chartSlices.length > 0">
          <div ref="chartContainer" class="donut-chart"></div>
          <div class="donut-legend">
            <div v-for="(s, i) in chartSlices" :key="s.cat" class="legend-row">
              <span class="legend-dot" :style="{ background: CHART_COLORS[i % CHART_COLORS.length] }"></span>
              <span class="legend-name" :title="s.cat">{{ s.cat }}</span>
              <span class="legend-pct">{{ (s.pct * 100).toFixed(1) }}%</span>
              <span class="legend-count">{{ s.count }}</span>
            </div>
          </div>
        </div>

        <!-- 失败结果 -->
        <div class="result-section" v-if="failResults.length > 0">
          <div class="section-label fail-label">
            <el-icon><CircleCloseFilled /></el-icon>
            <span>{{ t('photos.common.failedCount', { n: failCount }) }}</span>
          </div>
          <div class="result-list">
            <div v-for="result in failResults" :key="result.source_path" class="result-item fail">
              <el-icon><CircleCloseFilled /></el-icon>
              <span class="result-name clickable" @click="openFile(result.source_path)" :title="result.source_path">{{
                getFileName(result.source_path) }}</span>
              <span class="result-error-inline">{{ result.error }}</span>
            </div>
          </div>
        </div>

        <!-- 成功结果 -->
        <div class="result-section" v-if="successResults.length > 0">
          <div class="section-label success-label">
            <el-icon><SuccessFilled /></el-icon>
            <span>{{ t('photos.common.successCount', { n: successCount }) }}</span>
          </div>
          <div class="result-list">
            <div v-for="result in successResults" :key="result.source_path" class="result-item success">
              <el-icon><SuccessFilled /></el-icon>
              <span class="result-name clickable" @click="openFile(result.source_path)" :title="result.source_path">{{
                getFileName(result.source_path) }}</span>
              <span class="result-category">{{ result.category }}</span>
              <span class="result-confidence">{{ t('ai.result.confidence', { p: ((result.confidence || 0) * 100).toFixed(0) }) }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, inject, watch, nextTick, type Ref, onMounted, onUnmounted } from 'vue'
import { ElNotification, ElMessageBox } from 'element-plus'
import * as echarts from 'echarts/core'
import { PieChart } from 'echarts/charts'
import { TooltipComponent } from 'echarts/components'
import { CanvasRenderer } from 'echarts/renderers'
import { invoke } from '@tauri-apps/api/core'
import { SuccessFilled, CircleCloseFilled } from '@element-plus/icons-vue'
import { useFileOps } from '@/composables/useFileOps'
import { useOperationResults } from '@/composables/useOperationResults'
import { useProgressListener } from '@/composables/useProgressListener'
import { useLog } from '@/composables/useLog'
import { isCancelledError } from '@/utils/cancel'
import { t } from '@/i18n'
import type { WorkDirectory, ClassifyResult, ClassifyProgress, ClassifyRule, PersonGroup, Person, BatchOperationResult } from '@/types'

echarts.use([PieChart, TooltipComponent, CanvasRenderer])

const workDirs = inject<Ref<WorkDirectory[]>>('workDirs')!
const outputDir = inject<Ref<string>>('outputDir')!
const isProcessing = inject<Ref<boolean>>('isProcessing')!

const { getFileName, openFile } = useFileOps()

const CONFIDENCE_THRESHOLD = ref(0.15)

// 置信度阈值预设（4 档），0.15 为原滑块默认值
const THRESHOLD_OPTIONS = [
  { value: 0.05, labelKey: 'ai.threshold.loose' },
  { value: 0.15, labelKey: 'ai.threshold.standard' },
  { value: 0.35, labelKey: 'ai.threshold.strict' },
  { value: 0.6, labelKey: 'ai.threshold.ultra' },
]
const classifying = ref<boolean>(false)
const results = ref<ClassifyResult[]>([])
const progress = ref<ClassifyProgress>({
  total: 0,
  processed: 0,
  current_item: undefined,
  percentage: 0,
  current_category: undefined,
})

useProgressListener<ClassifyProgress>('classify-progress', (payload) => {
  progress.value = payload
  // 分类中逐张追加结果，实时展示
  if (payload.result) {
    results.value.push(payload.result)
  }
})

// ===== 人脸聚类模式 =====
const mode = ref<'content' | 'face' | 'hybrid'>('hybrid')
const personGroups = ref<PersonGroup[]>([])
const faceAnalyzed = ref<boolean>(false)
const archiving = ref<boolean>(false)

useProgressListener<ClassifyProgress>('face-cluster-progress', (payload) => {
  progress.value = payload
})

const SETTINGS_KEY = 'leaf-tidy-settings'

interface SettingsState {
  defaultModelId?: string
  customModels?: Array<{ id: string; name: string }>
}

function loadSettings(): SettingsState {
  try {
    return JSON.parse(localStorage.getItem(SETTINGS_KEY) || '{}')
  } catch {
    return {}
  }
}

function saveSettings(patch: Partial<SettingsState>) {
  const current = loadSettings()
  localStorage.setItem(SETTINGS_KEY, JSON.stringify({ ...current, ...patch }))
}

// 内置模型 ID 集合 & 显示名称（computed 以响应语言切换）
const BUILTIN_MODEL_IDS = new Set(['yolov8n-cls', 'yolov8s-cls'])
const BUILTIN_MODEL_NAMES = computed(() => ({
  'yolov8n-cls': `YOLOv8n (${t('ai.model.fast')})`,
  'yolov8s-cls': `YOLOv8s (${t('ai.model.accurate')})`,
}))

const currentSettings = computed(() => loadSettings())

const currentModelName = computed(() => {
  const settings = currentSettings.value
  if (!settings.defaultModelId) return ''

  // 自定义模型名称查找
  for (const m of (settings.customModels || [])) {
    if (m.id === settings.defaultModelId) return m.name
  }

  // 内置模型名称
  return BUILTIN_MODEL_NAMES.value[settings.defaultModelId as keyof typeof BUILTIN_MODEL_NAMES.value] || settings.defaultModelId
})

const isBuiltinModel = computed(() => {
  return BUILTIN_MODEL_IDS.has(currentSettings.value.defaultModelId || '')
})

const modelSource = computed(() => {
  return currentSettings.value.defaultModelId || ''
})

onMounted(() => {
  // 首次使用默认选中 yolov8n-cls
  const settings = loadSettings()
  if (!settings.defaultModelId) {
    saveSettings({ ...settings, defaultModelId: 'yolov8n-cls' })
  }
})

onUnmounted(() => {
  chartInstance?.dispose()
  chartInstance = null
})

const canClassify = computed(() => {
  return modelSource.value.length > 0 && workDirs.value.length > 0
})

const canStart = computed(() => {
  if (mode.value === 'content') return canClassify.value
  return workDirs.value.length > 0
})

const startBtnLabel = computed(() => {
  if (mode.value === 'face') return t('ai.face.start')
  if (mode.value === 'hybrid') return t('ai.mode.hybrid')
  return t('ai.action.start')
})

const progressTitle = computed(() => {
  if (mode.value === 'face') return t('ai.face.progressTitle')
  if (mode.value === 'hybrid') return t('ai.mode.hybrid')
  return t('ai.progress.title')
})

function onStart() {
  if (mode.value === 'face') startFaceCluster()
  else if (mode.value === 'hybrid') startHybrid()
  else startClassify()
}

const { successCount, failCount, successResults, failResults } = useOperationResults(results)

const categoryCounts = computed(() => {
  const counts: Record<string, number> = {}
  for (const r of results.value) {
    if (r.success && r.category) {
      counts[r.category] = (counts[r.category] || 0) + 1
    }
  }
  return counts
})

// 圆环图数据：取前 7 类，其余合并为"其他"（ImageNet 千类不可能全部展示）
const MAX_SLICES = 7
const CHART_COLORS = ['#5B8DEF', '#67C23A', '#E6A23C', '#F56C6C', '#9C6FE4', '#36B5A0', '#D9822B', '#909399']

interface ChartSlice { cat: string; count: number; pct: number }

const chartSlices = computed<ChartSlice[]>(() => {
  const sorted = Object.entries(categoryCounts.value)
    .map(([cat, count]) => ({ cat, count }))
    .sort((a, b) => b.count - a.count)
  const total = sorted.reduce((sum, item) => sum + item.count, 0)
  if (total === 0) return []

  const slices: ChartSlice[] = sorted.slice(0, MAX_SLICES)
    .map(item => ({ cat: item.cat, count: item.count, pct: item.count / total }))
  const restCount = sorted.slice(MAX_SLICES).reduce((sum, item) => sum + item.count, 0)
  if (restCount > 0) {
    slices.push({ cat: t('ai.result.otherCategories'), count: restCount, pct: restCount / total })
  }
  return slices
})

// echarts 圆环图（复用 RightSidebar 的按需引入模式）
const chartContainer = ref<HTMLElement | null>(null)
let chartInstance: echarts.ECharts | null = null

function renderChart() {
  if (!chartContainer.value || chartSlices.value.length === 0) return
  if (!chartInstance) chartInstance = echarts.init(chartContainer.value)
  chartInstance.setOption({
    backgroundColor: 'transparent',
    tooltip: {
      trigger: 'item',
      formatter: (p: { name: string; value: number }) => t('ai.result.categoryCount', { cat: p.name, n: p.value }),
    },
    series: [{
      type: 'pie',
      radius: ['58%', '82%'],
      center: ['50%', '50%'],
      label: { show: false },
      labelLine: { show: false },
      data: chartSlices.value.map(s => ({ name: s.cat, value: s.count })),
    }],
    color: CHART_COLORS,
  }, true)
}

watch(chartSlices, (slices) => {
  if (slices.length > 0) nextTick(renderChart)
})

async function stopClassify() {
  try {
    await invoke('cancel_operation')
    ElNotification({ type: 'info', title: t('photos.common.tip'), message: t('ai.cancelled') })
  } catch (e: any) {
    ElNotification({ type: 'error', title: t('photos.common.error'), message: String(e) })
  } finally {
    classifying.value = false
    isProcessing.value = false
  }
}

async function collectImageFiles(): Promise<string[]> {
  const imageExts = /\.(jpg|jpeg|png|bmp|webp|tiff?)$/i
  const files: string[] = []

  const sourceDirs = workDirs.value.map(d => d.path)
  await useLog().logDirectoryOperation('AI分类-读取目录', sourceDirs)
  for (const dir of workDirs.value) {
    try {
      const entries = await invoke<{ path: string; is_dir: boolean }[]>('scan_directory', { path: dir.path })
      const imageFiles = entries
        .filter(e => !e.is_dir && imageExts.test(e.path))
        .map(e => e.path)
      files.push(...imageFiles)
    } catch (e) {
      // 扫描失败时跳过该目录
    }
  }

  return files
}

// 同步标志位，防止双击（Vue 响应式更新是微任务，无法阻止同事件循环内的第二次点击）
let startGuard = false

async function startClassify() {
  // 双重检查：同步标志 + 响应式状态
  if (startGuard || classifying.value || isProcessing.value) {
    return
  }
  startGuard = true

  if (!canClassify.value) {
    ElNotification({ type: 'warning', title: t('photos.common.warning'), message: t('ai.noModel') })
    startGuard = false
    return
  }

  classifying.value = true
  isProcessing.value = true
  results.value = []
  progress.value = {
    total: 0,
    processed: 0,
    current_item: undefined,
    percentage: 0,
    current_category: undefined,
  }

  const imageFiles = await collectImageFiles()

  if (imageFiles.length === 0) {
    ElNotification({ type: 'warning', title: t('photos.common.warning'), message: t('ai.noImages') })
    classifying.value = false
    isProcessing.value = false
    return
  }

  await useLog().logOperationStart(t('ai.log.contentClassify'), {
    [t('log.field.model')]: currentModelName.value || modelSource.value,
    [t('log.field.confidence')]: CONFIDENCE_THRESHOLD.value,
    [t('log.field.imageCount')]: imageFiles.length,
    [t('log.field.outputDir')]: outputDir.value || t('log.field.unset'),
  })

  try {
    const rule: ClassifyRule = {
      confidence_threshold: CONFIDENCE_THRESHOLD.value,
    }

    const isBuiltin = isBuiltinModel.value
    const source = modelSource.value

    const res = await invoke<ClassifyResult[]>('classify_images_async', {
      modelSource: source,
      isBuiltin,
      imagePaths: imageFiles,
      rule,
      outputDir: outputDir.value,
    })
    results.value = res

    const success = res.filter(r => r.success).length
    const fail = res.filter(r => !r.success).length

    if (fail === 0) {
      ElNotification({ type: 'success', title: t('photos.common.success'), message: t('ai.doneAll', { n: success }) })
    } else {
      ElNotification({ type: 'warning', title: t('photos.common.warning'), message: t('ai.donePartial', { s: success, f: fail }) })
    }
  } catch (e: any) {
    if (isCancelledError(e)) {
      ElNotification({ type: 'info', title: t('photos.common.tip'), message: t('ai.cancelled') })
    } else {
      ElNotification({ type: 'error', title: t('photos.common.error'), message: t('ai.failed', { e: String(e) }) })
    }
  } finally {
    classifying.value = false
    isProcessing.value = false
    startGuard = false
  }
}

// ===== 人脸聚类逻辑 =====

// 同步标志位，防止双击
let faceStartGuard = false

async function startFaceCluster() {
  if (faceStartGuard || classifying.value || isProcessing.value) {
    return
  }
  faceStartGuard = true

  if (workDirs.value.length === 0) {
    ElNotification({ type: 'warning', title: t('photos.common.warning'), message: t('dup.selectDirFirst') })
    faceStartGuard = false
    return
  }

  classifying.value = true
  isProcessing.value = true
  personGroups.value = []
  faceAnalyzed.value = false
  progress.value = {
    total: 0,
    processed: 0,
    current_item: undefined,
    percentage: 0,
    current_category: undefined,
  }

  const dirPaths = workDirs.value.map(d => d.path)
  await useLog().logOperationStart(t('ai.log.faceCluster'), {
    [t('log.field.scanDir')]: dirPaths.join(', '),
  })

  try {
    // 后端自行遍历目录收集图片
    const res = await invoke<PersonGroup[]>('face_cluster_async', {
      paths: dirPaths,
    })
    personGroups.value = res
    faceAnalyzed.value = true
    ElNotification({ type: 'success', title: t('photos.common.success'), message: t('ai.face.done', { n: res.length }) })
  } catch (e: any) {
    if (isCancelledError(e)) {
      ElNotification({ type: 'info', title: t('photos.common.tip'), message: t('ai.cancelled') })
    } else {
      ElNotification({ type: 'error', title: t('photos.common.error'), message: t('ai.face.failed', { e: String(e) }) })
    }
  } finally {
    classifying.value = false
    isProcessing.value = false
    faceStartGuard = false
  }
}

// 同步标志位，防止双击
let hybridStartGuard = false

async function startHybrid() {
  if (hybridStartGuard || classifying.value || isProcessing.value) return
  hybridStartGuard = true

  if (!canClassify.value || workDirs.value.length === 0) {
    ElNotification({ type: 'warning', title: t('photos.common.warning'), message: t('ai.noModel') })
    hybridStartGuard = false
    return
  }

  classifying.value = true
  isProcessing.value = true
  personGroups.value = []
  results.value = []
  faceAnalyzed.value = false
  progress.value = { total: 0, processed: 0, current_item: undefined, percentage: 0, current_category: undefined }

  const dirPaths = workDirs.value.map(d => d.path)
  const imageFiles = await collectImageFiles()

  if (imageFiles.length === 0) {
    ElNotification({ type: 'warning', title: t('photos.common.warning'), message: t('ai.noImages') })
    classifying.value = false
    isProcessing.value = false
    hybridStartGuard = false
    return
  }

  await useLog().logOperationStart(t('ai.log.hybridClassify'), {
    [t('log.field.model')]: currentModelName.value || modelSource.value,
    [t('log.field.confidence')]: CONFIDENCE_THRESHOLD.value,
    [t('log.field.imageCount')]: imageFiles.length,
    [t('log.field.outputDir')]: outputDir.value || t('log.field.unset'),
  })

  try {
    // 第一步：人脸聚类
    const faceRes = await invoke<PersonGroup[]>('face_cluster_async', { paths: dirPaths })
    personGroups.value = faceRes
    faceAnalyzed.value = true

    // 收集有人脸的图片路径
    const facePaths = new Set<string>()
    for (const g of faceRes) {
      for (const p of g.photos) facePaths.add(p.path)
    }

    // 第二步：对无人脸图片做内容分类
    const nonFaceImages = imageFiles.filter(p => !facePaths.has(p))
    if (nonFaceImages.length > 0) {
      const rule: ClassifyRule = { confidence_threshold: CONFIDENCE_THRESHOLD.value }
      const isBuiltin = isBuiltinModel.value
      const source = modelSource.value
      const clsRes = await invoke<ClassifyResult[]>('classify_images_async', {
        modelSource: source,
        isBuiltin,
        imagePaths: nonFaceImages,
        rule,
        outputDir: outputDir.value,
      })
      results.value = clsRes
    }

    const faceCount = facePaths.size
    const clsSuccess = results.value.filter(r => r.success).length
    ElNotification({
      type: 'success',
      title: t('photos.common.success'),
      message: `${t('ai.mode.hybrid')} - ${t('ai.result.hybrid', { faceCount, clsCount: clsSuccess })}`,
    })
  } catch (e: any) {
    if (isCancelledError(e)) {
      ElNotification({ type: 'info', title: t('photos.common.tip'), message: t('ai.cancelled') })
    } else {
      ElNotification({ type: 'error', title: t('photos.common.error'), message: String(e) })
    }
  } finally {
    classifying.value = false
    isProcessing.value = false
    hybridStartGuard = false
  }
}

async function renamePerson(group: PersonGroup) {
  try {
    const { value } = await ElMessageBox.prompt(t('ai.face.renamePrompt'), t('ai.face.rename'), {
      inputValue: group.name,
      confirmButtonText: t('common.confirm'),
      cancelButtonText: t('common.cancel'),
    })
    const name = (value || '').trim()
    if (!name || name === group.name) return
    const persons = await invoke<Person[]>('rename_person', { id: group.id, newName: name })
    const updated = persons.find(p => p.id === group.id)
    group.name = updated?.name ?? name
    group.confirmed = true
    useLog().logEvent(t('ai.log.personRename', { name }))
  } catch (e) {
    if (e !== 'cancel' && e !== 'close') {
      ElNotification({ type: 'error', title: t('photos.common.error'), message: String(e) })
    }
  }
}

const mergeDialogVisible = ref<boolean>(false)
const mergeSource = ref<PersonGroup | null>(null)
const mergeTargetId = ref<string>('')
const mergeCandidates = computed(() => {
  return mergeSource.value ? personGroups.value.filter(g => g.id !== mergeSource.value!.id) : []
})

function openMergeDialog(group: PersonGroup) {
  mergeSource.value = group
  mergeTargetId.value = ''
  mergeDialogVisible.value = true
}

async function confirmMerge() {
  const source = mergeSource.value
  const target = personGroups.value.find(g => g.id === mergeTargetId.value)
  if (!source || !target) return
  try {
    const persons = await invoke<Person[]>('merge_persons', { targetId: target.id, sourceId: source.id })
    // 本地分组同步：照片并入目标组，移除来源组
    target.photos.push(...source.photos)
    target.photos.sort((a, b) => b.similarity - a.similarity)
    const updated = persons.find(p => p.id === target.id)
    if (updated) {
      target.name = updated.name
      target.confirmed = updated.confirmed
    }
    personGroups.value = personGroups.value.filter(g => g.id !== source.id)
    mergeDialogVisible.value = false
    useLog().logEvent(t('ai.log.personMerge', { source: source.name, target: target.name }))
  } catch (e: any) {
    ElNotification({ type: 'error', title: t('photos.common.error'), message: String(e) })
  }
}

async function archiveByPerson() {
  if (!outputDir.value) {
    ElNotification({ type: 'warning', title: t('photos.common.warning'), message: t('ai.face.archiveNeedOutDir') })
    return
  }
  const groups = personGroups.value.map(g => ({ name: g.name, paths: g.photos.map(p => p.path) }))
  if (groups.length === 0) return

  archiving.value = true
  try {
    const res = await invoke<BatchOperationResult>('archive_by_person', {
      outputDir: outputDir.value,
      groups,
    })
    if (res.fail_count === 0) {
      ElNotification({ type: 'success', title: t('photos.common.success'), message: t('ai.face.archived', { n: res.success_count }) })
    } else {
      ElNotification({ type: 'warning', title: t('photos.common.warning'), message: t('ai.face.archived', { n: res.success_count }) })
    }
    useLog().logEvent(t('ai.log.personArchive', { success: res.success_count, total: res.total, dir: outputDir.value }))
  } catch (e: any) {
    if (isCancelledError(e)) {
      ElNotification({ type: 'info', title: t('photos.common.tip'), message: t('ai.cancelled') })
    } else {
      ElNotification({ type: 'error', title: t('photos.common.error'), message: String(e) })
    }
  } finally {
    archiving.value = false
  }
}


</script>

<style scoped>
/* Page-specific styles (shared styles in components.css) */

.model-display {
  display: flex;
  align-items: center;
  gap: 12px;
}

.model-display-label {
  font-size: 13px;
  color: var(--text-secondary);
}

.model-display-name {
  font-size: 13px;
  color: var(--text);
  font-weight: 500;
}

.model-display-hint {
  font-size: 12px;
  color: var(--text-muted);
  margin-left: auto;
}

.threshold-item {
  display: flex;
  align-items: center;
  gap: 12px;
}

.threshold-select {
  width: 180px;
}

.current-cat {
  color: var(--primary);
  font-weight: 500;
}

.category-summary {
  display: flex;
  align-items: center;
  gap: 32px;
  margin-bottom: 16px;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--border);
}

.donut-chart {
  width: 150px;
  height: 150px;
  flex-shrink: 0;
}

.donut-legend {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.legend-row {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 13px;
  line-height: 20px;
}

.legend-dot {
  width: 10px;
  height: 10px;
  border-radius: 3px;
  flex-shrink: 0;
}

.legend-name {
  color: var(--text);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.legend-pct {
  margin-left: auto;
  color: var(--text-secondary);
  flex-shrink: 0;
}

.legend-count {
  color: var(--text-muted);
  min-width: 32px;
  text-align: right;
  flex-shrink: 0;
}

.result-list {
  gap: 6px;
  max-height: 200px;
}

.result-name {
  max-width: 200px;
}

.result-category {
  font-size: 12px;
  color: var(--primary);
  background: rgba(var(--primary-rgb), 0.1);
  padding: 2px 8px;
  border-radius: 4px;
}

.result-confidence {
  font-size: 11px;
  color: var(--text-muted);
}

/* ===== 人物分组 ===== */

.face-empty {
  padding: 24px 0;
  text-align: center;
  font-size: 13px;
  color: var(--text-muted);
}

.person-group {
  padding: 12px 0;
  border-bottom: 1px solid var(--border);
}

.person-group:last-child {
  border-bottom: none;
}

.person-header {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 10px;
}

.person-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text);
}

.person-count {
  font-size: 12px;
  color: var(--text-muted);
}

.person-actions {
  margin-left: auto;
  display: flex;
  align-items: center;
}

.face-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.face-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 4px 8px 4px 4px;
  border-radius: 8px;
  max-width: 220px;
}

.face-item:hover {
  background: rgba(var(--primary-rgb), 0.06);
}

.face-thumb {
  width: 40px;
  height: 40px;
  border-radius: 6px;
  object-fit: cover;
  flex-shrink: 0;
}

.face-name {
  font-size: 12px;
  color: var(--text-secondary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.face-sim {
  font-size: 11px;
  color: var(--text-muted);
  flex-shrink: 0;
}

.merge-row {
  display: flex;
  align-items: center;
  gap: 12px;
}

.merge-label {
  font-size: 13px;
  color: var(--text-secondary);
  flex-shrink: 0;
}
</style>
