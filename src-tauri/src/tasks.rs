//! 批量任务的取消作用域与进度事件名。
//!
//! 前后端各持一份同名常量（前端见 `src/constants/tasks.ts`），
//! 避免散落的字符串字面量出现拼写不一致导致取消静默失效。

/// 取消作用域：每个可独立取消的操作一个，互不影响。
pub mod scope {
    pub const ORGANIZE: &str = "organize";
    pub const RENAME: &str = "rename";
    pub const DUPLICATE: &str = "duplicate";
    pub const DUPLICATE_CLEAN: &str = "duplicate-clean";
    pub const MOVE: &str = "move";
    pub const CLEANUP: &str = "cleanup";
    pub const CLASSIFY: &str = "classify";
    pub const FACES: &str = "faces";
    pub const FIX_DATE: &str = "fix-date";
    pub const EXIF_CLEAN: &str = "exif-clean";
    pub const WRITE_GPS: &str = "write-gps";
}

/// 进度事件名：一个操作一个频道，避免多页面共用频道串台。
pub mod event {
    pub const ORGANIZE_PROGRESS: &str = "organize-progress";
    pub const RENAME_PROGRESS: &str = "rename-progress";
    pub const DUPLICATE_PROGRESS: &str = "duplicate-progress";
    pub const DUPLICATE_CLEAN_PROGRESS: &str = "duplicate-clean-progress";
    pub const MOVE_PROGRESS: &str = "move-progress";
    pub const CLEANUP_SCAN_PROGRESS: &str = "lowquality-scan-progress";
    pub const CLEANUP_SCAN_RESULT: &str = "lowquality-scan-result";
    pub const CLEANUP_DELETE_PROGRESS: &str = "lowquality-delete-progress";
    pub const CLASSIFY_PROGRESS: &str = "classify-progress";
    pub const FACE_CLUSTER_PROGRESS: &str = "face-cluster-progress";
    pub const FIX_DATE_PROGRESS: &str = "fix-date-progress";
    pub const EXIF_CLEAN_PROGRESS: &str = "exif-clean-progress";
    pub const WRITE_GPS_PROGRESS: &str = "write-gps-progress";
    /// 托盘请求退出时仍有任务在运行，通知前端提示用户
    pub const TRAY_QUIT_BLOCKED: &str = "tray-quit-blocked";
}
