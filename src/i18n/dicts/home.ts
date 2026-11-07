import type { Dict } from '../types'

const dict: Dict = {
  'home.workDirs.title': { 'zh-CN': '待整理文件夹', 'zh-TW': '待整理資料夾', en: 'Folders to Organize' },
  'home.workDirs.empty': { 'zh-CN': '点击添加待整理的文件夹', 'zh-TW': '點擊新增待整理的資料夾', en: 'Click to add folders to organize' },
  'home.outputDir.title': { 'zh-CN': '保存位置', 'zh-TW': '儲存位置', en: 'Save Location' },
  'home.outputDir.empty': { 'zh-CN': '点击选择整理后文件的保存位置', 'zh-TW': '點擊選擇整理後檔案的儲存位置', en: 'Click to choose where organized files go' },
  'home.add': { 'zh-CN': '添加', 'zh-TW': '新增', en: 'Add' },

  'home.dialog.selectWorkDirs': { 'zh-CN': '选择需要整理的文件夹', 'zh-TW': '選擇需要整理的資料夾', en: 'Select folders to organize' },
  'home.dialog.selectOutputDir': { 'zh-CN': '选择整理后文件的保存位置', 'zh-TW': '選擇整理後檔案的儲存位置', en: 'Choose where to save organized files' },

  'home.error.selectDir': { 'zh-CN': '选择目录失败', 'zh-TW': '選擇目錄失敗', en: 'Failed to select directory' },
  'home.error.stats': { 'zh-CN': '获取目录统计失败', 'zh-TW': '取得目錄統計失敗', en: 'Failed to get directory stats' },
  // SMB 网络共享
  'home.smb.connectBtn': { 'zh-CN': '连接 NAS', 'zh-TW': '連接 NAS', en: 'Connect NAS' },
  'home.smb.dialogTitle': { 'zh-CN': '连接 SMB 共享', 'zh-TW': '連接 SMB 共用', en: 'Connect SMB Share' },
  'home.smb.server': { 'zh-CN': '服务器地址', 'zh-TW': '伺服器位址', en: 'Server Address' },
  'home.smb.serverPlaceholder': { 'zh-CN': '例如: 192.168.1.100 或 nas.local', 'zh-TW': '例如: 192.168.1.100 或 nas.local', en: 'e.g. 192.168.1.100 or nas.local' },
  'home.smb.share': { 'zh-CN': '共享文件夹', 'zh-TW': '共用資料夾', en: 'Share Folder' },
  'home.smb.sharePlaceholder': { 'zh-CN': '例如: photo, video, homes', 'zh-TW': '例如: photo, video, homes', en: 'e.g. photo, video, homes' },
  'home.smb.username': { 'zh-CN': '用户名', 'zh-TW': '使用者名稱', en: 'Username' },
  'home.smb.password': { 'zh-CN': '密码', 'zh-TW': '密碼', en: 'Password' },
  'home.smb.persistent': { 'zh-CN': '记住连接（Windows 凭据管理器）', 'zh-TW': '記住連線（Windows 認證管理員）', en: 'Remember connection (Windows Credential Manager)' },
  'home.smb.connecting': { 'zh-CN': '正在连接...', 'zh-TW': '正在連線...', en: 'Connecting...' },
  'home.smb.connectSuccess': { 'zh-CN': '连接成功: {path}', 'zh-TW': '連線成功: {path}', en: 'Connected: {path}' },
  'home.smb.connectFailed': { 'zh-CN': '连接失败: {msg}', 'zh-TW': '連線失敗: {msg}', en: 'Connection failed: {msg}' },
  'home.smb.disconnect': { 'zh-CN': '断开连接', 'zh-TW': '中斷連線', en: 'Disconnect' },
  'home.smb.disconnectSuccess': { 'zh-CN': '已断开连接: {path}', 'zh-TW': '已中斷連線: {path}', en: 'Disconnected: {path}' },
  'home.smb.disconnectConfirm': { 'zh-CN': '确定要断开连接 {path} 吗？', 'zh-TW': '確定要中斷連線 {path} 嗎？', en: 'Are you sure you want to disconnect {path}?' },
  'home.smb.invalidServer': { 'zh-CN': '请输入服务器地址', 'zh-TW': '請輸入伺服器位址', en: 'Please enter server address' },
  'home.smb.invalidShare': { 'zh-CN': '请输入共享文件夹名称', 'zh-TW': '請輸入共用資料夾名稱', en: 'Please enter the share name' },
  'home.smb.connectedShares': { 'zh-CN': '已连接的共享', 'zh-TW': '已連線的共用', en: 'Connected Shares' },
  'home.smb.noShares': { 'zh-CN': '暂无已连接的共享', 'zh-TW': '尚無已連線的共用', en: 'No connected shares' },
  'home.smb.statusConnected': { 'zh-CN': '已连接', 'zh-TW': '已連線', en: 'Connected' },
  'home.smb.statusDisconnected': { 'zh-CN': '不可访问', 'zh-TW': '無法存取', en: 'Unreachable' },

  // 日志操作名
  'home.log.addDir': { 'zh-CN': '添加待整理文件夹: {name}', 'zh-TW': '新增待整理資料夾: {name}', en: 'Add folder: {name}' },
  'home.log.removeDir': { 'zh-CN': '移除待整理文件夹: {name}', 'zh-TW': '移除待整理資料夾: {name}', en: 'Remove folder: {name}' },
  'home.log.smbConnect': { 'zh-CN': '连接SMB共享: {path}', 'zh-TW': '連接SMB共用: {path}', en: 'Connect SMB share: {path}' },
  'home.log.smbDisconnect': { 'zh-CN': '断开SMB共享: {path}', 'zh-TW': '中斷SMB共用: {path}', en: 'Disconnect SMB share: {path}' },
}

export default dict
