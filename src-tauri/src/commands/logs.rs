use tauri::State;

use crate::models::{AppSettings, LogEntry};

use super::AppState;

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
/// 记录一条操作日志（`JSONL` 写入）。
///
/// # Errors
///
/// - 后端未初始化或日志写入失败时返回错误
pub fn write_event(
    state: State<'_, AppState>,
    action: String,
    size: u64,
) -> Result<LogEntry, String> {
    state.log_manager.write_event(&action, size)
}

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
/// 查询指定日期的日志。
///
/// # Errors
///
/// - 后端未初始化或日志目录读取失败时返回错误
pub fn query_logs_by_date(
    state: State<'_, AppState>,
    date: String,
) -> Result<Vec<LogEntry>, String> {
    log::debug!("查询日志: {date}");
    state.log_manager.query_logs_by_date(&date)
}

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
/// 按日期删除日志文件。
///
/// # Errors
///
/// - 后端未初始化或文件删除失败时返回错误
pub fn delete_logs_by_date(
    state: State<'_, AppState>,
    date: String,
) -> Result<usize, String> {
    log::warn!("删除日期日志: {date}");
    state.log_manager.delete_logs_by_date(&date)
}

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
/// 查询全部日期的每日汇总。
///
/// # Errors
///
/// - 后端未初始化或日志目录读取失败时返回错误
pub fn query_daily_summary(
    state: State<'_, AppState>,
) -> Result<Vec<(String, usize, u64)>, String> {
    log::debug!("查询每日汇总");
    state.log_manager.query_daily_summary()
}

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
#[must_use]
pub fn get_app_settings(state: State<'_, AppState>) -> AppSettings {
    AppSettings {
        log_retention_days: state.log_manager.get_retention_days(),
        log_dir: state.log_manager.log_dir().to_string_lossy().to_string(),
    }
}

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::unnecessary_wraps)]
#[tauri::command]
/// 设置日志保留天数（1/7/14/30），并删除超期日志。
///
/// # Errors
///
/// - 天数不在允许范围内或清理失败时返回错误
pub fn set_log_retention(state: State<'_, AppState>, days: u32) -> Result<(), String> {
    log::info!("设置日志保留天数: {days}");
    state.log_manager.set_retention_days(days);
    // 立即按新保留期清理，而不是等下次启动
    state.log_manager.clean_expired(u64::from(days))
}
