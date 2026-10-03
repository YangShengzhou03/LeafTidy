use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// 取消操作的错误标记前缀，用于结构化识别取消事件。
/// 前端通过此前缀判断错误是否为用户主动取消，而非依赖中文字符串模糊匹配。
pub const CANCELLED_PREFIX: &str = "[CANCELLED]";

/// 用户取消操作时的友好提示。
pub const CANCEL_MESSAGE: &str = "操作已取消";

/// 每个批量任务持有独立的取消令牌，避免并发任务互相污染取消状态
/// （任务 A 取消不会影响任务 B，任务 B 启动也不会重置任务 A 的取消请求）。
pub struct CancelToken {
    flag: AtomicBool,
}

impl CancelToken {
    pub fn is_cancelled(&self) -> bool {
        self.flag.load(Ordering::SeqCst)
    }

    fn cancel(&self) {
        self.flag.store(true, Ordering::SeqCst);
    }
}

/// 当前在跑的任务令牌表。`run_batch_task` 启动时注册、结束时注销。
static ACTIVE: Mutex<Vec<Arc<CancelToken>>> = Mutex::new(Vec::new());

/// 创建并注册新任务令牌。任务结束时必须调用 `deactivate` 注销。
pub fn activate() -> Arc<CancelToken> {
    let token = Arc::new(CancelToken { flag: AtomicBool::new(false) });
    if let Ok(mut active) = ACTIVE.lock() {
        active.push(Arc::clone(&token));
    }
    token
}

/// 任务结束后注销令牌。
pub fn deactivate(token: &Arc<CancelToken>) {
    if let Ok(mut active) = ACTIVE.lock() {
        active.retain(|t| !Arc::ptr_eq(t, token));
    }
}

/// 用户请求取消：取消当前所有在跑的批量任务。
pub fn cancel_all() {
    if let Ok(active) = ACTIVE.lock() {
        for token in active.iter() {
            token.cancel();
        }
    }
}

/// 检查任务取消状态，若已取消返回结构化取消错误。
/// 用于批量循环内的逐项检查：`cancel::consume(token)?;`
pub fn consume(token: &CancelToken) -> Result<(), String> {
    if token.is_cancelled() {
        Err(format!("{CANCELLED_PREFIX} 用户已取消操作"))
    } else {
        Ok(())
    }
}
