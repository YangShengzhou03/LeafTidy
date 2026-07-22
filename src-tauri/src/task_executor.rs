use chrono::{Local, TimeZone};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTask {
    pub id: u64,
    pub recipient: String,
    pub content: String,
    pub content_type: String,
    pub execute_time: i64,
    pub interval: Option<u32>,
    pub max_execute_count: u32,
    pub execute_count: u32,
    pub enabled: bool,
    // 问题19修复：添加单调时钟偏移量，用于检测时间回拨
    #[serde(default)]
    pub monotonic_offset: Option<i64>,
    // 问题 M3 修复：添加重试相关字段
    #[serde(default)]
    pub retry_on_fail: bool,
    #[serde(default)]
    pub max_retry_count: u32,
    #[serde(default)]
    pub current_retry_count: u32,
}

pub struct TaskScheduler {
    tasks: Mutex<HashMap<u64, ScheduledTask>>,
    // 问题19修复：记录上次检查的单调时间
    last_check_instant: Mutex<Option<Instant>>,
    last_check_system_time: Mutex<Option<i64>>,
}

impl TaskScheduler {
    pub fn new() -> Self {
        Self {
            tasks: Mutex::new(HashMap::new()),
            last_check_instant: Mutex::new(None),
            last_check_system_time: Mutex::new(None),
        }
    }

    // 问题19修复：检测系统时间回拨
    fn detect_time_jump(&self) -> bool {
        let now_instant = Instant::now();
        let now_system_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let mut last_instant = self.last_check_instant.lock().unwrap();
        let mut last_system_time = self.last_check_system_time.lock().unwrap();

        let time_jump_detected = if let (Some(prev_instant), Some(prev_system_time)) =
            (*last_instant, *last_system_time)
        {
            let elapsed_monotonic = now_instant.duration_since(prev_instant).as_secs() as i64;
            let elapsed_system = now_system_time - prev_system_time;

            // 如果系统时间比单调时间少超过5秒，说明发生了时间回拨
            elapsed_system < elapsed_monotonic - 5
        } else {
            false
        };

        *last_instant = Some(now_instant);
        *last_system_time = Some(now_system_time);

        // 问题15修复：检测到时间回拨后执行补救措施
        if time_jump_detected {
            self.handle_time_jump(now_system_time);
        }

        time_jump_detected
    }

    // 问题15修复：时间回拨处理函数
    fn handle_time_jump(&self, current_time: i64) {
        log_warn!("TaskScheduler", "检测到系统时间回拨，重新调整任务调度");

        match self.tasks.lock() {
            Ok(mut tasks) => {
                for task in tasks.values_mut() {
                    if task.enabled {
                        // 重新计算下次执行时间
                        if let Some(ref offset) = task.monotonic_offset {
                            // 应用单调时钟偏移，确保任务不会过早执行
                            task.execute_time = current_time + offset;
                            log_info!("TaskScheduler", "任务 #{} 执行时间已调整", task.id);
                        }
                    }
                }
            }
            Err(e) => {
                log_error!("TaskScheduler", "处理时间回拨时获取任务锁失败: {:?}", e);
            }
        }
    }

    // 问题12修复：处理 Mutex 毒化（H3优化：添加重试和验证机制）
    pub fn add_task(&self, task: ScheduledTask) {
        // 尝试获取锁，最多重试3次
        for attempt in 0..3 {
            match self.tasks.lock() {
                Ok(mut tasks) => {
                    tasks.insert(task.id, task);
                    return;
                }
                Err(e) => {
                    log_error!("TaskScheduler", "获取任务锁失败 (尝试 {}): {:?}", attempt + 1, e);

                    if attempt < 2 {
                        // 短暂等待后重试
                        thread::sleep(Duration::from_millis(50));
                        continue;
                    }

                    // 最后一次尝试：恢复数据并验证
                    log_warn!("TaskScheduler", "尝试从中毒的 Mutex 恢复数据");
                    let mut tasks = e.into_inner();

                    // H3 修复：验证数据完整性
                    if tasks.len() > 1000 {
                        log_error!("TaskScheduler", "检测到数据异常，任务数量过多，可能已损坏");
                        // 清空并重新开始，避免损坏数据扩散
                        tasks.clear();
                    }

                    tasks.insert(task.id, task.clone());
                    log_info!("TaskScheduler", "任务 #{} 已在恢复模式下添加", task.id);
                }
            }
        }
    }

    pub fn remove_task(&self, task_id: u64) {
        // H3 修复：同样添加重试机制
        for attempt in 0..3 {
            match self.tasks.lock() {
                Ok(mut tasks) => {
                    tasks.remove(&task_id);
                    return;
                }
                Err(e) => {
                    log_error!("TaskScheduler", "获取任务锁失败 (尝试 {}): {:?}", attempt + 1, e);

                    if attempt < 2 {
                        thread::sleep(Duration::from_millis(50));
                        continue;
                    }

                    log_warn!("TaskScheduler", "尝试从中毒的 Mutex 恢复数据");
                    let mut tasks = e.into_inner();
                    tasks.remove(&task_id);
                }
            }
        }
    }

    pub fn get_pending_tasks(&self) -> Vec<ScheduledTask> {
        match self.tasks.lock() {
            Ok(tasks) => {
                let now = Local::now().timestamp();

                tasks
                    .values()
                    .filter(|task| {
                        task.enabled && task.execute_count < task.max_execute_count && task.execute_time <= now
                    })
                    .cloned()
                    .collect()
            }
            Err(e) => {
                log_error!("TaskScheduler", "获取任务锁失败: {:?}", e);
                Vec::new()
            }
        }
    }

    pub fn increment_execute_count(&self, task_id: u64) {
        match self.tasks.lock() {
            Ok(mut tasks) => {
                if let Some(task) = tasks.get_mut(&task_id) {
                    task.execute_count += 1;

                    if let Some(interval) = task.interval {
                        task.execute_time = Local::now().timestamp() + (interval as i64 * 60);
                    }
                }
            }
            Err(e) => {
                log_error!("TaskScheduler", "获取任务锁失败: {:?}", e);
                let mut tasks = e.into_inner();
                if let Some(task) = tasks.get_mut(&task_id) {
                    task.execute_count += 1;

                    if let Some(interval) = task.interval {
                        task.execute_time = Local::now().timestamp() + (interval as i64 * 60);
                    }
                }
            }
        }
    }
}

lazy_static! {
    pub static ref SCHEDULER: TaskScheduler = TaskScheduler::new();
    // 问题 H5 修复：全局执行锁，防止并发执行
    pub static ref EXECUTION_LOCK: AtomicBool = AtomicBool::new(false);
}

impl TaskScheduler {
    /// 问题 H1 修复：启动后台任务执行循环
    pub fn start_execution_loop(&self) {
        thread::spawn(move || {
            log_info!("TaskScheduler", "后台任务执行循环已启动");

            loop {
                thread::sleep(Duration::from_secs(1));

                // 检测时间回拨
                SCHEDULER.detect_time_jump();

                // 获取待执行任务
                let pending_tasks = SCHEDULER.get_pending_tasks();

                // 执行任务
                for task in pending_tasks {
                    // 问题 H5 修复：尝试获取执行锁
                    if EXECUTION_LOCK.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
                        log_warn!("TaskScheduler", "任务 #{} 执行被跳过（另一个任务正在执行）", task.id);
                        continue;
                    }

                    // 使用 RAII 确保锁释放
                    let _guard = scopeguard::guard((), |_| {
                        EXECUTION_LOCK.store(false, Ordering::SeqCst);
                    });

                    // 问题 M3 修复：带重试的任务执行
                    match SCHEDULER.execute_task_with_retry(&task) {
                        Ok(_) => {
                            log_info!("TaskExecutor", "任务 #{} 执行成功", task.id);
                            SCHEDULER.increment_execute_count(task.id);
                        }
                        Err(e) => {
                            log_error!("TaskExecutor", "任务 #{} 执行失败: {:?}", task.id, e);
                        }
                    }
                }
            }
        });
    }

    /// 问题 M3 修复：带重试机制的任务执行
    fn execute_task_with_retry(&self, task: &ScheduledTask) -> Result<(), String> {
        let max_attempts = if task.retry_on_fail && task.max_retry_count > 0 {
            task.max_retry_count + 1
        } else {
            1
        };

        for attempt in 1..=max_attempts {
            match self.execute_single_task(task) {
                Ok(_) => {
                    return Ok(());
                }
                Err(e) => {
                    if attempt < max_attempts {
                        log_warn!("TaskExecutor", "任务 #{} 执行失败，准备重试 ({}/{}): {:?}",
                            task.id, attempt, max_attempts - 1, e);
                        thread::sleep(Duration::from_secs(5));
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        Err("不应到达此处".to_string())
    }

    /// 执行单个任务
    fn execute_single_task(&self, task: &ScheduledTask) -> Result<(), String> {
        match task.content_type.as_str() {
            "text" => {
                unsafe {
                    crate::commands::send_wechat_message(&task.recipient, &task.content)
                        .map_err(|e| format!("发送消息失败: {:?}", e))?;
                }
            }
            "file" => {
                unsafe {
                    crate::commands::send_wechat_file(&task.recipient, &task.content)
                        .map_err(|e| format!("发送文件失败: {:?}", e))?;
                }
            }
            _ => return Err(format!("不支持的内容类型: {}", task.content_type)),
        }

        Ok(())
    }
}

#[tauri::command]
#[allow(non_snake_case)]
pub async fn schedule_task(
    id: u64,
    recipient: String,
    content: String,
    contentType: String,
    executeTime: String,
    interval: Option<u32>,
    maxExecuteCount: u32,
) -> Result<String, String> {
    log_info!("TaskScheduler", "收到任务调度请求: id={}, recipient={}, content_type={}, execute_time={}", id, recipient, contentType, executeTime);

    // 使用本地时区解析时间字符串
    let execute_timestamp = if executeTime.contains('T') {
        // ISO 8601 格式 (带时区)
        chrono::DateTime::parse_from_rfc3339(&executeTime)
            .map(|dt| dt.timestamp())
            .map_err(|e| {
                log_error!("TaskScheduler", "解析 ISO 8601 时间失败: {:?}, 时间字符串: {}", e, executeTime);
                format!("解析时间失败: {:?}", e)
            })?
    } else {
        // 本地时间格式 "YYYY-MM-DD HH:MM"，使用本地时区
        let naive_dt = chrono::NaiveDateTime::parse_from_str(&executeTime, "%Y-%m-%d %H:%M")
            .map_err(|e| {
                log_error!("TaskScheduler", "解析本地时间失败: {:?}, 时间字符串: {}", e, executeTime);
                format!("解析时间失败: {:?}", e)
            })?;
        Local.from_local_datetime(&naive_dt).single()
            .map(|dt| dt.timestamp())
            .ok_or_else(|| {
                log_error!("TaskScheduler", "无效的本地时间: {}", executeTime);
                "无效的本地时间".to_string()
            })?
    };

    // 问题13修复：验证时间是否在过去（超过5分钟）
    let now = Local::now().timestamp();
    if execute_timestamp < now - 300 {
        log_warn!("TaskScheduler", "任务 #{} 的执行时间在过去，将立即执行", id);
    }

    let task = ScheduledTask {
        id,
        recipient,
        content,
        content_type: contentType,
        execute_time: execute_timestamp,
        interval,
        max_execute_count: maxExecuteCount,
        execute_count: 0,
        enabled: true,
        monotonic_offset: None,
        // 问题 M3 修复：初始化重试字段
        retry_on_fail: false,
        max_retry_count: 0,
        current_retry_count: 0,
    };

    SCHEDULER.add_task(task);
    log_info!("TaskScheduler", "任务已添加: #{}", id);
    Ok("任务已添加".to_string())
}

#[tauri::command]
pub async fn cancel_task(task_id: u64) -> Result<String, String> {
    SCHEDULER.remove_task(task_id);
    log_info!("TaskScheduler", "任务已取消: #{}", task_id);
    Ok("任务已取消".to_string())
}

#[tauri::command]
pub async fn get_pending_tasks_count() -> Result<u32, String> {
    Ok(SCHEDULER.get_pending_tasks().len() as u32)
}