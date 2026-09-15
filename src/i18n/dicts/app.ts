import type { Dict } from '../types'

const dict: Dict = {
  // App.vue
  'app.back': { 'zh-CN': '后退', 'zh-TW': '後退', en: 'Back' },
  'app.forward': { 'zh-CN': '前进', 'zh-TW': '前進', en: 'Forward' },
  'app.geoLoading': { 'zh-CN': '正在加载地理数据…', 'zh-TW': '正在載入地理資料…', en: 'Loading geo data…' },
  'app.geoReady': { 'zh-CN': '地理数据已就绪', 'zh-TW': '地理資料已就緒', en: 'Geo data ready' },
  'app.geoFailed': { 'zh-CN': '地理数据加载失败，写入地理信息功能暂不可用', 'zh-TW': '地理資料載入失敗，寫入地理資訊功能暫不可用', en: 'Failed to load geo data. GPS writing is unavailable' },
  'app.warning': { 'zh-CN': '提示', 'zh-TW': '提示', en: 'Notice' },
  'app.taskInProgress': { 'zh-CN': '有任务正在进行，请等任务完成或先停止任务，再关闭应用', 'zh-TW': '有任務正在進行，請等任務完成或先停止任務，再關閉應用', en: 'A task is running. Please wait for it to finish or stop it before closing' },
  'app.confirmExitMsg': { 'zh-CN': '确定要退出轻羽归档吗？退出后未完成的任务会中断。', 'zh-TW': '確定要退出輕羽歸檔嗎？退出後未完成的任務會中斷。', en: 'Quit LeafTidy? Unfinished tasks will be interrupted.' },
  'app.confirmExit': { 'zh-CN': '确认退出', 'zh-TW': '確認退出', en: 'Confirm Exit' },
  'app.ok': { 'zh-CN': '确定', 'zh-TW': '確定', en: 'OK' },
  'app.cancel': { 'zh-CN': '取消', 'zh-TW': '取消', en: 'Cancel' },
  'app.error': { 'zh-CN': '错误', 'zh-TW': '錯誤', en: 'Error' },
  'app.openFileFailed': { 'zh-CN': '打开文件失败: {msg}', 'zh-TW': '打開檔案失敗: {msg}', en: 'Failed to open file: {msg}' },
  // LeftSidebar.vue
  'sidebar.left.home': { 'zh-CN': '首页', 'zh-TW': '首頁', en: 'Home' },
  'sidebar.left.groupCommon': { 'zh-CN': '通用文件工具', 'zh-TW': '通用檔案工具', en: 'General File Tools' },
  'sidebar.left.fileOrganize': { 'zh-CN': '文件整理', 'zh-TW': '檔案整理', en: 'File Organizer' },
  'sidebar.left.batchRename': { 'zh-CN': '批量重命名', 'zh-TW': '批量重新命名', en: 'Batch Rename' },
  'sidebar.left.duplicateClean': { 'zh-CN': '重复文件清理', 'zh-TW': '重複檔案清理', en: 'Duplicate Cleaner' },
  'sidebar.left.associatedCleanup': { 'zh-CN': '附属文件清理', 'zh-TW': '附屬檔案清理', en: 'Associated Cleanup' },
  'sidebar.left.groupPhoto': { 'zh-CN': '照片与智能整理', 'zh-TW': '照片與智慧整理', en: 'Photos & Smart Tools' },
  'sidebar.left.fixDate': { 'zh-CN': '修复拍摄时间', 'zh-TW': '修復拍攝時間', en: 'Fix Capture Date' },
  'sidebar.left.exifClean': { 'zh-CN': '照片脱敏', 'zh-TW': '照片去識別', en: 'Privacy Cleanup' },
  'sidebar.left.writeGps': { 'zh-CN': '写入地理信息', 'zh-TW': '寫入地理資訊', en: 'Write GPS Info' },
  'sidebar.left.aiClassify': { 'zh-CN': 'AI 智能分类', 'zh-TW': 'AI 智慧分類', en: 'AI Classification' },
  'sidebar.left.logView': { 'zh-CN': '操作日志', 'zh-TW': '操作日誌', en: 'Operation Logs' },
  'sidebar.left.settings': { 'zh-CN': '设置', 'zh-TW': '設定', en: 'Settings' },
  // RightSidebar.vue
  'sidebar.right.selectDir': { 'zh-CN': '请先在左侧添加待整理的文件夹', 'zh-TW': '請先在左側新增待整理的資料夾', en: 'Add folders to organize on the left first' },
  'sidebar.right.totalFiles': { 'zh-CN': '文件总数', 'zh-TW': '檔案總數', en: 'Total Files' },
  'sidebar.right.totalDirs': { 'zh-CN': '目录总数', 'zh-TW': '目錄總數', en: 'Total Directories' },
  'sidebar.right.totalSize': { 'zh-CN': '总大小', 'zh-TW': '總大小', en: 'Total Size' },
  'sidebar.right.oldestFile': { 'zh-CN': '最旧文件', 'zh-TW': '最舊檔案', en: 'Oldest File' },
  'sidebar.right.newestFile': { 'zh-CN': '最新文件', 'zh-TW': '最新檔案', en: 'Newest File' },
  'sidebar.right.none': { 'zh-CN': '无', 'zh-TW': '無', en: 'None' },
  'sidebar.right.fileTypeDist': { 'zh-CN': '文件类型分布', 'zh-TW': '檔案類型分佈', en: 'File Type Distribution' },
  'sidebar.right.chartTooltip': { 'zh-CN': '{b}: {c} 个 ({d}%)', 'zh-TW': '{b}: {c} 個 ({d}%)', en: '{b}: {c} ({d}%)' },
}

export default dict
