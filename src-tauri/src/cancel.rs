use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, Weak};

/// 取消操作的错误标记前缀，用于结构化识别取消事件。
/// 前端通过此前缀判断错误是否为用户主动取消，而非依赖中文字符串模糊匹配。
pub const CANCELLED_PREFIX: &str = "[CANCELLED]";

/// 用户取消操作时的友好提示。
pub const CANCEL_MESSAGE: &str = "操作已取消";

/// 每个批量任务持有独立的取消令牌，避免并发任务互相污染取消状态
/// （任务 A 取消不会影响任务 B，任务 B 启动也不会重置任务 A 的取消请求）。
pub struct CancelToken {
    flag: AtomicBool,
    /// 任务种类（organize/rename/duplicate/cleanup/metadata/classify/faces），用于定向取消
    scope: &'static str,
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
/// 改为 Weak 引用：当外部所有 Arc 释放（如 spawn_blocking panic 后 Future 被 drop），
/// 弱引用自动失效，下次 deactivate 时自动清除，防止令牌永久泄漏。
static ACTIVE: Mutex<Vec<Weak<CancelToken>>> = Mutex::new(Vec::new());

/// 创建并注册新任务令牌。任务结束时必须调用 `deactivate` 注销。
pub fn activate(scope: &'static str) -> Arc<CancelToken> {
    let token = Arc::new(CancelToken {
        flag: AtomicBool::new(false),
        scope,
    });
    if let Ok(mut active) = ACTIVE.lock() {
        active.push(Arc::downgrade(&token));
    }
    token
}

/// 任务结束后注销令牌。
pub fn deactivate(token: &Arc<CancelToken>) {
    if let Ok(mut active) = ACTIVE.lock() {
        active.retain(|weak| {
            match weak.upgrade() {
                Some(arc) => !Arc::ptr_eq(&arc, token),
                None => false, // Arc 已释放，顺带清理
            }
        });
    }
}

/// 用户请求取消：只取消指定种类的任务，不影响其他页面的任务。
pub fn cancel_scope(scope: &str) {
    if let Ok(active) = ACTIVE.lock() {
        for weak in active.iter() {
            if let Some(token) = weak.upgrade() {
                if token.scope == scope {
                    token.cancel();
                }
            }
        }
    }
}

/// 是否有批量任务正在运行（用于退出前保护，避免中断写盘）。
pub fn has_active() -> bool {
    ACTIVE.lock().is_ok_and(|active| {
        // 清理已失效的弱引用后判断是否还有存活令牌
        let live_count = active.iter().filter(|w| w.strong_count() > 0).count();
        live_count > 0
    })
}

/// 用户请求取消：取消当前所有在跑的批量任务。
pub fn cancel_all() {
    if let Ok(active) = ACTIVE.lock() {
        for weak in active.iter() {
            if let Some(token) = weak.upgrade() {
                token.cancel();
            }
        }
    }
}

/// 构造结构化的取消错误。需要先写取消日志再返回时使用，避免各处硬编码同一文案。
pub fn cancelled_error() -> String {
    format!("{CANCELLED_PREFIX} 用户已取消操作")
}

/// 检查任务取消状态，若已取消返回结构化取消错误。
/// 用于批量循环内的逐项检查：`cancel::consume(token)?;`
pub fn consume(token: &CancelToken) -> Result<(), String> {
    if token.is_cancelled() {
        Err(cancelled_error())
    } else {
        Ok(())
    }
}
