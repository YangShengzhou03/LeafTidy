pub mod cancel;
pub mod classify;
pub mod cleanup;
pub mod duplicates;
pub mod faces;
pub mod files;
pub mod geocode;
pub mod logs;
pub mod metadata_cmd;
pub mod organize;
pub mod rename;
pub mod smb;

use std::sync::atomic::{AtomicBool, Ordering};

use crate::audit_log::LogManager;
pub struct AppState {
    pub log_manager: LogManager,
}

/// 分类任务并发锁，防止并发启动多个任务
static CLASSIFY_LOCK: AtomicBool = AtomicBool::new(false);

/// 批量任务骨架：注册取消令牌 → 阻塞线程池执行 → 注销并消费取消状态。
/// 所有耗时批量命令必须经由本函数执行，避免同步 IO 阻塞 async 运行时。
pub async fn run_batch_task<T, F>(task: F) -> Result<T, String>
where
    F: FnOnce(&crate::cancel::CancelToken) -> Result<T, String> + Send + 'static,
    T: Send + 'static,
{
    let start = std::time::Instant::now();
    let token = crate::cancel::activate();
    let task_token = std::sync::Arc::clone(&token);
    let result = tauri::async_runtime::spawn_blocking(move || task(&task_token))
        .await
        .map_err(|e| format!("任务执行异常: {e}"));
    let elapsed = start.elapsed();
    let was_cancelled = token.is_cancelled();
    crate::cancel::deactivate(&token);
    crate::cancel::consume(&token)?;
    if was_cancelled {
        log::info!("批量任务已取消, 耗时 {elapsed:?}");
    } else {
        log::debug!("批量任务完成, 耗时 {elapsed:?}");
    }
    result?
}

pub fn acquire_classify_lock() -> bool {
    CLASSIFY_LOCK
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
}

pub fn release_classify_lock() {
    CLASSIFY_LOCK.store(false, Ordering::SeqCst);
}

// 锁守卫：任务结束（含 panic）时自动释放分类锁
pub struct ClassifyLockGuard;
impl Drop for ClassifyLockGuard {
    fn drop(&mut self) {
        release_classify_lock();
    }
}
