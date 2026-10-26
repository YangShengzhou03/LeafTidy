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
pub mod updater;

use std::sync::atomic::{AtomicBool, Ordering};

use crate::audit_log::LogManager;
pub struct AppState {
    pub log_manager: LogManager,
}

/// 分类任务并发锁，防止并发启动多个任务
static CLASSIFY_LOCK: AtomicBool = AtomicBool::new(false);

/// 批量任务骨架：注册取消令牌 → 阻塞线程池执行 → 注销并返回结果。
/// `scope` 为任务种类，用于用户定向取消；所有耗时批量命令必须经由本函数执行，
/// 避免同步 IO 阻塞 async 运行时。
///
/// 返回优先级：任务执行结果 > 取消状态。任务已完成（Ok）时，即使取消标记已设置，
/// 也返回任务结果，避免丢弃用户已完成的工作；仅当任务本身失败时返回错误。
pub async fn run_batch_task<T, F>(scope: &'static str, task: F) -> Result<T, String>
where
    F: FnOnce(&crate::cancel::CancelToken) -> Result<T, String> + Send + 'static,
    T: Send + 'static,
{
    let start = std::time::Instant::now();
    let token = crate::cancel::activate(scope);
    let task_token = std::sync::Arc::clone(&token);
    let result = tauri::async_runtime::spawn_blocking(move || task(&task_token))
        .await
        .map_err(|e| format!("任务执行异常: {e}"));
    let elapsed = start.elapsed();
    let was_cancelled = token.is_cancelled();
    crate::cancel::deactivate(&token);
    if was_cancelled {
        log::info!("批量任务已取消, 耗时 {elapsed:?}");
    } else {
        log::debug!("批量任务完成, 耗时 {elapsed:?}");
    }
    // 任务执行结果（Ok/Err）优先于取消状态返回。
    // 用户在任务快结束时点取消，已完成的工作不应被丢弃。
    match result {
        Ok(value) => value,
        Err(e) => Err(e),
    }
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
