use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::models::LogEntry;

pub struct LogManager {
    log_dir: PathBuf,
}

impl LogManager {
    pub fn new(log_dir: Option<PathBuf>) -> Self {
        let dir = log_dir.unwrap_or_else(|| {
            let mut path = std::env::temp_dir();
            path.push("leaf-tidy-logs");
            path
        });

        let _ = fs::create_dir_all(&dir);

        Self { log_dir: dir }
    }

    pub fn write_log(
        &self,
        operation_type: &str,
        source_path: &str,
        target_path: Option<&str>,
        status: &str,
        detail: Option<&str>,
    ) -> Result<LogEntry, String> {
        let id = uuid::Uuid::new_v4().to_string();
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();

        let entry = LogEntry {
            id,
            timestamp,
            operation_type: operation_type.to_string(),
            source_path: source_path.to_string(),
            target_path: target_path.map(ToString::to_string),
            status: status.to_string(),
            detail: detail.map(ToString::to_string),
        };

        let log_content = serde_json::to_string(&entry)
            .map_err(|e| format!("序列化日志失败: {e}"))?;

        let date_str = chrono::Local::now().format("%Y%m%d").to_string();
        let log_file = self.log_dir.join(format!("leaf-tidy-{date_str}.leaftidylog"));

        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)
            .map_err(|e| format!("打开日志文件失败: {e}"))?;
        writeln!(file, "{log_content}")
            .map_err(|e| format!("写入日志失败: {e}"))?;

        Ok(entry)
    }

    pub fn query_logs(
        &self,
        operation_type: Option<&str>,
        start_date: Option<&str>,
        end_date: Option<&str>,
        limit: usize,
    ) -> Result<Vec<LogEntry>, String> {
        let mut all_entries = Vec::new();

        let dir = fs::read_dir(&self.log_dir)
            .map_err(|e| format!("读取日志目录失败: {e}"))?;

        for entry in dir {
            let entry = entry.map_err(|e| format!("读取日志文件失败: {e}"))?;
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != "leaftidylog") {
                continue;
            }

            let content = fs::read_to_string(&path)
                .map_err(|e| format!("读取日志文件失败: {e}"))?;

            for line in content.lines() {
                if line.trim().is_empty() {
                    continue;
                }
                if let Ok(log_entry) = serde_json::from_str::<LogEntry>(line) {
                    if let Some(op_type) = operation_type {
                        if log_entry.operation_type != op_type {
                            continue;
                        }
                    }
                    if let Some(start) = start_date {
                        if log_entry.timestamp.as_str() < start {
                            continue;
                        }
                    }
                    if let Some(end) = end_date {
                        if log_entry.timestamp.as_str() > end {
                            continue;
                        }
                    }
                    all_entries.push(log_entry);
                }
            }
        }

        all_entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        all_entries.truncate(limit);

        Ok(all_entries)
    }

    pub fn clean_expired(&self, retention_days: u32) -> Result<usize, String> {
        let cutoff = chrono::Local::now() - chrono::Duration::days(i64::from(retention_days));
        let cutoff_str = cutoff.format("%Y%m%d").to_string();
        let mut deleted = 0;

        let dir = fs::read_dir(&self.log_dir)
            .map_err(|e| format!("读取日志目录失败: {e}"))?;

        for entry in dir {
            let entry = entry.map_err(|e| format!("读取日志文件失败: {e}"))?;
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != "leaftidylog") {
                continue;
            }
            if let Some(name_str) = path.file_stem().map(|s| s.to_string_lossy()) {
                if let Some(date_part) = name_str.strip_prefix("leaf-tidy-") {
                    if *date_part < *cutoff_str {
                        let _ = fs::remove_file(&path);
                        deleted += 1;
                    }
                }
            }
        }

        Ok(deleted)
    }

    fn settings_file(&self) -> PathBuf {
        self.log_dir.join("settings.json")
    }

    /// 读取日志保留天数（未配置时默认 30 天）
    pub fn get_retention_days(&self) -> u32 {
        fs::read_to_string(self.settings_file())
            .ok()
            .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
            .and_then(|v| v.get("log_retention_days").and_then(serde_json::Value::as_u64))
            .map_or(30, |d| {
                // ponytail: 有意截断，值域由调用方保证；如需严格检查改用 try_from
                #[allow(clippy::cast_possible_truncation)]
                {
                    d as u32
                }
            })
    }

    /// 保存日志保留天数并立即清理过期日志
    pub fn set_retention_days(&self, days: u32) -> Result<(), String> {
        // 目录可能在运行中被外部删除（如临时目录被清理工具移除），写入前确保存在
        fs::create_dir_all(&self.log_dir).map_err(|e| format!("创建日志目录失败: {e}"))?;
        let json = serde_json::json!({ "log_retention_days": days }).to_string();
        fs::write(self.settings_file(), json).map_err(|e| format!("写入设置失败: {e}"))?;
        self.clean_expired(days)?;
        Ok(())
    }

    pub const fn log_dir(&self) -> &PathBuf {
        &self.log_dir
    }

    pub fn clear_all(&self) -> Result<usize, String> {
        let mut deleted = 0;

        let dir = fs::read_dir(&self.log_dir)
            .map_err(|e| format!("读取日志目录失败: {e}"))?;

        for entry in dir {
            let entry = entry.map_err(|e| format!("读取日志文件失败: {e}"))?;
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != "leaftidylog") {
                continue;
            }
            let _ = fs::remove_file(&path);
            deleted += 1;
        }

        Ok(deleted)
    }

    pub fn delete_log(&self, log_id: &str) -> Result<bool, String> {
        let dir = fs::read_dir(&self.log_dir)
            .map_err(|e| format!("读取日志目录失败: {e}"))?;

        for entry in dir {
            let entry = entry.map_err(|e| format!("读取日志文件失败: {e}"))?;
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != "leaftidylog") {
                continue;
            }

            let content = fs::read_to_string(&path)
                .map_err(|e| format!("读取日志文件失败: {e}"))?;

            let mut remaining_lines = Vec::new();
            let mut found = false;

            for line in content.lines() {
                if line.trim().is_empty() {
                    continue;
                }
                if let Ok(log_entry) = serde_json::from_str::<LogEntry>(line) {
                    if log_entry.id == log_id {
                        found = true;
                        continue;
                    }
                }
                remaining_lines.push(line);
            }

            if found {
                let new_content = remaining_lines.join("\n");
                fs::write(&path, new_content)
                    .map_err(|e| format!("写入日志文件失败: {e}"))?;
                return Ok(true);
            }
        }

        Ok(false)
    }
}
