use std::fs;
use std::path::PathBuf;

use crate::models::*;

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
            id: id.clone(),
            timestamp: timestamp.clone(),
            operation_type: operation_type.to_string(),
            source_path: source_path.to_string(),
            target_path: target_path.map(|s| s.to_string()),
            status: status.to_string(),
            detail: detail.map(|s| s.to_string()),
        };

        let log_content = serde_json::to_string(&entry)
            .map_err(|e| format!("序列化日志失败: {}", e))?;

        let date_str = chrono::Local::now().format("%Y%m%d").to_string();
        let log_file = self.log_dir.join(format!("leaf-tidy-{}.leaftidylog", date_str));

        use std::io::Write;
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)
            .map_err(|e| format!("打开日志文件失败: {}", e))?;
        writeln!(file, "{}", log_content)
            .map_err(|e| format!("写入日志失败: {}", e))?;

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
            .map_err(|e| format!("读取日志目录失败: {}", e))?;

        for entry in dir {
            let entry = entry.map_err(|e| format!("读取日志文件失败: {}", e))?;
            let path = entry.path();
            if path.extension().map_or(true, |ext| ext != "leaftidylog") {
                continue;
            }

            let content = fs::read_to_string(&path)
                .map_err(|e| format!("读取日志文件失败: {}", e))?;

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
                        if log_entry.timestamp < start.to_string() {
                            continue;
                        }
                    }
                    if let Some(end) = end_date {
                        if log_entry.timestamp > end.to_string() {
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

    pub fn get_log_stats(&self) -> Result<serde_json::Value, String> {
        let all_logs = self.query_logs(None, None, None, 10000)?;

        let total = all_logs.len();
        let mut type_count: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        let mut success_count = 0;
        let mut fail_count = 0;

        for log in &all_logs {
            *type_count.entry(log.operation_type.clone()).or_insert(0) += 1;
            if log.status == "success" {
                success_count += 1;
            } else {
                fail_count += 1;
            }
        }

        Ok(serde_json::json!({
            "total": total,
            "success_count": success_count,
            "fail_count": fail_count,
            "by_type": type_count,
        }))
    }

    pub fn clean_expired(&self, retention_days: u32) -> Result<usize, String> {
        let cutoff = chrono::Local::now() - chrono::Duration::days(retention_days as i64);
        let cutoff_str = cutoff.format("%Y%m%d").to_string();
        let mut deleted = 0;

        let dir = fs::read_dir(&self.log_dir)
            .map_err(|e| format!("读取日志目录失败: {}", e))?;

        for entry in dir {
            let entry = entry.map_err(|e| format!("读取日志文件失败: {}", e))?;
            let path = entry.path();
            if path.extension().map_or(true, |ext| ext != "leaftidylog") {
                continue;
            }
            if let Some(name) = path.file_stem() {
                let name_str = name.to_string_lossy();
                if name_str.starts_with("leaf-tidy-") {
                    let date_part = &name_str[10..];
                    if *date_part < *cutoff_str {
                        let _ = fs::remove_file(&path);
                        deleted += 1;
                    }
                }
            }
        }

        Ok(deleted)
    }

    pub fn clear_all(&self) -> Result<usize, String> {
        let mut deleted = 0;

        let dir = fs::read_dir(&self.log_dir)
            .map_err(|e| format!("读取日志目录失败: {}", e))?;

        for entry in dir {
            let entry = entry.map_err(|e| format!("读取日志文件失败: {}", e))?;
            let path = entry.path();
            if path.extension().map_or(true, |ext| ext != "leaftidylog") {
                continue;
            }
            let _ = fs::remove_file(&path);
            deleted += 1;
        }

        Ok(deleted)
    }

    pub fn delete_log(&self, log_id: &str) -> Result<bool, String> {
        let dir = fs::read_dir(&self.log_dir)
            .map_err(|e| format!("读取日志目录失败: {}", e))?;

        for entry in dir {
            let entry = entry.map_err(|e| format!("读取日志文件失败: {}", e))?;
            let path = entry.path();
            if path.extension().map_or(true, |ext| ext != "leaftidylog") {
                continue;
            }

            let content = fs::read_to_string(&path)
                .map_err(|e| format!("读取日志文件失败: {}", e))?;

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
                    .map_err(|e| format!("写入日志文件失败: {}", e))?;
                return Ok(true);
            }
        }

        Ok(false)
    }
}
