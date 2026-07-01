mod commands;
mod file_ops;
mod geocode;
mod log;
mod metadata;
mod models;

use commands::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let log_manager = log::LogManager::new(None);
    let _ = log_manager.clean_expired(30);

    // 尝试初始化地理编码器
    init_geocode_data();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            log_manager,
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_directory,
            commands::scan_directory,
            commands::list_subdirs,
            commands::get_directory_stats,
            commands::rename_file_command,
            commands::delete_to_trash,
            commands::compute_md5,
            commands::sanitize_filename,
            commands::format_file_size,
            commands::read_exif,
            commands::reverse_geocode,
            commands::init_geocoder,
            commands::is_geocoder_ready,
            commands::get_file_category,
            commands::is_image_file,
            commands::is_video_file,
            commands::get_file_detail,
            commands::write_log,
            commands::query_logs,
            commands::get_log_stats,
            commands::clean_expired_logs,
            commands::get_app_info,
            commands::organize_files,
            commands::open_in_explorer,
            commands::batch_rename,
            commands::find_duplicates,
            commands::clean_duplicates,
            commands::move_files_batch,
            commands::scan_auxiliary_files,
            commands::cleanup_auxiliary_files,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// 初始化地理编码数据（异步加载）
fn init_geocode_data() {
    // 在后台线程中加载，避免阻塞 UI
    std::thread::spawn(|| {
        // 获取可执行文件所在目录
        let exe_dir = match std::env::current_exe() {
            Ok(path) => match path.parent() {
                Some(p) => p.to_path_buf(),
                None => {
                    eprintln!("无法获取父目录");
                    return;
                }
            },
            Err(e) => {
                eprintln!("无法获取可执行文件路径: {}", e);
                return;
            }
        };
        
        // 尝试多个可能的路径
        let possible_paths = [
            exe_dir.join("geodata/CN.txt"),           // 生产模式: 可执行文件目录/geodata/CN.txt
            exe_dir.parent().unwrap().join("src-tauri/geodata/CN.txt"), // 开发模式: 项目根/src-tauri/geodata/CN.txt
            exe_dir.parent().unwrap().parent().unwrap().join("src-tauri/geodata/CN.txt"), // 开发模式深层
            std::path::PathBuf::from("geodata/CN.txt"), // 当前工作目录 (src-tauri)
            std::path::PathBuf::from("../geodata/CN.txt"), // 上级目录
        ];

        for path in &possible_paths {
            println!("尝试加载地理数据: {}", path.display());
            if let Ok(content) = std::fs::read_to_string(path) {
                let lines = content.lines().count();
                println!("找到文件，行数: {}", lines);
                if lines > 100 {
                    if geocode::init_geocoder(&content).is_ok() {
                        println!("地理编码数据已从 {} 加载成功 ({} 条记录)", path.display(), lines);
                        return;
                    }
                }
            }
        }

        eprintln!("无法加载地理编码数据文件，尝试的路径: {:?}", possible_paths);
    });
}
