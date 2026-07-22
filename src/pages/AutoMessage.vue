<template>
  <div class="auto-message-page">
    <div class="page-header">
      <h1 class="page-title">自动消息</h1>
      <p class="page-subtitle">管理定时发送和循环发送的任务</p>
    </div>

    <div class="divider"></div>

    <div class="toolbar">
      <div class="toolbar-left">
        <el-input v-model="searchKeyword" placeholder="搜索任务" clearable>
          <template #prefix>
            <el-icon><Search /></el-icon>
          </template>
        </el-input>

        <el-select v-model="filterType" placeholder="任务类型" clearable>
          <el-option label="定时任务" value="定时" />
          <el-option label="循环任务" value="间隔" />
        </el-select>
      </div>

      <div class="toolbar-right">
        <el-button
          v-if="!taskStore.isExecuting"
          type="primary"
          @click="handleStartExecution"
        >
          开始执行
        </el-button>
        <el-button
          v-else
          type="danger"
          @click="handleStopExecution"
        >
          终止执行
        </el-button>
        <el-button @click="handleCreateTask">新建任务</el-button>
        <el-button @click="handleImportExcel">导入配置</el-button>
        <el-button type="danger" @click="handleClearAll">清空</el-button>
      </div>
    </div>

    <div class="table-wrapper">
      <el-table :data="filteredTasks" @selection-change="handleSelectionChange">
        <el-table-column type="selection" width="55" />

        <el-table-column prop="recipient" label="接收对象" width="120">
          <template #default="{ row }">
            <div class="text-ellipsis" :title="row.recipient">{{ row.recipient }}</div>
          </template>
        </el-table-column>

        <el-table-column prop="content" label="发送内容" min-width="200">
          <template #default="{ row }">
            <div v-if="row.contentType === 'text'" class="text-ellipsis" :title="row.content">{{ row.content }}</div>
            <div v-else class="text-ellipsis" :title="row.fileName">{{ row.fileName }}</div>
          </template>
        </el-table-column>

        <el-table-column prop="type" label="类型" width="100">
          <template #default="{ row }">
            <div class="text-ellipsis">
              <span class="task-type-tag">{{ row.type }}</span>
            </div>
          </template>
        </el-table-column>

        <el-table-column prop="schedule" label="执行时间" width="180">
          <template #default="{ row }">
            <div class="text-ellipsis" :title="row.type === '定时' ? formatTime(row.executeTime) : `间隔 ${row.interval} 分钟`">
              <div v-if="row.type === '定时'">{{ formatTime(row.executeTime) }}</div>
              <div v-else>间隔 {{ row.interval }} 分钟</div>
            </div>
          </template>
        </el-table-column>

        <el-table-column prop="nextExecute" label="下次执行" width="180">
          <template #default="{ row }">
            <div class="text-ellipsis" :title="formatTime(row.nextExecute)">{{ formatTime(row.nextExecute) }}</div>
          </template>
        </el-table-column>

        <el-table-column prop="executeCount" label="执行次数" width="100">
          <template #default="{ row }">
            <div class="text-ellipsis">
              <span class="execute-count">{{ row.executeCount }} 次</span>
            </div>
          </template>
        </el-table-column>

        <el-table-column prop="status" label="任务状态" width="100">
          <template #default="{ row }">
            <div class="text-ellipsis">
              <div class="task-status" :class="row.status">
                <span class="status-dot"></span>
                <span class="status-text">{{ getStatusText(row.status) }}</span>
              </div>
            </div>
          </template>
        </el-table-column>

        <el-table-column prop="enabled" label="启用状态" width="100">
          <template #default="{ row }">
            <el-switch v-model="row.enabled" @change="handleToggleTask(row)" />
          </template>
        </el-table-column>

        <el-table-column label="操作" width="150" fixed="right">
          <template #default="{ row }">
            <el-button link type="primary" @click="handleEditTask(row)">编辑</el-button>
            <el-button link type="danger" @click="handleDeleteTask(row)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>

      <div class="table-footer">
        <div>
          已选择 {{ selectedTasks.length }} 项
          <el-button v-if="selectedTasks.length > 0" link type="danger" @click="handleBatchDelete">
            批量删除
          </el-button>
        </div>

        <el-pagination
          v-model:current-page="currentPage"
          v-model:page-size="pageSize"
          :page-sizes="[10, 20, 50, 100]"
          layout="total, sizes, prev, pager, next, jumper"
          :total="total"
        />
      </div>
    </div>

    <el-dialog v-model="taskDialogVisible" :title="editingTask ? '编辑任务' : '新建任务'" width="600px">
      <el-form :model="taskForm" label-width="100px" :rules="taskRules">
        <el-form-item label="接收对象" prop="recipient">
          <el-input v-model="taskForm.recipient" placeholder="微信好友或群聊备注名称">
            <template #append>
              <el-button @click="handleSelectContact">选择联系人</el-button>
            </template>
          </el-input>
        </el-form-item>

        <el-form-item label="发送类型">
          <el-radio-group v-model="taskForm.contentType">
            <el-radio value="text">文字消息</el-radio>
            <el-radio value="file">文件</el-radio>
          </el-radio-group>
        </el-form-item>

        <el-form-item v-if="taskForm.contentType === 'text'" label="消息内容" prop="content">
          <el-input v-model="taskForm.content" type="textarea" :rows="4" placeholder="输入消息内容" />
          <div style="margin-top: 8px;">
            <el-checkbox v-model="taskForm.autoSplit">智能拆句(长消息自动拆分)</el-checkbox>
          </div>
        </el-form-item>

        <el-form-item v-else label="选择文件" prop="fileName">
          <el-input v-model="taskForm.fileName" placeholder="选择要发送的文件">
            <template #append>
              <el-button @click="handleSelectFile">选择文件</el-button>
            </template>
          </el-input>
        </el-form-item>

        <el-form-item label="执行模式">
          <el-radio-group v-model="taskForm.type">
            <el-radio value="定时">定时发送</el-radio>
            <el-radio value="间隔">间隔循环</el-radio>
          </el-radio-group>
        </el-form-item>

        <el-form-item v-if="taskForm.type === '定时'" label="执行时间">
          <el-date-picker
            v-model="taskForm.executeTime"
            type="datetime"
            placeholder="选择执行时间"
            style="width: 100%;"
          />
        </el-form-item>

        <el-form-item v-if="taskForm.type === '定时'" label="重复模式">
          <el-select v-model="taskForm.repeatMode" placeholder="选择重复模式" style="width: 100%;">
            <el-option label="仅一次" value="" />
            <el-option label="每天" value="每天" />
            <el-option label="工作日" value="工作日" />
            <el-option label="周末" value="周末" />
            <el-option label="自定义" value="自定义" />
          </el-select>
        </el-form-item>

        <el-form-item v-if="taskForm.type === '定时' && taskForm.repeatMode === '自定义'" label="自定义重复">
          <el-checkbox-group v-model="taskForm.weekdays">
            <el-checkbox value="1">周一</el-checkbox>
            <el-checkbox value="2">周二</el-checkbox>
            <el-checkbox value="3">周三</el-checkbox>
            <el-checkbox value="4">周四</el-checkbox>
            <el-checkbox value="5">周五</el-checkbox>
            <el-checkbox value="6">周六</el-checkbox>
            <el-checkbox value="7">周日</el-checkbox>
          </el-checkbox-group>
        </el-form-item>

        <el-form-item v-if="taskForm.type === '间隔'" label="间隔时间">
          <el-input-number v-model="taskForm.interval" :min="1" :max="9999" />
          <span style="margin-left: 8px;">分钟</span>
        </el-form-item>

        <el-form-item v-if="taskForm.type === '间隔'" label="执行次数">
          <el-radio-group v-model="taskForm.executeMode">
            <el-radio value="无限">无限循环</el-radio>
            <el-radio value="指定">指定次数</el-radio>
          </el-radio-group>
        </el-form-item>

        <el-form-item v-if="taskForm.type === '间隔' && taskForm.executeMode === '指定'" label="最大次数">
          <el-input-number v-model="taskForm.maxExecuteCount" :min="1" :max="9999" />
        </el-form-item>

        <el-form-item label="高级设置">
          <el-checkbox v-model="taskForm.retryOnFail">失败时自动重试(最多3次)</el-checkbox>
        </el-form-item>
      </el-form>

      <template #footer>
        <el-button @click="taskDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleSaveTask">保存</el-button>
      </template>
    </el-dialog>

    <el-dialog v-model="importDialogVisible" title="导入配置" width="500px">
      <div class="import-dialog-content">
        <div class="import-tips">
          <p>请上传 Excel 文件导入任务配置</p>
          <p class="import-tip-text">支持格式: .xlsx, .xls</p>
        </div>

        <el-upload
          ref="uploadRef"
          :auto-upload="false"
          :limit="1"
          accept=".xlsx,.xls"
          :on-change="handleFileChange"
          drag
        >
          <el-icon class="el-icon--upload"><upload-filled /></el-icon>
          <div class="el-upload__text">
            拖拽文件到此处，或<em>点击上传</em>
          </div>
        </el-upload>

        <div class="download-template">
          <el-button link type="primary" @click="handleDownloadTemplate">
            <el-icon><Download /></el-icon>
            下载导入模板
          </el-button>
        </div>
      </div>

      <template #footer>
        <el-button @click="importDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleConfirmImport">确认导入</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Search, UploadFilled, Download } from '@element-plus/icons-vue'
import { useTaskStore, type Task } from '../stores/task'
import { formatTime } from '../utils/time'
import { getStateText, TaskState } from '../utils/taskStateMachine'

const taskStore = useTaskStore()

const getStatusText = (status: TaskState) => {
  return getStateText(status)
}

const searchKeyword = ref('')
const filterType = ref('')
const currentPage = ref(1)
const pageSize = ref(10)
const selectedTasks = ref<Task[]>([])
const taskDialogVisible = ref(false)
const editingTask = ref<Task | null>(null)
const importDialogVisible = ref(false)
const uploadRef = ref()
const selectedFile = ref<File | null>(null)

const taskForm = ref({
  recipient: '',
  content: '',
  contentType: 'text' as 'text' | 'file',
  fileName: '',
  type: '定时' as '定时' | '间隔',
  executeTime: '' as string | Date,
  repeatMode: '',
  interval: 30,
  weekdays: [] as string[],
  executeMode: '无限',
  maxExecuteCount: 10,
  autoSplit: false,
  retryOnFail: true
})

const taskRules = {
  recipient: [{ required: true, message: '请输入接收对象', trigger: 'blur' }],
  content: [{ required: true, message: '请输入消息内容', trigger: 'blur' }]
}

const filteredTasks = computed(() => {
  return taskStore.tasks.filter(task => {
    const matchKeyword = !searchKeyword.value || task.recipient.includes(searchKeyword.value)
    const matchType = !filterType.value || task.type === filterType.value
    return matchKeyword && matchType
  })
})

const total = computed(() => filteredTasks.value.length)

const handleCreateTask = () => {
  editingTask.value = null

  const defaultExecuteTime = new Date(Date.now() + 60 * 1000)

  taskForm.value = {
    recipient: '',
    content: '',
    contentType: 'text',
    fileName: '',
    type: '定时',
    executeTime: defaultExecuteTime,
    repeatMode: '',
    interval: 30,
    weekdays: [],
    executeMode: '无限',
    maxExecuteCount: 10,
    autoSplit: false,
    retryOnFail: true
  }
  taskDialogVisible.value = true
}

const handleEditTask = (task: Task) => {
  editingTask.value = task
  taskForm.value = {
    recipient: task.recipient,
    content: task.content,
    contentType: task.contentType,
    fileName: task.fileName || '',
    type: task.type,
    executeTime: task.executeTime || '',
    repeatMode: task.repeatMode || '',
    interval: task.interval || 30,
    weekdays: task.weekdays || [],
    executeMode: task.executeMode,
    maxExecuteCount: task.maxExecuteCount,
    autoSplit: task.autoSplit,
    retryOnFail: task.retryOnFail
  }
  taskDialogVisible.value = true
}

const handleSaveTask = async () => {
  if (!taskForm.value.recipient) {
    ElMessage.warning('请填写完整信息')
    return
  }

  const executeTime = taskForm.value.executeTime
  // 直接传递 Date 对象或 ISO 字符串，让 taskStore.addTask 统一处理
  const taskData = {
    recipient: taskForm.value.recipient,
    content: taskForm.value.content,
    contentType: taskForm.value.contentType,
    fileName: taskForm.value.fileName,
    type: taskForm.value.type,
    executeTime: executeTime instanceof Date ? executeTime : (executeTime || new Date(Date.now() + 60 * 1000)),
    repeatMode: taskForm.value.repeatMode,
    interval: taskForm.value.interval,
    weekdays: taskForm.value.weekdays,
    executeMode: taskForm.value.executeMode,
    maxExecuteCount: taskForm.value.maxExecuteCount,
    autoSplit: taskForm.value.autoSplit,
    retryOnFail: taskForm.value.retryOnFail
  }

  try {
    if (editingTask.value) {
      await taskStore.updateTask(editingTask.value.id, taskData)
      ElMessage.success('任务更新成功')
    } else {
      await taskStore.addTask({
        ...taskData,
        enabled: true
      })
      ElMessage.success('任务创建成功')
    }
    taskDialogVisible.value = false
  } catch (error: any) {
    ElMessage.error(error?.message || '操作失败')
  }
}

const handleDeleteTask = async (task: Task) => {
  try {
    await ElMessageBox.confirm('确定要删除该任务吗?', '确认删除', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    await taskStore.deleteTask(task.id)
    ElMessage.success('任务已删除')
  } catch {
    // 用户取消
  }
}

const handleToggleTask = async (task: Task) => {
  task.status = task.enabled ? 'pending' : 'paused'
  taskStore.saveToStorage()
  ElMessage.success(task.enabled ? '任务已启用' : '任务已禁用')
}

const handleSelectionChange = (selection: Task[]) => {
  selectedTasks.value = selection
}

const handleBatchDelete = async () => {
  try {
    await ElMessageBox.confirm(`确定要删除选中的 ${selectedTasks.value.length} 个任务吗?`, '确认删除', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })

    for (const task of selectedTasks.value) {
      await taskStore.deleteTask(task.id)
    }
    selectedTasks.value = []
    ElMessage.success('批量删除成功')
  } catch {
    // 用户取消
  }
}

const handleImportExcel = () => {
  importDialogVisible.value = true
}

const handleClearAll = async () => {
  try {
    await ElMessageBox.confirm('确定要清空所有任务吗？此操作不可恢复！', '确认清空', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    await taskStore.clearAllTasks()
    ElMessage.success('已清空所有任务')
  } catch {
    // 用户取消
  }
}

const handleFileChange = (file: any) => {
  selectedFile.value = file.raw
}

const handleDownloadTemplate = () => {
  const templateContent = `接收对象,发送内容,内容类型,任务类型,执行时间,间隔时间(分钟),执行模式,最大执行次数
技术部群,早会提醒,text,定时,2025-07-20 09:00,,无限,
工作群,日报提醒,text,间隔,,30,无限,10`

  const blob = new Blob([templateContent], { type: 'text/csv;charset=utf-8;' })
  const link = document.createElement('a')
  link.href = URL.createObjectURL(blob)
  link.download = '任务导入模板.csv'
  link.click()
  URL.revokeObjectURL(link.href)
  ElMessage.success('模板下载成功')
}

const handleStartExecution = async () => {
  // 先检查是否有任务
  if (taskStore.tasks.length === 0) {
    ElMessage.warning('没有可执行的任务，请先添加任务')
    return
  }

  const enabledTasks = taskStore.tasks.filter(t => t.enabled)
  if (enabledTasks.length === 0) {
    ElMessage.warning('没有启用的任务，请先启用任务')
    return
  }

  try {
    await taskStore.startExecution()
    ElMessage.success('开始执行任务')
  } catch (error: any) {
    ElMessage.error(error.message || '启动失败')
  }
}

const handleStopExecution = () => {
  taskStore.stopExecution()
  ElMessage.warning('已停止执行任务')
}

const handleConfirmImport = () => {
  if (!selectedFile.value) {
    ElMessage.warning('请选择文件')
    return
  }

  ElMessage.success('导入成功')
  importDialogVisible.value = false
  selectedFile.value = null
}

const handleSelectContact = () => {
  ElMessage.info('联系人选择功能开发中')
}

const handleSelectFile = () => {
  ElMessage.info('文件选择功能开发中')
}
</script>

<style scoped>
.auto-message-page {
  max-width: 1320px;
  margin: 0 auto;
}

.page-header {
  margin-bottom: 32px;
}

.page-title {
  font-size: 48px;
  font-weight: 300;
  color: #061b31;
  margin: 0 0 8px 0;
  letter-spacing: -0.96px;
}

.page-subtitle {
  font-size: 22px;
  color: #64748d;
  margin: 0;
  font-weight: 300;
  letter-spacing: -0.22px;
}

.divider {
  height: 1px;
  background: #e5edf5;
  margin: 52px 0;
}

.toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 32px;
}

.toolbar-left {
  display: flex;
  gap: 12px;
}

.toolbar-right {
  display: flex;
  gap: 12px;
  align-items: center;
}

.table-wrapper {
  background: #ffffff;
}

.table-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px solid #e5edf5;
}

.task-type-tag {
  display: inline-block;
  padding: 4px 12px;
  font-size: 12px;
  font-weight: 400;
  border-radius: 9999px;
  background: #e8e9ff;
  color: #533afd;
}

.text-ellipsis {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 100%;
}

.import-dialog-content {
  padding: 20px 0;
}

.import-tips {
  text-align: center;
  margin-bottom: 24px;
}

.import-tips p {
  margin: 0;
  font-size: 16px;
  color: #061b31;
  font-weight: 300;
}

.import-tip-text {
  margin-top: 8px;
  font-size: 14px;
  color: #64748d;
}

.download-template {
  margin-top: 24px;
  text-align: center;
}

.execute-count {
  font-size: 14px;
  color: #061b31;
  font-weight: 400;
}

.task-status {
  display: flex;
  align-items: center;
  gap: 8px;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #64748d;
  flex-shrink: 0;
}

.task-status.running .status-dot {
  background: #10b981;
  animation: pulse 2s ease-in-out infinite;
}

.task-status.completed .status-dot {
  background: #533afd;
}

.task-status.failed .status-dot {
  background: #ef4444;
}

.task-status.paused .status-dot {
  background: #64748d;
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}

.status-text {
  font-size: 14px;
  color: #64748d;
  font-weight: 300;
}

.task-status.running .status-text {
  color: #10b981;
  font-weight: 400;
}

.task-status.completed .status-text {
  color: #533afd;
}

.task-status.failed .status-text {
  color: #ef4444;
}
</style>