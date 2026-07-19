<template>
  <div>
    <el-card style="margin-bottom: 20px;">
      <div style="display: flex; justify-content: space-between; align-items: center;">
        <div style="display: flex; gap: 12px;">
          <el-input v-model="searchKeyword" placeholder="搜索任务" style="width: 200px;" clearable>
            <template #prefix>
              <el-icon><Search /></el-icon>
            </template>
          </el-input>

          <el-select v-model="filterType" placeholder="任务类型" style="width: 120px;" clearable>
            <el-option label="定时任务" value="定时" />
            <el-option label="循环任务" value="间隔" />
          </el-select>

          <el-select v-model="filterStatus" placeholder="任务状态" style="width: 120px;" clearable>
            <el-option label="已启用" :value="true" />
            <el-option label="已禁用" :value="false" />
          </el-select>
        </div>

        <div style="display: flex; gap: 12px;">
          <el-button type="primary" @click="handleCreateTask">
            <el-icon style="margin-right: 4px;"><Plus /></el-icon>
            新建任务
          </el-button>

          <el-button type="success" @click="handleImportExcel">
            <el-icon style="margin-right: 4px;"><Upload /></el-icon>
            导入配置
          </el-button>

          <el-button type="warning" @click="handleExportExcel">
            <el-icon style="margin-right: 4px;"><Download /></el-icon>
            导出配置
          </el-button>
        </div>
      </div>
    </el-card>

    <el-card>
      <el-table :data="filteredTasks" style="width: 100%" @selection-change="handleSelectionChange">
        <el-table-column type="selection" width="55" />

        <el-table-column prop="name" label="任务名称" width="150">
          <template #default="{ row }">
            <div style="display: flex; align-items: center; gap: 8px;">
              <el-icon :color="row.enabled ? '#67C23A' : '#909399'">
                <component :is="row.type === '定时' ? 'Clock' : 'Refresh'" />
              </el-icon>
              <span>{{ row.name }}</span>
            </div>
          </template>
        </el-table-column>

        <el-table-column prop="recipient" label="接收对象" width="120" />

        <el-table-column prop="content" label="发送内容" min-width="200">
          <template #default="{ row }">
            <div v-if="row.contentType === 'text'" style="white-space: pre-wrap;">{{ row.content }}</div>
            <div v-else style="display: flex; align-items: center; gap: 8px;">
              <el-icon><Document /></el-icon>
              <span>{{ row.fileName }}</span>
            </div>
          </template>
        </el-table-column>

        <el-table-column prop="type" label="类型" width="100">
          <template #default="{ row }">
            <el-tag :type="row.type === '定时' ? 'primary' : 'success'" size="small">
              {{ row.type }}
            </el-tag>
          </template>
        </el-table-column>

        <el-table-column prop="schedule" label="执行时间" width="160">
          <template #default="{ row }">
            <div v-if="row.type === '定时'">
              <div>{{ row.executeTime }}</div>
              <div style="font-size: 12px; color: #909399;">{{ row.repeatMode || '仅一次' }}</div>
            </div>
            <div v-else>
              间隔 {{ row.interval }} 分钟
            </div>
          </template>
        </el-table-column>

        <el-table-column prop="nextExecute" label="下次执行" width="160" />

        <el-table-column prop="executeCount" label="执行次数" width="100">
          <template #default="{ row }">
            <el-tag size="small">{{ row.executeCount }} 次</el-tag>
          </template>
        </el-table-column>

        <el-table-column prop="enabled" label="状态" width="80">
          <template #default="{ row }">
            <el-switch v-model="row.enabled" @change="handleToggleTask(row)" />
          </template>
        </el-table-column>

        <el-table-column label="操作" width="180" fixed="right">
          <template #default="{ row }">
            <el-button type="primary" text size="small" @click="handleEditTask(row)">编辑</el-button>
            <el-button type="success" text size="small" @click="handleExecuteNow(row)">立即执行</el-button>
            <el-button type="danger" text size="small" @click="handleDeleteTask(row)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>

      <div style="margin-top: 20px; display: flex; justify-content: space-between; align-items: center;">
        <div>
          已选择 {{ selectedTasks.length }} 项
          <el-button v-if="selectedTasks.length > 0" type="danger" text @click="handleBatchDelete">
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
    </el-card>

    <el-dialog v-model="taskDialogVisible" :title="editingTask ? '编辑任务' : '新建任务'" width="600px">
      <el-form :model="taskForm" label-width="100px" :rules="taskRules">
        <el-form-item label="任务名称" prop="name">
          <el-input v-model="taskForm.name" placeholder="例如：早会提醒" />
        </el-form-item>

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
            <el-checkbox v-model="taskForm.autoSplit">智能拆句（长消息自动拆分）</el-checkbox>
          </div>
        </el-form-item>

        <el-form-item v-else label="选择文件" prop="filePath">
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
          <el-checkbox v-model="taskForm.retryOnFail">失败时自动重试（最多3次）</el-checkbox>
        </el-form-item>

        <el-form-item label="任务备注">
          <el-input v-model="taskForm.remark" type="textarea" :rows="2" placeholder="任务备注（可选）" />
        </el-form-item>
      </el-form>

      <template #footer>
        <el-button @click="taskDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleSaveTask">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Search, Plus, Upload, Download, Clock, Refresh, Document } from '@element-plus/icons-vue'

interface Task {
  id: number
  name: string
  recipient: string
  content: string
  contentType: 'text' | 'file'
  fileName?: string
  type: '定时' | '间隔'
  executeTime?: string
  repeatMode?: string
  interval?: number
  weekdays?: string[]
  executeMode?: string
  maxExecuteCount?: number
  nextExecute: string
  enabled: boolean
  executeCount: number
  autoSplit: boolean
  retryOnFail: boolean
  remark?: string
}

const searchKeyword = ref('')
const filterType = ref('')
const filterStatus = ref<boolean | ''>('')
const currentPage = ref(1)
const pageSize = ref(10)
const selectedTasks = ref<Task[]>([])
const taskDialogVisible = ref(false)
const editingTask = ref<Task | null>(null)

const tasks = ref<Task[]>([
  { id: 1, name: '早会提醒', recipient: '技术部群', content: '各位同事，早会时间到了，请大家准时参加会议。', contentType: 'text', type: '定时', executeTime: '2025-07-20 09:00', repeatMode: '工作日', nextExecute: '2025-07-20 09:00', enabled: true, executeCount: 45, autoSplit: false, retryOnFail: true },
  { id: 2, name: '日报提醒', recipient: '工作群', content: '请大家记得提交今日工作日报。', contentType: 'text', type: '间隔', interval: 120, nextExecute: '2025-07-19 18:00', enabled: true, executeCount: 128, autoSplit: false, retryOnFail: false },
  { id: 3, name: '周报发送', recipient: '张总', content: '', contentType: 'file', fileName: '周报.xlsx', type: '定时', executeTime: '2025-07-21 18:00', repeatMode: '', nextExecute: '2025-07-21 18:00', enabled: false, executeCount: 12, autoSplit: false, retryOnFail: true }
])

const taskForm = ref({
  name: '',
  recipient: '',
  content: '',
  contentType: 'text' as 'text' | 'file',
  fileName: '',
  type: '定时' as '定时' | '间隔',
  executeTime: '',
  repeatMode: '',
  interval: 30,
  weekdays: [] as string[],
  executeMode: '无限',
  maxExecuteCount: 10,
  autoSplit: false,
  retryOnFail: true,
  remark: ''
})

const taskRules = {
  name: [{ required: true, message: '请输入任务名称', trigger: 'blur' }],
  recipient: [{ required: true, message: '请输入接收对象', trigger: 'blur' }],
  content: [{ required: true, message: '请输入消息内容', trigger: 'blur' }]
}

const filteredTasks = computed(() => {
  return tasks.value.filter(task => {
    const matchKeyword = !searchKeyword.value || task.name.includes(searchKeyword.value) || task.recipient.includes(searchKeyword.value)
    const matchType = !filterType.value || task.type === filterType.value
    const matchStatus = filterStatus.value === '' || task.enabled === filterStatus.value
    return matchKeyword && matchType && matchStatus
  })
})

const total = computed(() => filteredTasks.value.length)

const handleCreateTask = () => {
  editingTask.value = null
  taskForm.value = {
    name: '',
    recipient: '',
    content: '',
    contentType: 'text',
    fileName: '',
    type: '定时',
    executeTime: '',
    repeatMode: '',
    interval: 30,
    weekdays: [],
    executeMode: '无限',
    maxExecuteCount: 10,
    autoSplit: false,
    retryOnFail: true,
    remark: ''
  }
  taskDialogVisible.value = true
}

const handleEditTask = (task: Task) => {
  editingTask.value = task
  taskForm.value = { ...task }
  taskDialogVisible.value = true
}

const handleSaveTask = () => {
  if (!taskForm.value.name || !taskForm.value.recipient) {
    ElMessage.warning('请填写完整信息')
    return
  }

  if (editingTask.value) {
    const index = tasks.value.findIndex(t => t.id === editingTask.value!.id)
    if (index !== -1) {
      tasks.value[index] = {
        ...editingTask.value,
        ...taskForm.value
      } as Task
    }
    ElMessage.success('任务更新成功')
  } else {
    const newTask: Task = {
      id: Date.now(),
      ...taskForm.value,
      nextExecute: taskForm.value.executeTime || new Date().toISOString(),
      executeCount: 0,
      enabled: true
    } as Task
    tasks.value.push(newTask)
    ElMessage.success('任务创建成功')
  }

  taskDialogVisible.value = false
}

const handleDeleteTask = (task: Task) => {
  ElMessageBox.confirm('确定要删除该任务吗？', '确认删除', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning'
  }).then(() => {
    const index = tasks.value.findIndex(t => t.id === task.id)
    if (index !== -1) {
      tasks.value.splice(index, 1)
      ElMessage.success('任务已删除')
    }
  })
}

const handleToggleTask = (task: Task) => {
  ElMessage.success(task.enabled ? '任务已启用' : '任务已禁用')
}

const handleExecuteNow = (task: Task) => {
  ElMessage.success('任务已立即执行')
}

const handleSelectionChange = (selection: Task[]) => {
  selectedTasks.value = selection
}

const handleBatchDelete = () => {
  ElMessageBox.confirm(`确定要删除选中的 ${selectedTasks.value.length} 个任务吗？`, '确认删除', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning'
  }).then(() => {
    selectedTasks.value.forEach(task => {
      const index = tasks.value.findIndex(t => t.id === task.id)
      if (index !== -1) {
        tasks.value.splice(index, 1)
      }
    })
    selectedTasks.value = []
    ElMessage.success('批量删除成功')
  })
}

const handleImportExcel = () => {
  ElMessage.info('导入功能开发中')
}

const handleExportExcel = () => {
  ElMessage.info('导出功能开发中')
}

const handleSelectContact = () => {
  ElMessage.info('联系人选择功能开发中')
}

const handleSelectFile = () => {
  ElMessage.info('文件选择功能开发中')
}
</script>