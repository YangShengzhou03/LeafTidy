use crate::cancel;
use crate::models::CancelResult;

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub fn cancel_operation(scope: Option<String>) -> CancelResult {
    if let Some(scope) = scope.as_deref() {
        log::info!("用户取消操作: {scope}");
        cancel::cancel_scope(scope);
    } else {
        log::info!("用户取消操作: 全部任务");
        cancel::cancel_all();
    }
    CancelResult {
        cancelled: true,
        message: cancel::CANCEL_MESSAGE.to_string(),
    }
}
