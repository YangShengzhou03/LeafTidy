use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::models::LogEntry;

/// 日志设置落盘文件名（保存在日志目录，便于随日志目录一起管理）
const SETTINGS_FILE: &str = "settings.json";

#[derive(serde::Serialize, serde::Deserialize)]
struct LogSettings {
    retention_days: u32,
}

/// 从日志文件名解析日期（yyyy-mm-dd-<uuid>.json）；旧版 uuid.json / settings.json 返回 None
fn date_of_file(path: &std::path::Path) -> Option<&str> {
    let name = path.file_name()?.to_str()?;
    let date = name.get(..10)?;
    let valid = name.ends_with(".json")
        && name.as_bytes().get(10) == Some(&b'-')
        && date.bytes().enumerate().all(|(i, b)| i == 4 || i == 7 || b.is_ascii_digit());
    valid.then_some(date)
}

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

        // 保留天数持久化，重启后沿用用户设置
        let retention_days = fs::read_to_string(dir.join(SETTINGS_FILE))
            .ok()
            .and_then(|s| serde_json::from_str::<LogSettings>(&s).ok())
            .map_or(30, |s| s.retention_days);

        Self { log_dir: dir, retention_days: AtomicU32::new(retention_days) }
    }

    pub const fn log_dir(&self) -> &PathBuf {
        &self.log_dir
    }

    pub fn write_event(&self, action: &str, size: u64) -> Result<LogEntry, String> {
        let id = uuid::Uuid::new_v4().to_string();
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        let date = timestamp[..10].to_string();

        let entry = LogEntry {
            id: id.clone(),
            timestamp,
            action: action.to_string(),
            size,
        };

        let file_path = self.log_dir.join(format!("{date}-{id}.json"));
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
        if let Ok(json) = serde_json::to_vec(&LogSettings { retention_days: days }) {
            if let Err(e) = crate::fs_util::atomic_write(&self.log_dir.join(SETTINGS_FILE), &json) {
                log::warn!("保存日志设置失败: {e}");
            }
        }
    }

    pub fn query_logs_by_date(&self, date: &str) -> Result<Vec<LogEntry>, String> {
        let mut results = Vec::new();
        for entry in fs::read_dir(&self.log_dir).map_err(|e| format!("读取日志目录失败: {e}"))? {
            let entry = entry.map_err(|e| format!("读取目录项失败: {e}"))?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            // 文件名带日期，其他日期的日志无需读取内容
            if date_of_file(&path).is_some_and(|d| d != date) {
                continue;
            }
            let content = fs::read_to_string(&path).map_err(|e| format!("读取日志文件失败: {e}"))?;
            // 目录里还可能有 settings.json，无法解析为日志条目的文件直接跳过
            let Ok(entry) = serde_json::from_str::<LogEntry>(&content) else {
                continue;
            };
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
            if date_of_file(&path).is_some_and(|d| d != date) {
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
            let size = entry.metadata().map_or(0, |m| m.len());
            // 新文件名自带日期，统计只需元数据，不再解析日志内容
            if let Some(date) = date_of_file(&path) {
                let (count, total) = summary.entry(date.to_string()).or_insert((0, 0));
                *count += 1;
                *total += size;
                continue;
            }
            // 旧版 uuid.json 无日期，只能读取内容兼容
            let content = fs::read_to_string(&path).map_err(|e| format!("读取日志文件失败: {e}"))?;
            if let Ok(log) = serde_json::from_str::<LogEntry>(&content) {
                let date = log.timestamp[..10.min(log.timestamp.len())].to_string();
                let (count, total) = summary.entry(date).or_insert((0, 0));
                *count += 1;
                *total += size;
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
            // 设置文件不参与过期清理
            if path.file_name().and_then(|n| n.to_str()) == Some(SETTINGS_FILE) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_of_file() {
        assert_eq!(date_of_file(std::path::Path::new("logs/2026-10-23_abc.json")), None);
        assert_eq!(date_of_file(std::path::Path::new("logs/2026-10-23-abc.json")), Some("2026-10-23"));
        assert_eq!(date_of_file(std::path::Path::new("logs/6f2a-uuid.json")), None);
        assert_eq!(date_of_file(std::path::Path::new("logs/settings.json")), None);
    }
}
