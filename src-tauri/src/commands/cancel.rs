use crate::cancel;
use crate::models::CancelResult;

#[tauri::command]
pub fn cancel_operation() -> CancelResult {
    log::info!("用户取消操作");
    cancel::cancel_all();
    CancelResult {
        cancelled: true,
        message: cancel::CANCEL_MESSAGE.to_string(),
    }
}
