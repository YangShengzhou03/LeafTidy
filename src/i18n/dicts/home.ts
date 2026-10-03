import type { Dict } from '../types'

const dict: Dict = {
  'home.workDirs.title': { 'zh-CN': '待整理文件夹', 'zh-TW': '待整理資料夾', en: 'Folders to Organize' },
  'home.workDirs.empty': { 'zh-CN': '点击添加需要整理的文件夹', 'zh-TW': '點擊新增需要整理的資料夾', en: 'Click to add folders to process' },
  'home.outputDir.title': { 'zh-CN': '保存位置', 'zh-TW': '儲存位置', en: 'Save Location' },
  'home.outputDir.empty': { 'zh-CN': '点击选择整理后文件的保存位置', 'zh-TW': '點擊選擇整理後檔案的儲存位置', en: 'Click to choose where organized files go' },
  'home.quickActions.title': { 'zh-CN': '快速操作', 'zh-TW': '快速操作', en: 'Quick Actions' },
  'home.add': { 'zh-CN': '添加', 'zh-TW': '新增', en: 'Add' },

  'home.action.organize': { 'zh-CN': '文件整理', 'zh-TW': '檔案整理', en: 'Organize Files' },
  'home.action.organizeDesc': { 'zh-CN': '按类型、日期、大小自动整理文件', 'zh-TW': '依類型、日期、大小自動整理檔案', en: 'Sort files by type, date or size' },
  'home.action.rename': { 'zh-CN': '批量重命名', 'zh-TW': '批次重新命名', en: 'Batch Rename' },
  'home.action.renameDesc': { 'zh-CN': '用模板批量修改文件名', 'zh-TW': '用範本批次修改檔名', en: 'Rename files in bulk with templates' },
  'home.action.duplicates': { 'zh-CN': '重复清理', 'zh-TW': '重複清理', en: 'Duplicate Cleanup' },
  'home.action.duplicatesDesc': { 'zh-CN': '识别并清理重复文件，释放空间', 'zh-TW': '識別並清理重複檔案，釋放空間', en: 'Find and remove duplicates to free up space' },
  'home.action.aiClassify': { 'zh-CN': 'AI 智能分类', 'zh-TW': 'AI 智慧分類', en: 'AI Classify' },
  'home.action.aiClassifyDesc': { 'zh-CN': 'AI 自动识别照片内容并分类', 'zh-TW': 'AI 自動辨識照片內容並分類', en: 'Let AI recognize and classify photos' },
  'home.action.fixDate': { 'zh-CN': '修复拍摄时间', 'zh-TW': '修復拍攝時間', en: 'Fix Shot Date' },
  'home.action.fixDateDesc': { 'zh-CN': '修正照片错误的拍摄日期', 'zh-TW': '修正照片錯誤的拍攝日期', en: 'Correct wrong photo capture dates' },

  'home.tour.workDirs': {
    'zh-CN': '在此添加需要整理的文件夹，可以添加多个目录。添加后右侧会显示文件统计信息。',
    'zh-TW': '在此新增需要整理的資料夾，可以新增多個目錄。新增後右側會顯示檔案統計資訊。',
    en: 'Add folders to organize here — multiple folders are supported. File stats appear on the right once added.',
  },
  'home.tour.outputDir': {
    'zh-CN': '指定文件整理后的输出位置。建议选择一个空文件夹作为输出目录。',
    'zh-TW': '指定檔案整理後的輸出位置。建議選擇一個空資料夾作為輸出目錄。',
    en: 'Choose where organized files go. An empty folder is recommended as the output directory.',
  },
  'home.tour.quickActions': {
    'zh-CN': '点击卡片快速进入对应功能。包括文件整理、批量重命名、重复文件清理和AI智能分类等强大功能。',
    'zh-TW': '點擊卡片快速進入對應功能。包括檔案整理、批次重新命名、重複檔案清理和AI智慧分類等強大功能。',
    en: 'Click a card to jump into a feature — file organizing, batch rename, duplicate cleanup, AI classification and more.',
  },

  'home.dialog.selectWorkDirs': { 'zh-CN': '选择需要整理的文件夹', 'zh-TW': '選擇需要整理的資料夾', en: 'Select folders to organize' },
  'home.dialog.selectOutputDir': { 'zh-CN': '选择整理后文件的保存位置', 'zh-TW': '選擇整理後檔案的儲存位置', en: 'Choose where to save organized files' },

  'home.error.selectDir': { 'zh-CN': '选择目录失败', 'zh-TW': '選擇目錄失敗', en: 'Failed to select directory' },
  'home.error.stats': { 'zh-CN': '获取目录统计失败', 'zh-TW': '取得目錄統計失敗', en: 'Failed to get directory stats' },
  // SMB 网络共享
  'home.smb.connectBtn': { 'zh-CN': '连接NAS', 'zh-TW': '連接NAS', en: 'Connect NAS' },
  'home.smb.dialogTitle': { 'zh-CN': '连接 SMB 共享', 'zh-TW': '連接 SMB 共用', en: 'Connect SMB Share' },
  'home.smb.server': { 'zh-CN': '服务器地址', 'zh-TW': '伺服器位址', en: 'Server Address' },
  'home.smb.serverPlaceholder': { 'zh-CN': '例如: 192.168.1.100 或 nas.local', 'zh-TW': '例如: 192.168.1.100 或 nas.local', en: 'e.g. 192.168.1.100 or nas.local' },
  'home.smb.share': { 'zh-CN': '共享文件夹', 'zh-TW': '共用資料夾', en: 'Share Folder' },
  'home.smb.sharePlaceholder': { 'zh-CN': '例如: photo, video, homes', 'zh-TW': '例如: photo, video, homes', en: 'e.g. photo, video, homes' },
  'home.smb.username': { 'zh-CN': '用户名', 'zh-TW': '使用者名稱', en: 'Username' },
  'home.smb.password': { 'zh-CN': '密码', 'zh-TW': '密碼', en: 'Password' },
  'home.smb.persistent': { 'zh-CN': '记住连接（Windows 凭据管理器）', 'zh-TW': '記住連線（Windows 認證管理員）', en: 'Remember connection (Windows Credential Manager)' },
  'home.smb.browse': { 'zh-CN': '浏览...', 'zh-TW': '瀏覽...', en: 'Browse...' },
  'home.smb.connecting': { 'zh-CN': '正在连接...', 'zh-TW': '正在連線...', en: 'Connecting...' },
  'home.smb.connectSuccess': { 'zh-CN': '连接成功: {path}', 'zh-TW': '連線成功: {path}', en: 'Connected: {path}' },
  'home.smb.connectFailed': { 'zh-CN': '连接失败: {msg}', 'zh-TW': '連線失敗: {msg}', en: 'Connection failed: {msg}' },
  'home.smb.alreadyConnected': { 'zh-CN': '该共享已连接', 'zh-TW': '此共用已連線', en: 'Share already connected' },
  'home.smb.disconnect': { 'zh-CN': '断开连接', 'zh-TW': '中斷連線', en: 'Disconnect' },
  'home.smb.disconnectSuccess': { 'zh-CN': '已断开连接: {path}', 'zh-TW': '已中斷連線: {path}', en: 'Disconnected: {path}' },
  'home.smb.disconnectConfirm': { 'zh-CN': '确定要断开连接 {path} 吗？', 'zh-TW': '確定要中斷連線 {path} 嗎？', en: 'Are you sure you want to disconnect {path}?' },
  'home.smb.invalidServer': { 'zh-CN': '请输入服务器地址', 'zh-TW': '請輸入伺服器位址', en: 'Please enter server address' },
  'home.smb.invalidShare': { 'zh-CN': '请输入共享文件夹', 'zh-TW': '請輸入共用資料夾', en: 'Please enter share folder' },
  'home.smb.connectedShares': { 'zh-CN': '已连接的共享', 'zh-TW': '已連線的共用', en: 'Connected Shares' },
  'home.smb.noShares': { 'zh-CN': '暂无已连接的共享', 'zh-TW': '尚無已連線的共用', en: 'No connected shares' },
  'home.smb.statusConnected': { 'zh-CN': '已连接', 'zh-TW': '已連線', en: 'Connected' },
  'home.smb.statusDisconnected': { 'zh-CN': '不可访问', 'zh-TW': '無法存取', en: 'Unreachable' },
  'home.smb.addAsWorkDir': { 'zh-CN': '添加到工作目录', 'zh-TW': '新增到工作目錄', en: 'Add to folders' },

  // 日志操作名
  'home.log.addDir': { 'zh-CN': '添加工作目录: {name}', 'zh-TW': '新增工作目錄: {name}', en: 'Add work folder: {name}' },
  'home.log.removeDir': { 'zh-CN': '移除工作目录: {name}', 'zh-TW': '移除工作目錄: {name}', en: 'Remove work folder: {name}' },
  'home.log.smbConnect': { 'zh-CN': '连接SMB共享: {path}', 'zh-TW': '連接SMB共用: {path}', en: 'Connect SMB share: {path}' },
  'home.log.smbDisconnect': { 'zh-CN': '断开SMB共享: {path}', 'zh-TW': '中斷SMB共用: {path}', en: 'Disconnect SMB share: {path}' },
}

export default dict
