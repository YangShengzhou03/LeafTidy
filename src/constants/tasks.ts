/// 批量任务的作用域与进度事件名（与 Rust 端 `src-tauri/src/tasks.rs` 保持一致）。
/// 页面统一从这里取值，避免字符串拼写不一致导致取消失效或进度串台。
export const TASK_SCOPE = {
  organize: 'organize',
  rename: 'rename',
  duplicate: 'duplicate',
  duplicateClean: 'duplicate-clean',
  move: 'move',
  cleanup: 'cleanup',
  classify: 'classify',
  faces: 'faces',
  fixDate: 'fix-date',
  exifClean: 'exif-clean',
  writeGps: 'write-gps',
} as const

export const TASK_EVENT = {
  organizeProgress: 'organize-progress',
  renameProgress: 'rename-progress',
  duplicateProgress: 'duplicate-progress',
  duplicateCleanProgress: 'duplicate-clean-progress',
  moveProgress: 'move-progress',
  cleanupScanProgress: 'lowquality-scan-progress',
  cleanupScanResult: 'lowquality-scan-result',
  cleanupDeleteProgress: 'lowquality-delete-progress',
  classifyProgress: 'classify-progress',
  faceClusterProgress: 'face-cluster-progress',
  fixDateProgress: 'fix-date-progress',
  exifCleanProgress: 'exif-clean-progress',
  writeGpsProgress: 'write-gps-progress',
  trayQuitBlocked: 'tray-quit-blocked',
} as const
