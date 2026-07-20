use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

const MAX_LOG_FILES: usize = 7; // 最多保存7个日志文件

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogFile {
    pub name: String,
    pub date: String,
    pub size: String,
    pub path: String,
}

pub struct Logger {
    log_dir: PathBuf,
    current_file: Mutex<Option<PathBuf>>,
}

impl Logger {
    pub fn new() -> Self {
        let log_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("LeafMaster")
            .join("logs");

        fs::create_dir_all(&log_dir).ok();

        Logger {
            log_dir,
            current_file: Mutex::new(None),
        }
    }

    pub fn log(&self, level: &str, module: &str, message: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let log_entry = format!("[{}] [{}] [{}] {}\n", timestamp, level, module, message);

        // 打印到控制台
        print!("{}", log_entry);

        // 写入文件
        if let Ok(mut current_file) = self.current_file.lock() {
            // 检查是否需要更新日志文件（每天一个文件）
            let today_file = self.get_current_log_file();
            if current_file.is_none() || *current_file != Some(today_file.clone()) {
                *current_file = Some(today_file);
                self.rotate_logs();
            }

            if let Some(ref path) = *current_file {
                if let Ok(mut file) = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path)
                {
                    let _ = file.write_all(log_entry.as_bytes());
                }
            }
        }
    }

    fn get_current_log_file(&self) -> PathBuf {
        let today = Local::now().format("%Y-%m-%d").to_string();
        self.log_dir.join(format!("leaf-master-{}.log", today))
    }

    fn rotate_logs(&self) {
        let mut log_files: Vec<PathBuf> = fs::read_dir(&self.log_dir)
            .ok()
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().extension().map(|ext| ext == "log").unwrap_or(false))
                    .map(|e| e.path())
                    .collect()
            })
            .unwrap_or_default();

        // 按文件名排序（文件名包含日期）
        log_files.sort();

        // 删除最老的日志文件（保留最新的 MAX_LOG_FILES 个）
        while log_files.len() > MAX_LOG_FILES {
            if let Some(old_file) = log_files.first() {
                let _ = fs::remove_file(old_file);
                log_files.remove(0);
            } else {
                break;
            }
        }
    }

    pub fn get_log_files(&self) -> Vec<LogFile> {
        let mut files = Vec::new();

        if let Ok(entries) = fs::read_dir(&self.log_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.extension().map(|ext| ext == "log").unwrap_or(false) {
                    if let Ok(metadata) = entry.metadata() {
                        let name = path.file_name().unwrap().to_string_lossy().to_string();
                        let size = format_size(metadata.len());
                        let date = name.replace("leaf-master-", "").replace(".log", "");

                        files.push(LogFile {
                            name,
                            date,
                            size,
                            path: path.to_string_lossy().to_string(),
                        });
                    }
                }
            }
        }

        // 按日期排序（最新的在前）
        files.sort_by(|a, b| b.date.cmp(&a.date));
        files
    }

    pub fn read_log_file(&self, file_name: &str) -> std::io::Result<String> {
        let path = self.log_dir.join(file_name);
        fs::read_to_string(path)
    }

    pub fn delete_log_file(&self, file_name: &str) -> std::io::Result<()> {
        let path = self.log_dir.join(file_name);
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }

    pub fn delete_all_logs(&self) -> std::io::Result<()> {
        if let Ok(entries) = fs::read_dir(&self.log_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.extension().map(|ext| ext == "log").unwrap_or(false) {
                    fs::remove_file(path)?;
                }
            }
        }
        Ok(())
    }

    pub fn info(&self, module: &str, message: &str) {
        self.log("INFO", module, message);
    }

    pub fn warn(&self, module: &str, message: &str) {
        self.log("WARN", module, message);
    }

    pub fn error(&self, module: &str, message: &str) {
        self.log("ERROR", module, message);
    }

    pub fn debug(&self, module: &str, message: &str) {
        self.log("DEBUG", module, message);
    }
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;

    if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

// Tauri commands for log management
#[tauri::command]
pub async fn get_log_files() -> Result<Vec<LogFile>, String> {
    Ok(LOGGER.get_log_files())
}

#[tauri::command]
pub async fn read_log_file(file_name: String) -> Result<String, String> {
    LOGGER.read_log_file(&file_name).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_log_file(file_name: String) -> Result<(), String> {
    LOGGER.delete_log_file(&file_name).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_all_logs() -> Result<(), String> {
    LOGGER.delete_all_logs().map_err(|e| e.to_string())
}

// 前端日志记录API
#[tauri::command]
pub async fn log_frontend(level: String, module: String, message: String) -> Result<(), String> {
    match level.to_uppercase().as_str() {
        "INFO" => LOGGER.info(&module, &message),
        "WARN" => LOGGER.warn(&module, &message),
        "ERROR" => LOGGER.error(&module, &message),
        "DEBUG" => LOGGER.debug(&module, &message),
        _ => LOGGER.info(&module, &message),
    }
    Ok(())
}

// 全局日志实例
lazy_static::lazy_static! {
    pub static ref LOGGER: Logger = Logger::new();
}

#[macro_export]
macro_rules! log_info {
    ($module:expr, $($arg:tt)*) => {
        $crate::logger::LOGGER.info($module, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warn {
    ($module:expr, $($arg:tt)*) => {
        $crate::logger::LOGGER.warn($module, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($module:expr, $($arg:tt)*) => {
        $crate::logger::LOGGER.error($module, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_debug {
    ($module:expr, $($arg:tt)*) => {
        $crate::logger::LOGGER.debug($module, &format!($($arg)*))
    };
}