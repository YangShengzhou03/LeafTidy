use chrono::{Local, TimeZone};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTask {
    pub id: u32,
    pub recipient: String,
    pub content: String,
    pub content_type: String,
    pub execute_time: i64,
    pub interval: Option<u32>,
    pub max_execute_count: u32,
    pub execute_count: u32,
    pub enabled: bool,
}

pub struct TaskScheduler {
    tasks: Mutex<HashMap<u32, ScheduledTask>>,
}

impl TaskScheduler {
    pub fn new() -> Self {
        Self {
            tasks: Mutex::new(HashMap::new()),
        }
    }

    pub fn add_task(&self, task: ScheduledTask) {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.insert(task.id, task);
    }

    pub fn remove_task(&self, task_id: u32) {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.remove(&task_id);
    }

    pub fn get_pending_tasks(&self) -> Vec<ScheduledTask> {
        let tasks = self.tasks.lock().unwrap();
        let now = Local::now().timestamp();

        tasks
            .values()
            .filter(|task| {
                task.enabled && task.execute_count < task.max_execute_count && task.execute_time <= now
            })
            .cloned()
            .collect()
    }

    pub fn increment_execute_count(&self, task_id: u32) {
        let mut tasks = self.tasks.lock().unwrap();
        if let Some(task) = tasks.get_mut(&task_id) {
            task.execute_count += 1;

            if let Some(interval) = task.interval {
                task.execute_time = Local::now().timestamp() + (interval as i64 * 60);
            }
        }
    }
}

lazy_static! {
    pub static ref SCHEDULER: TaskScheduler = TaskScheduler::new();
}

#[tauri::command]
pub async fn schedule_task(
    id: u32,
    recipient: String,
    content: String,
    content_type: String,
    execute_time: String,
    interval: Option<u32>,
    max_execute_count: u32,
) -> Result<String, String> {
    // 使用本地时区解析时间字符串
    let execute_timestamp = if execute_time.contains('T') {
        // ISO 8601 格式 (带时区)
        chrono::DateTime::parse_from_rfc3339(&execute_time)
            .map(|dt| dt.timestamp())
            .map_err(|e| format!("解析时间失败: {:?}", e))?
    } else {
        // 本地时间格式 "YYYY-MM-DD HH:MM"，使用本地时区
        let naive_dt = chrono::NaiveDateTime::parse_from_str(&execute_time, "%Y-%m-%d %H:%M")
            .map_err(|e| format!("解析时间失败: {:?}", e))?;
        Local.from_local_datetime(&naive_dt).single()
            .map(|dt| dt.timestamp())
            .ok_or_else(|| "无效的本地时间".to_string())?
    };

    let task = ScheduledTask {
        id,
        recipient,
        content,
        content_type,
        execute_time: execute_timestamp,
        interval,
        max_execute_count,
        execute_count: 0,
        enabled: true,
    };

    SCHEDULER.add_task(task);
    log_info!("TaskScheduler", "任务已添加: #{}", id);
    Ok("任务已添加".to_string())
}

#[tauri::command]
pub async fn cancel_task(task_id: u32) -> Result<String, String> {
    SCHEDULER.remove_task(task_id);
    log_info!("TaskScheduler", "任务已取消: #{}", task_id);
    Ok("任务已取消".to_string())
}

#[tauri::command]
pub async fn get_pending_tasks_count() -> Result<u32, String> {
    Ok(SCHEDULER.get_pending_tasks().len() as u32)
}