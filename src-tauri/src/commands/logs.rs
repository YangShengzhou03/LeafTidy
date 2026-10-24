use tauri::State;

use crate::models::{AppSettings, LogEntry};

use super::AppState;

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub fn write_event(
    state: State<'_, AppState>,
    action: String,
    size: u64,
) -> Result<LogEntry, String> {
    state.log_manager.write_event(&action, size)
}

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub fn query_logs_by_date(
    state: State<'_, AppState>,
    date: String,
) -> Result<Vec<LogEntry>, String> {
    log::debug!("查询日志: {date}");
    state.log_manager.query_logs_by_date(&date)
}

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub fn delete_logs_by_date(
    state: State<'_, AppState>,
    date: String,
) -> Result<usize, String> {
    log::warn!("删除日期日志: {date}");
    state.log_manager.delete_logs_by_date(&date)
}

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub fn query_daily_summary(
    state: State<'_, AppState>,
) -> Result<Vec<(String, usize, u64)>, String> {
    log::debug!("查询每日汇总");
    state.log_manager.query_daily_summary()
}

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub fn get_app_settings(state: State<'_, AppState>) -> AppSettings {
    AppSettings {
        log_retention_days: state.log_manager.get_retention_days(),
        log_dir: state.log_manager.log_dir().to_string_lossy().to_string(),
    }
}

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::unnecessary_wraps)]
#[tauri::command]
pub fn set_log_retention(state: State<'_, AppState>, days: u32) -> Result<(), String> {
    log::info!("设置日志保留天数: {days}");
    state.log_manager.set_retention_days(days);
    Ok(())
}
