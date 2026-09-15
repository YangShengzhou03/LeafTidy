mod cancel;
mod commands;
mod file_ops;
mod geocode;
mod log;
mod metadata;
mod models;

use commands::AppState;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

fn show_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let log_manager = log::LogManager::new(None);
    let retention = log_manager.get_retention_days();
    let _ = log_manager.clean_expired(retention);

    init_geocode_data();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // 二次启动时唤起已有窗口，保证程序单实例运行
            show_main_window(app);
        }))
        .setup(|app| {
            let show_item = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "退出程序", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

            let mut tray = TrayIconBuilder::with_id("main-tray")
                .tooltip("轻羽归档")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => show_main_window(app),
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    // 左键单击托盘图标恢复窗口
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        show_main_window(tray.app_handle());
                    }
                });
            if let Some(icon) = app.default_window_icon() {
                tray = tray.icon(icon.clone());
            }
            tray.build(app)?;
            Ok(())
        })
        .manage(AppState {
            log_manager,
        })
        .invoke_handler(tauri::generate_handler![
            commands::scan_directory,
            commands::get_directory_stats,
            commands::is_geocoder_ready,
            commands::query_location,
            commands::write_log,
            commands::query_logs,
            commands::clear_all_logs,
            commands::delete_log,
            commands::get_app_settings,
            commands::set_log_retention,
            commands::organize_files_async,
            commands::open_in_explorer,
            commands::batch_rename_async,
            commands::find_duplicates_async,
            commands::clean_duplicates,
            commands::move_files_batch,
            commands::scan_auxiliary_files_async,
            commands::cleanup_auxiliary_files_async,
            commands::cancel_operation,
            commands::fix_date_taken_async,
            commands::strip_exif_async,
            commands::write_gps_async,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn init_geocode_data() {
    std::thread::spawn(|| {
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

        let possible_paths = [
            exe_dir.join("geodata/CN.txt"),
            exe_dir.parent().unwrap().join("src-tauri/geodata/CN.txt"),
            exe_dir.parent().unwrap().parent().unwrap().join("src-tauri/geodata/CN.txt"),
            std::path::PathBuf::from("geodata/CN.txt"),
            std::path::PathBuf::from("../geodata/CN.txt"),
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
