use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::models::LogEntry;

pub struct LogManager {
    log_dir: PathBuf,
    retention_days: AtomicU32,
}

impl Clone for LogManager {
    fn clone(&self) -> Self {
        Self {
            log_dir: self.log_dir.clone(),
            retention_days: AtomicU32::new(self.retention_days.load(Ordering::Relaxed)),
        }
    }
}

impl LogManager {
    pub fn new(log_dir: Option<PathBuf>) -> Self {
        let dir = log_dir.unwrap_or_else(|| {
            let mut path = dirs::data_dir().unwrap_or_else(std::env::temp_dir);
            path.push("leaf-tidy");
            path.push("logs");
            path
        });

        let _ = fs::create_dir_all(&dir);

        Self { log_dir: dir, retention_days: AtomicU32::new(30) }
    }

    pub const fn log_dir(&self) -> &PathBuf {
        &self.log_dir
    }

    pub fn write_event(&self, action: &str, size: u64) -> Result<LogEntry, String> {
        let id = uuid::Uuid::new_v4().to_string();
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();

        let entry = LogEntry {
            id: id.clone(),
            timestamp,
            action: action.to_string(),
            size,
        };

        let file_path = self.log_dir.join(format!("{id}.json"));
        let json = serde_json::to_string(&entry).map_err(|e| format!("序列化日志失败: {e}"))?;

        let mut file = fs::File::create(&file_path).map_err(|e| format!("创建日志文件失败: {e}"))?;
        writeln!(file, "{json}").map_err(|e| format!("写入日志失败: {e}"))?;

        let size_display = if size > 0 {
            format!(" ({size} 字节)")
        } else {
            String::new()
        };
        println!("{action}{size_display}");

        Ok(entry)
    }

    pub fn get_retention_days(&self) -> u32 {
        self.retention_days.load(Ordering::Relaxed)
    }

    pub fn set_retention_days(&self, days: u32) {
        self.retention_days.store(days, Ordering::Relaxed);
    }

    pub fn query_logs_by_date(&self, date: &str) -> Result<Vec<LogEntry>, String> {
        let mut results = Vec::new();
        for entry in fs::read_dir(&self.log_dir).map_err(|e| format!("读取日志目录失败: {e}"))? {
            let entry = entry.map_err(|e| format!("读取目录项失败: {e}"))?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let content = fs::read_to_string(&path).map_err(|e| format!("读取日志文件失败: {e}"))?;
            let entry: LogEntry = serde_json::from_str(&content).map_err(|e| format!("解析日志失败: {e}"))?;
            if entry.timestamp.starts_with(date) {
                results.push(entry);
            }
        }
        results.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        Ok(results)
    }

    pub fn delete_logs_by_date(&self, date: &str) -> Result<usize, String> {
        let mut count = 0;
        for entry in fs::read_dir(&self.log_dir).map_err(|e| format!("读取日志目录失败: {e}"))? {
            let entry = entry.map_err(|e| format!("读取目录项失败: {e}"))?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let content = fs::read_to_string(&path).map_err(|e| format!("读取日志文件失败: {e}"))?;
            if let Ok(log) = serde_json::from_str::<LogEntry>(&content) {
                if log.timestamp.starts_with(date) {
                    fs::remove_file(&path).map_err(|e| format!("删除日志失败: {e}"))?;
                    count += 1;
                }
            }
        }
        Ok(count)
    }

    // 汇总日志文件本身在磁盘上的大小
    pub fn query_daily_summary(&self) -> Result<Vec<(String, usize, u64)>, String> {
        let mut summary: std::collections::HashMap<String, (usize, u64)> = std::collections::HashMap::new();
        for entry in fs::read_dir(&self.log_dir).map_err(|e| format!("读取日志目录失败: {e}"))? {
            let entry = entry.map_err(|e| format!("读取目录项失败: {e}"))?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let content = fs::read_to_string(&path).map_err(|e| format!("读取日志文件失败: {e}"))?;
            if let Ok(log) = serde_json::from_str::<LogEntry>(&content) {
                let date = log.timestamp[..10.min(log.timestamp.len())].to_string();
                let (count, size) = summary.entry(date).or_insert((0, 0));
                *count += 1;
                *size += entry.metadata().map_or(0, |m| m.len());
            }
        }
        let mut result: Vec<(String, usize, u64)> = summary.into_iter().map(|(d, (c, s))| (d, c, s)).collect();
        result.sort_by(|a, b| b.0.cmp(&a.0));
        Ok(result)
    }

    pub fn clean_expired(&self, days: u64) -> Result<(), String> {
        let cutoff = chrono::Local::now() - chrono::Duration::days(days.cast_signed());

        for entry in fs::read_dir(&self.log_dir).map_err(|e| format!("读取日志目录失败: {e}"))? {
            let entry = entry.map_err(|e| format!("读取目录项失败: {e}"))?;
            let path = entry.path();

            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }

            let metadata = entry.metadata().map_err(|e| format!("读取文件元数据失败: {e}"))?;
            let modified = metadata.modified().map_err(|e| format!("读取修改时间失败: {e}"))?;
            let modified_time: chrono::DateTime<chrono::Local> = modified.into();

            if modified_time < cutoff {
                let _ = fs::remove_file(&path);
            }
        }

        Ok(())
    }
}
