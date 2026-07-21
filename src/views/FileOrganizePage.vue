<template>
  <div class="page-container">
    <div class="page-header">
      <h2>文件整理</h2>
      <p class="desc">按类型、日期、大小等规则自动整理文件到目标目录</p>
    </div>
    <div class="page-content">
      <div class="config-panel">
        <div class="config-item">
          <label>时间数据来源</label>
          <el-select v-model="timeSource" placeholder="选择时间来源" style="width: 150px">
            <el-option label="修改时间" value="modified" />
            <el-option label="创建时间" value="created" />
            <el-option label="拍摄日期" value="taken" />
          </el-select>
          <span class="hint">用于年份、月份、日期分类</span>
        </div>
        <div class="config-item">
          <label>分类标签（点击添加，可重复选择）</label>
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
          <label>文件夹层级（点击移除）</label>
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
          <span class="progress-title">正在整理文件</span>
          <span class="progress-percent">{{ progress.percentage.toFixed(1) }}%</span>
        </div>
        <el-progress :percentage="progress.percentage" :stroke-width="12" :show-text="false" class="progress-bar" />
        <div class="progress-stats">
          <span>总数: {{ progress.total }}</span>
          <span>已处理: {{ progress.processed }}</span>
          <span style="color: #52C41A;">成功: {{ progress.success_count }}</span>
          <span style="color: #E81123;">失败: {{ progress.fail_count }}</span>
        </div>
        <div class="progress-current" v-if="progress.current_file">
          <span class="current-label">当前:</span>
          <span class="current-file">{{ progress.current_file }}</span>
        </div>
      </div>

      <div class="action-bar">
        <el-button v-if="!organizing" type="primary" @click="startOrganize" :disabled="!canOrganize">
          开始整理
        </el-button>
        <el-button v-else type="danger" @click="stopOrganize">
          终止整理
        </el-button>
      </div>
      <div class="result-panel" v-if="results.length > 0">
        <div class="result-header">
          <span class="result-title">整理结果</span>
          <span class="result-stats">
            成功: {{ successCount }} / 失败: {{ failCount }}
          </span>
        </div>
        <!-- 失败结果 -->
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
import { ElMessage, ElNotification } from 'element-plus'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { FolderOpened, Document, Calendar, Clock, MapLocation, Location, Camera, VideoCamera, Files, Coin, Close, SuccessFilled, CircleCloseFilled } from '@element-plus/icons-vue'
import { useFileOps } from '@/composables/useFileOps'
import { useLog, type OrganizeLogOptions, type FileResult } from '@/composables/useLog'
import type { WorkDirectory, OrganizeResult, OrganizeProgress, CancelResult } from '@/types'

const workDirs = inject<Ref<WorkDirectory[]>>('workDirs')!
const outputDir = inject<Ref<string>>('outputDir')!
const isProcessing = inject<Ref<boolean>>('isProcessing')!

const { getFileName, openFile, getRelativePath } = useFileOps()
const { logOrganizeResults, logCancelledOperation } = useLog()

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

let unlistenProgress: UnlistenFn | null = null

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
    // ignore parse errors
  }
}

function saveState() {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify({
      selectedTags: selectedTags.value,
      timeSource: timeSource.value,
    }))
  } catch {
    // ignore storage errors
  }
}

onMounted(async () => {
  loadState()
  // 监听进度事件
  unlistenProgress = await listen<OrganizeProgress>('organize-progress', (event) => {
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
  { value: 'type', label: '文件类型', icon: Document },
  { value: 'year', label: '年份', icon: Calendar },
  { value: 'month', label: '月份', icon: Clock },
  { value: 'day', label: '日', icon: Calendar },
  { value: 'date', label: '日期', icon: Calendar },
  { value: 'province', label: '省份', icon: MapLocation },
  { value: 'city', label: '城市', icon: Location },
  { value: 'district', label: '区县', icon: Location },
  { value: 'place', label: '地点', icon: Location },
  { value: 'make', label: '相机品牌', icon: Camera },
  { value: 'model', label: '相机型号', icon: VideoCamera },
  { value: 'ext', label: '扩展名', icon: Files },
  { value: 'size', label: '文件大小', icon: Coin },
]

function getTagLabel(value: string): string {
  const tag = availableTags.find(t => t.value === value)
  return tag ? tag.label : value
}

function addTag(value: string) {
  selectedTags.value.push(value)
}

function removeTag(index: number) {
  selectedTags.value.splice(index, 1)
}

const previewPath = computed(() => {
  const examples: Record<string, string> = {
    type: '图片',
    year: '2024',
    month: '01',
    day: '15',
    date: '2024-01-15',
    province: '浙江省',
    city: '杭州市',
    district: '滨江区',
    place: '星民村',
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

const successCount = computed(() => results.value.filter(r => r.success).length)
const failCount = computed(() => results.value.filter(r => !r.success).length)
const successResults = computed(() => results.value.filter(r => r.success))
const failResults = computed(() => results.value.filter(r => !r.success))

async function stopOrganize() {
  try {
    await invoke<CancelResult>('cancel_operation')
    ElNotification({ type: 'info', title: '提示', message: '正在终止整理操作...' })
  } catch (e: any) {
    ElNotification({ type: 'error', title: '错误', message: `终止失败: ${e}` })
  }
}

async function startOrganize() {
  if (!canOrganize.value) {
    ElNotification({ type: 'warning', title: '警告', message: '请先选择待处理目录、输出目录和分类标签' })
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
    // 使用异步版本的命令
    const res = await invoke<OrganizeResult[]>('organize_files_async', {
      sourceDirs,
      targetDir: outputDir.value,
      rule: {
        tags: selectedTags.value,
        time_source: timeSource.value,
      },
    })
    results.value = res

    // 写入日志，包含用户设置参数和元数据
    const logOptions: OrganizeLogOptions = {
      timeSource: timeSource.value,
      tags: selectedTags.value,
      sourceDirs,
      targetDir: outputDir.value,
    }

    // 转换为 FileResult 格式，包含元数据
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
        ElNotification({ type: 'success', title: '成功', message: `整理完成，共处理 ${success} 个文件` })
      } else {
        ElNotification({ type: 'warning', title: '警告', message: `整理完成，成功 ${success} 个，失败 ${fail} 个` })
      }
    } else {
      ElNotification({ type: 'info', title: '提示', message: '没有找到需要整理的文件' })
    }
  } catch (e: any) {
    const errMsg = String(e)
    if (errMsg.includes('用户已取消')) {
      const sourceDirsStr = workDirs.value.map(d => d.path).join(', ')
      await logCancelledOperation('organize', sourceDirsStr, `源目录: ${sourceDirsStr}`)
      ElNotification({ type: 'info', title: '提示', message: '整理操作已取消' })
    } else {
      ElNotification({ type: 'error', title: '错误', message: `整理失败: ${errMsg}` })
    }
  } finally {
    organizing.value = false
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

.path-builder {
  background: #2A2B30;
  border-radius: 8px;
  padding: 16px;
}

.path-segments {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-bottom: 12px;
}

.path-segment {
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

.path-segment:hover {
  opacity: 0.8;
}

.segment-order {
  font-size: 11px;
  color: rgba(255, 255, 255, 0.7);
  background: rgba(255, 255, 255, 0.2);
  padding: 2px 6px;
  border-radius: 4px;
}

.segment-name {
  font-size: 13px;
  color: #FFFFFF;
}

.segment-remove {
  font-size: 14px;
  color: rgba(255, 255, 255, 0.7);
}

.path-preview {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px;
  background: #1F2023;
  border-radius: 6px;
  border: 1px dashed #3A3B40;
}

.path-preview .el-icon {
  font-size: 16px;
  color: #8A94A6;
}

.preview-text {
  font-size: 13px;
  color: #8A94A6;
  word-break: break-all;
}

.source-info,
.target-info {
  background: #1F2023;
  border-radius: 8px;
  padding: 16px;
  margin-bottom: 16px;
}

.info-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.info-title {
  font-size: 13px;
  font-weight: 500;
  color: #C8D0DC;
}

.info-count {
  font-size: 12px;
  color: #8A94A6;
}

.info-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.info-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 12px;
  background: #2A2B30;
  border-radius: 6px;
}

.info-item .el-icon {
  font-size: 16px;
  color: #3A86FF;
}

.info-name {
  font-size: 13px;
  color: #E0E6ED;
  font-weight: 500;
}

.info-path {
  font-size: 12px;
  color: #8A94A6;
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.info-content {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 12px;
  background: #2A2B30;
  border-radius: 6px;
}

.info-content .el-icon {
  font-size: 16px;
  color: #3A86FF;
}

.empty-hint {
  display: flex;
  align-items: center;
  gap: 8px;
  color: #8A94A6;
  font-size: 13px;
}

.empty-hint .el-icon {
  font-size: 16px;
  color: #E81123;
}

.action-bar {
  display: flex;
  justify-content: flex-end;
  margin-bottom: 20px;
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