import type { Dict } from '../types'

const dict: Dict = {
  // ===== 公共（三个照片工具页共用） =====
  'photos.common.selectDirsHint': {
    'zh-CN': '请先在左侧选择待处理目录',
    'zh-TW': '請先在左側選擇待處理目錄',
    'en': 'Select a folder to process on the left first',
  },
  'photos.common.selectDirs': {
    'zh-CN': '请先选择待处理目录',
    'zh-TW': '請先選擇待處理目錄',
    'en': 'Select a folder to process first',
  },
  'photos.common.selectOutput': {
    'zh-CN': '请先选择输出目录',
    'zh-TW': '請先選擇輸出目錄',
    'en': 'Select an output folder first',
  },
  'photos.common.noJpg': {
    'zh-CN': '没有找到 JPG 格式的照片',
    'zh-TW': '沒有找到 JPG 格式的照片',
    'en': 'No JPG photos found',
  },
  'photos.common.total': {
    'zh-CN': '总数: {n}',
    'zh-TW': '總數: {n}',
    'en': 'Total: {n}',
  },
  'photos.common.processed': {
    'zh-CN': '已处理: {n}',
    'zh-TW': '已處理: {n}',
    'en': 'Processed: {n}',
  },
  'photos.common.current': {
    'zh-CN': '当前:',
    'zh-TW': '當前:',
    'en': 'Current:',
  },
  'photos.common.resultStats': {
    'zh-CN': '成功: {s} / 失败: {f}',
    'zh-TW': '成功: {s} / 失敗: {f}',
    'en': 'Success: {s} / Failed: {f}',
  },
  'photos.common.failedCount': {
    'zh-CN': '失败 {n} 个',
    'zh-TW': '失敗 {n} 個',
    'en': '{n} failed',
  },
  'photos.common.successCount': {
    'zh-CN': '成功 {n} 个',
    'zh-TW': '成功 {n} 個',
    'en': '{n} succeeded',
  },
  'photos.common.tip': {
    'zh-CN': '提示',
    'zh-TW': '提示',
    'en': 'Notice',
  },
  'photos.common.warning': {
    'zh-CN': '警告',
    'zh-TW': '警告',
    'en': 'Warning',
  },
  'photos.common.error': {
    'zh-CN': '错误',
    'zh-TW': '錯誤',
    'en': 'Error',
  },
  'photos.common.success': {
    'zh-CN': '成功',
    'zh-TW': '成功',
    'en': 'Success',
  },
  'photos.common.stopFailed': {
    'zh-CN': '终止失败: {e}',
    'zh-TW': '終止失敗: {e}',
    'en': 'Stop failed: {e}',
  },

  // ===== 修复拍摄时间 =====
  'fixDate.title': {
    'zh-CN': '修复拍摄时间',
    'zh-TW': '修復拍攝時間',
    'en': 'Fix Date Taken',
  },
  'fixDate.desc': {
    'zh-CN': '为拍摄时间丢失的照片补写EXIF拍摄时间，支持 JPG 格式',
    'zh-TW': '為拍攝時間遺失的照片補寫EXIF拍攝時間，支援 JPG 格式',
    'en': 'Write EXIF date taken back onto photos that lost it. Supports JPG.',
  },
  'fixDate.sourceLabel': {
    'zh-CN': '时间来源',
    'zh-TW': '時間來源',
    'en': 'Date Source',
  },
  'fixDate.sourcePlaceholder': {
    'zh-CN': '选择时间来源',
    'zh-TW': '選擇時間來源',
    'en': 'Select date source',
  },
  'fixDate.sourceFilename': {
    'zh-CN': '从文件名识别时间',
    'zh-TW': '從檔案名稱識別時間',
    'en': 'From file name',
  },
  'fixDate.sourceSpecified': {
    'zh-CN': '指定时间',
    'zh-TW': '指定時間',
    'en': 'Specified time',
  },
  'fixDate.hintFilename': {
    'zh-CN': '从 IMG_20240115_143022 等常见命名中识别拍摄时间',
    'zh-TW': '從 IMG_20240115_143022 等常見命名中識別拍攝時間',
    'en': 'Recognize the date from common names like IMG_20240115_143022',
  },
  'fixDate.hintSpecified': {
    'zh-CN': '所有照片统一写入下方指定的时间',
    'zh-TW': '所有照片統一寫入下方指定的時間',
    'en': 'All photos get the same time specified below',
  },
  'fixDate.timeLabel': {
    'zh-CN': '指定时间',
    'zh-TW': '指定時間',
    'en': 'Specified time',
  },
  'fixDate.timePlaceholder': {
    'zh-CN': '选择日期和时间',
    'zh-TW': '選擇日期和時間',
    'en': 'Pick date and time',
  },
  'fixDate.progressTitle': {
    'zh-CN': '正在修复拍摄时间',
    'zh-TW': '正在修復拍攝時間',
    'en': 'Fixing date taken',
  },
  'fixDate.start': {
    'zh-CN': '开始修复',
    'zh-TW': '開始修復',
    'en': 'Start Fix',
  },
  'fixDate.stop': {
    'zh-CN': '终止修复',
    'zh-TW': '終止修復',
    'en': 'Stop Fix',
  },
  'fixDate.resultTitle': {
    'zh-CN': '修复结果',
    'zh-TW': '修復結果',
    'en': 'Fix Results',
  },
  'fixDate.stopInProgress': {
    'zh-CN': '正在终止修复操作...',
    'zh-TW': '正在終止修復操作...',
    'en': 'Stopping fix operation...',
  },
  'fixDate.selectTime': {
    'zh-CN': '请先选择指定时间',
    'zh-TW': '請先選擇指定時間',
    'en': 'Pick the specified time first',
  },
  'fixDate.doneAll': {
    'zh-CN': '修复完成，共处理 {n} 个文件，已复制到输出目录',
    'zh-TW': '修復完成，共處理 {n} 個檔案，已複製到輸出目錄',
    'en': 'Fix complete, {n} files processed, copied to the output folder',
  },
  'fixDate.donePartial': {
    'zh-CN': '修复完成，成功 {s} 个，失败 {f} 个',
    'zh-TW': '修復完成，成功 {s} 個，失敗 {f} 個',
    'en': 'Fix complete, {s} succeeded, {f} failed',
  },
  'fixDate.cancelled': {
    'zh-CN': '修复操作已取消',
    'zh-TW': '修復操作已取消',
    'en': 'Fix operation cancelled',
  },
  'fixDate.failed': {
    'zh-CN': '修复失败: {e}',
    'zh-TW': '修復失敗: {e}',
    'en': 'Fix failed: {e}',
  },

  // ===== 照片脱敏 =====
  'exif.title': {
    'zh-CN': '照片脱敏',
    'zh-TW': '照片脫敏',
    'en': 'Photo Privacy Cleanup',
  },
  'exif.desc': {
    'zh-CN': '清除照片中的拍摄时间、GPS位置、相机型号等隐私信息，支持 JPG 格式',
    'zh-TW': '清除照片中的拍攝時間、GPS位置、相機型號等隱私資訊，支援 JPG 格式',
    'en': 'Strip capture date, GPS location, camera model and other private info from photos. Supports JPG.',
  },
  'exif.stripExif': {
    'zh-CN': 'EXIF拍摄信息',
    'zh-TW': 'EXIF拍攝資訊',
    'en': 'EXIF Capture Info',
  },
  'exif.stripGps': {
    'zh-CN': 'GPS位置',
    'zh-TW': 'GPS位置',
    'en': 'GPS Location',
  },
  'exif.stripCamera': {
    'zh-CN': '相机品牌型号',
    'zh-TW': '相機品牌型號',
    'en': 'Camera Make & Model',
  },
  'exif.stripXmp': {
    'zh-CN': 'XMP标记',
    'zh-TW': 'XMP標記',
    'en': 'XMP Tags',
  },
  'exif.stripPhotoshop': {
    'zh-CN': 'Photoshop信息',
    'zh-TW': 'Photoshop資訊',
    'en': 'Photoshop Info',
  },
  'exif.progressTitle': {
    'zh-CN': '正在清除照片隐私信息',
    'zh-TW': '正在清除照片隱私資訊',
    'en': 'Cleaning photo privacy info',
  },
  'exif.start': {
    'zh-CN': '开始脱敏',
    'zh-TW': '開始脫敏',
    'en': 'Start Clean',
  },
  'exif.stop': {
    'zh-CN': '终止脱敏',
    'zh-TW': '終止脫敏',
    'en': 'Stop Clean',
  },
  'exif.resultTitle': {
    'zh-CN': '脱敏结果',
    'zh-TW': '脫敏結果',
    'en': 'Clean Results',
  },
  'exif.stopInProgress': {
    'zh-CN': '正在终止脱敏操作...',
    'zh-TW': '正在終止脫敏操作...',
    'en': 'Stopping clean operation...',
  },
  'exif.noneSelected': {
    'zh-CN': '请至少勾选一项要清除的隐私信息',
    'zh-TW': '請至少勾選一項要清除的隱私資訊',
    'en': 'Check at least one item to strip',
  },
  'exif.doneAll': {
    'zh-CN': '脱敏完成，共处理 {n} 个文件，已复制到输出目录',
    'zh-TW': '脫敏完成，共處理 {n} 個檔案，已複製到輸出目錄',
    'en': 'Clean complete, {n} files processed, copied to the output folder',
  },
  'exif.donePartial': {
    'zh-CN': '脱敏完成，成功 {s} 个，失败 {f} 个',
    'zh-TW': '脫敏完成，成功 {s} 個，失敗 {f} 個',
    'en': 'Clean complete, {s} succeeded, {f} failed',
  },
  'exif.cancelled': {
    'zh-CN': '脱敏操作已取消',
    'zh-TW': '脫敏操作已取消',
    'en': 'Clean operation cancelled',
  },
  'exif.failed': {
    'zh-CN': '脱敏失败: {e}',
    'zh-TW': '脫敏失敗: {e}',
    'en': 'Clean failed: {e}',
  },

  // ===== 写入地理信息 =====
  'gps.title': {
    'zh-CN': '写入地理信息',
    'zh-TW': '寫入地理資訊',
    'en': 'Write GPS Info',
  },
  'gps.desc': {
    'zh-CN': '为照片补写GPS坐标，方便按地点分类整理，支持 JPG 格式',
    'zh-TW': '為照片補寫GPS座標，方便按地點分類整理，支援 JPG 格式',
    'en': 'Write GPS coordinates to photos for location-based organizing. Supports JPG.',
  },
  'gps.latLabel': {
    'zh-CN': '纬度',
    'zh-TW': '緯度',
    'en': 'Latitude',
  },
  'gps.latRange': {
    'zh-CN': '-90 到 90，北纬为正',
    'zh-TW': '-90 到 90，北緯為正',
    'en': '-90 to 90, positive for north',
  },
  'gps.lngLabel': {
    'zh-CN': '经度',
    'zh-TW': '經度',
    'en': 'Longitude',
  },
  'gps.lngRange': {
    'zh-CN': '-180 到 180，东经为正',
    'zh-TW': '-180 到 180，東經為正',
    'en': '-180 to 180, positive for east',
  },
  'gps.preset.label': {
    'zh-CN': '预设地点',
    'zh-TW': '預設地點',
    'en': 'Preset Places',
  },
  'gps.preset.jian': {
    'zh-CN': '吉安',
    'zh-TW': '吉安',
    'en': "Ji'an",
  },
  'gps.preset.nanchang': {
    'zh-CN': '南昌',
    'zh-TW': '南昌',
    'en': 'Nanchang',
  },
  'gps.preset.hangzhou': {
    'zh-CN': '杭州',
    'zh-TW': '杭州',
    'en': 'Hangzhou',
  },
  'gps.locationLabel': {
    'zh-CN': '地点预览',
    'zh-TW': '地點預覽',
    'en': 'Location Preview',
  },
  'gps.query': {
    'zh-CN': '查询地点',
    'zh-TW': '查詢地點',
    'en': 'Query Location',
  },
  'gps.querying': {
    'zh-CN': '查询中...',
    'zh-TW': '查詢中...',
    'en': 'Querying...',
  },
  'gps.notFound': {
    'zh-CN': '未找到附近地名',
    'zh-TW': '未找到附近地名',
    'en': 'No nearby place found',
  },
  'gps.queryFailed': {
    'zh-CN': '查询失败: {e}',
    'zh-TW': '查詢失敗: {e}',
    'en': 'Query failed: {e}',
  },
  'gps.progressTitle': {
    'zh-CN': '正在写入地理信息',
    'zh-TW': '正在寫入地理資訊',
    'en': 'Writing GPS info',
  },
  'gps.start': {
    'zh-CN': '开始写入',
    'zh-TW': '開始寫入',
    'en': 'Start Write',
  },
  'gps.stop': {
    'zh-CN': '终止写入',
    'zh-TW': '終止寫入',
    'en': 'Stop Write',
  },
  'gps.resultTitle': {
    'zh-CN': '写入结果',
    'zh-TW': '寫入結果',
    'en': 'Write Results',
  },
  'gps.stopInProgress': {
    'zh-CN': '正在终止写入操作...',
    'zh-TW': '正在終止寫入操作...',
    'en': 'Stopping write operation...',
  },
  'gps.originWarning': {
    'zh-CN': '坐标为原点(0, 0)，请确认是否继续',
    'zh-TW': '座標為原點(0, 0)，請確認是否繼續',
    'en': 'Coordinates are the origin (0, 0), confirm to continue',
  },
  'gps.doneAll': {
    'zh-CN': '写入完成，共处理 {n} 个文件，已复制到输出目录',
    'zh-TW': '寫入完成，共處理 {n} 個檔案，已複製到輸出目錄',
    'en': 'Write complete, {n} files processed, copied to the output folder',
  },
  'gps.donePartial': {
    'zh-CN': '写入完成，成功 {s} 个，失败 {f} 个',
    'zh-TW': '寫入完成，成功 {s} 個，失敗 {f} 個',
    'en': 'Write complete, {s} succeeded, {f} failed',
  },
  'gps.cancelled': {
    'zh-CN': '写入操作已取消',
    'zh-TW': '寫入操作已取消',
    'en': 'Write operation cancelled',
  },
  'gps.failed': {
    'zh-CN': '写入失败: {e}',
    'zh-TW': '寫入失敗: {e}',
    'en': 'Write failed: {e}',
  },
}

export default dict
