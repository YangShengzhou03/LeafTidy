<template>
  <div class="page-container">
    <div class="page-header">
      <h2>批量重命名</h2>
      <p class="desc">用灵活模板批量修改文件名，让文件命名更规范</p>
    </div>
    <div class="page-content">
      <div class="config-panel">
        <div class="config-row">
          <div class="config-item">
            <label>时间数据来源</label>
            <el-select v-model="timeSource" placeholder="选择时间来源" style="width: 150px">
              <el-option label="修改时间" value="modified" />
              <el-option label="创建时间" value="created" />
              <el-option label="拍摄日期" value="taken" />
            </el-select>
            <span class="hint">用于日期、年份、月份、时间等标签</span>
          </div>
          <div class="config-item">
            <label>序号起始值</label>
            <el-input-number v-model="startIndex" :min="1" :max="9999" controls-position="right" style="width: 120px" />
          </div>
        </div>
        <div class="config-item">
          <label>命名标签（点击添加，可重复选择）</label>
          <div class="tag-selector">
            <div class="tag-grid">
              <div v-for="tag in availableTags" :key="tag.value" class="tag-chip" @click="addTag(tag.value)">
                <el-icon>
                  <component :is="tag.icon" />
                </el-icon>
                <span>{{ tag.label }}</span>
              </div>
            </div>
            <div class="separator-grid">
              <div v-for="sep in separators" :key="sep.value" class="sep-chip" @click="addSeparator(sep.value)">
                <span>{{ sep.label }}</span>
              </div>
            </div>
          </div>
        </div>
        <div class="config-item" v-if="templateParts.length > 0">
          <label>当前模板（点击移除）</label>
          <div class="template-builder">
            <div class="template-segments">
              <div v-for="(part, index) in templateParts" :key="index" class="template-part"
                :class="{ separator: part.type === 'separator' }" @click="removePart(index)">
                <span v-if="part.type === 'separator'" class="part-sep">{{ part.value }}</span>
                <span v-else class="part-tag">{{ part.label }}</span>
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
          <span class="progress-title">正在重命名文件</span>
          <span class="progress-percent">{{ progress.percentage.toFixed(1) }}%</span>
        </div>
        <el-progress :percentage="progress.percentage" :stroke-width="12" :show-text="false" class="progress-bar" />
        <div class="progress-stats">
          <span>总数: {{ progress.total }}</span>
          <span>已处理: {{ progress.processed }}</span>
        </div>
        <div class="progress-current" v-if="progress.current_item">
          <span class="current-label">当前:</span>
          <span class="current-file">{{ progress.current_item }}</span>
        </div>
      </div>

      <div class="action-bar">
        <el-button v-if="!renaming" type="primary" :disabled="templateParts.length === 0 || workDirs.length === 0"
          @click="startRename">
          开始重命名
        </el-button>
        <el-button v-else type="danger" @click="stopRename">
          终止重命名
        </el-button>
      </div>
      <div class="result-panel" v-if="results.length > 0">
        <div class="result-header">
          <span class="result-title">重命名结果</span>
          <span class="result-stats">
            成功: {{ successCount }} / 失败: {{ failCount }}
          </span>
        </div>
        <div class="result-section" v-if="failResults.length > 0">
          <div class="section-label fail-label">
            <el-icon>
              <CircleCloseFilled />
            </el-icon>
            <span>失败 {{ failCount }} 个</span>
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
            <span>成功 {{ successCount }} 个</span>
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
import { ElMessage, ElNotification } from 'element-plus'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { Document, Calendar, Clock, Camera, VideoCamera, Sort, Close, SuccessFilled, CircleCloseFilled, MapLocation, Location, Files, Coin } from '@element-plus/icons-vue'
import { useFileOps } from '@/composables/useFileOps'
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
    // ignore parse errors
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
    // ignore storage errors
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
  { value: 'date', label: '日期', icon: Calendar },
  { value: 'time', label: '时间', icon: Clock },
  { value: 'year', label: '年份', icon: Calendar },
  { value: 'month', label: '月份', icon: Clock },
  { value: 'day', label: '日', icon: Calendar },
  { value: 'type', label: '文件类型', icon: Document },
  { value: 'name', label: '原文件名', icon: Document },
  { value: 'ext', label: '扩展名', icon: Files },
  { value: 'index', label: '序号', icon: Sort },
  { value: 'province', label: '省份', icon: MapLocation },
  { value: 'city', label: '城市', icon: Location },
  { value: 'district', label: '区县', icon: Location },
  { value: 'place', label: '地点', icon: Location },
  { value: 'make', label: '相机品牌', icon: Camera },
  { value: 'model', label: '相机型号', icon: VideoCamera },
  { value: 'exact_size', label: '文件大小', icon: Coin },
]

const separators = [
  { value: '_', label: '_' },
  { value: '-', label: '-' },
  { value: ' ', label: '空格' },
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
    type: '图片',
    name: 'photo',
    ext: 'jpg',
    index: String(startIndex.value).padStart(3, '0'),
    province: '江西省',
    city: '南昌市',
    district: '红谷滩区',
    place: '江西科技师范大学',
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
  return result || '点击标签构建模板'
})

const successCount = computed(() => results.value.filter(r => r.success).length)
const failCount = computed(() => results.value.filter(r => !r.success).length)
const successResults = computed(() => results.value.filter(r => r.success))
const failResults = computed(() => results.value.filter(r => !r.success))

async function stopRename() {
  try {
    await invoke<CancelResult>('cancel_operation')
    ElNotification({ type: 'info', title: '提示', message: '正在终止重命名操作...' })
  } catch (e: any) {
    ElNotification({ type: 'error', title: '错误', message: `终止失败: ${e}` })
  }
}

async function startRename() {
  if (templateParts.value.length === 0) {
    ElNotification({ type: 'warning', title: '警告', message: '请先构建命名模板' })
    return
  }
  if (workDirs.value.length === 0) {
    ElNotification({ type: 'warning', title: '警告', message: '请先选择待处理目录' })
    return
  }
  if (!outputDir.value) {
    ElNotification({ type: 'warning', title: '警告', message: '请先选择输出目录' })
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

  try {
    const paths: string[] = []
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
    // 写入日志
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
        ElNotification({ type: 'success', title: '成功', message: `重命名完成，共处理 ${success} 个文件，已复制到输出目录` })
      } else {
        ElNotification({ type: 'warning', title: '警告', message: `重命名完成，成功 ${success} 个，失败 ${fail} 个` })
      }
    } else {
      ElNotification({ type: 'info', title: '提示', message: '没有找到需要重命名的文件' })
    }
  } catch (e: any) {
    const errMsg = String(e)
    if (errMsg.includes('取消')) {
      const sourcePathsStr = files.value.map(f => f.path).join(', ')
      await logCancelledOperation('rename', sourcePathsStr, `源文件数: ${files.value.length}`)
      ElNotification({ type: 'info', title: '提示', message: '重命名操作已取消' })
    } else {
      ElNotification({ type: 'error', title: '错误', message: `重命名失败: ${errMsg}` })
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
  color: #C8D0DC;
  margin-bottom: 12px;
  font-weight: 500;
}

.config-item .hint {
  font-size: 12px;
  color: #8A94A6;
  margin-left: 12px;
}

.tag-selector {
  background: #2A2B30;
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
  background: #1F2023;
  border-radius: 6px;
  border: 1px solid #3A3B40;
  cursor: pointer;
  transition: background 0.15s;
}

.tag-chip:hover {
  background: #353639;
}

.tag-chip .el-icon {
  font-size: 14px;
  color: #C8D0DC;
}

.tag-chip span {
  font-size: 13px;
  color: #C8D0DC;
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
  background: #1F2023;
  border-radius: 6px;
  border: 1px solid #3A3B40;
  cursor: pointer;
  transition: background 0.15s;
}

.sep-chip:hover {
  background: #353639;
}

.sep-chip span {
  font-size: 13px;
  color: #C8D0DC;
}

.template-builder {
  background: #2A2B30;
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
  background: #3A86FF;
  border-radius: 6px;
  cursor: pointer;
  user-select: none;
  transition: opacity 0.15s;
}

.template-part.separator {
  background: #5A5F6A;
}

.template-part:hover {
  opacity: 0.8;
}

.part-tag {
  font-size: 13px;
  color: #FFFFFF;
}

.part-sep {
  font-size: 13px;
  color: #E0E6ED;
}

.part-remove {
  font-size: 14px;
  color: rgba(255, 255, 255, 0.7);
}

.template-preview {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px;
  background: #1F2023;
  border-radius: 6px;
  border: 1px dashed #3A3B40;
}

.template-preview .el-icon {
  font-size: 16px;
  color: #8A94A6;
}

.preview-text {
  font-size: 13px;
  color: #3A86FF;
  word-break: break-all;
}

.ext-hint {
  color: #8A94A6;
  font-size: 13px;
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
  margin-bottom: 20px;
}

.result-panel {
  background: #1F2023;
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
  background: rgba(232, 17, 35, 0.15);
  color: #E81123;
}

.success-label {
  background: rgba(82, 196, 26, 0.15);
  color: #52C41A;
}

.result-title {
  font-size: 13px;
  font-weight: 500;
  color: #C8D0DC;
}

.result-stats {
  font-size: 12px;
  color: #8A94A6;
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
  background: #2A2B30;
  border-radius: 6px;
}

.result-item .el-icon {
  font-size: 14px;
  flex-shrink: 0;
}

.result-item.success .el-icon {
  color: #52C41A;
}

.result-item.fail .el-icon {
  color: #E81123;
}

.result-name {
  font-size: 12px;
  color: #E0E6ED;
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
  color: #3A86FF;
}

.result-arrow {
  font-size: 12px;
  color: #8A94A6;
  flex-shrink: 0;
}

.result-target {
  font-size: 12px;
  color: #3A86FF;
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
  color: #E81123;
  flex-shrink: 0;
  margin-left: auto;
}
</style>
