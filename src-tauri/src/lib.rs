mod cancel;
mod classify;
mod cleanup;
mod commands;
mod duplicates;
mod faces;
mod file_ops;
mod fs_util;
mod geocode;
mod organize;
mod rename;
mod audit_log;
mod imagenet_labels;
mod metadata;
mod models;

use commands::AppState;
use tauri::{
    menu::{MenuBuilder, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager,
};

fn show_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

fn toggle_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            show_main_window(app);
        }
    }
}

/// 启动应用。
///
/// # Panics
///
/// Tauri 应用运行失败（如窗口环境初始化异常）时会 panic。
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志系统：默认 Info 级别，可通过 LEAF_TIDY_LOG=debug 覆盖
    env_logger::Builder::from_env(env_logger::Env::default().filter_or("LEAF_TIDY_LOG", "info"))
        .format_timestamp_millis()
        .init();

    log::info!("轻羽归档启动");
    let log_manager = audit_log::LogManager::new(None);
    let retention = u64::from(log_manager.get_retention_days());
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
            let show_item = MenuItem::with_id(app, "show", "显示/隐藏窗口", true, None::<&str>)?;
            let output_item = MenuItem::with_id(app, "output", "打开输出目录", true, None::<&str>)?;
            let settings_item = MenuItem::with_id(app, "settings", "设置", true, None::<&str>)?;
            let about_item = MenuItem::with_id(app, "about", "关于", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "退出程序", true, None::<&str>)?;
            let menu = MenuBuilder::new(app)
                .item(&show_item)
                .separator()
                .item(&output_item)
                .separator()
                .item(&settings_item)
                .item(&about_item)
                .separator()
                .item(&quit_item)
                .build()?;

            let mut tray = TrayIconBuilder::with_id("main-tray")
                .tooltip("轻羽归档")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => toggle_window(app),
                    "output" => { let _ = app.emit("tray-open-output", ()); }
                    "settings" => { let _ = app.emit("tray-navigate", "settings"); }
                    "about" => { let _ = app.emit("tray-navigate", "about"); }
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
            commands::files::scan_directory,
            commands::files::get_directory_stats,
            commands::files::open_in_explorer,
            commands::files::copy_files_batch,
            commands::geocode::is_geocoder_ready,
            commands::geocode::query_location,
            commands::logs::write_event,
            commands::logs::query_logs_by_date,
            commands::logs::query_daily_summary,
            commands::logs::delete_logs_by_date,
            commands::logs::get_app_settings,
            commands::logs::set_log_retention,
            commands::organize::organize_files_async,
            commands::rename::batch_rename_async,
            commands::duplicates::find_duplicates_async,
            commands::duplicates::clean_duplicates,
            commands::cleanup::scan_low_quality_images_async,
            commands::cleanup::delete_low_quality_images_async,
            commands::cancel::cancel_operation,
            commands::metadata_cmd::fix_date_taken_async,
            commands::metadata_cmd::strip_exif_async,
            commands::metadata_cmd::write_gps_async,
            commands::classify::list_builtin_models,
            commands::classify::classify_images_async,
            commands::classify::validate_model_path,
            commands::faces::face_cluster_async,
            commands::faces::rename_person,
            commands::faces::merge_persons,
            commands::faces::archive_by_person,
            commands::smb::smb_connect,
            commands::smb::smb_disconnect,
            commands::smb::smb_list_connected,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn init_geocode_data() {
    std::thread::spawn(|| {
        let exe_dir = match std::env::current_exe() {
            Ok(path) => {
                let Some(exe_dir) = path.parent() else {
                    log::error!("无法获取父目录");
                    return;
                };
                exe_dir.to_path_buf()
            }
            Err(e) => {
                log::error!("无法获取可执行文件路径: {e}");
                return;
            }
        };

        let mut possible_paths = vec![
            exe_dir.join("geodata/CN.txt"),
            std::path::PathBuf::from("geodata/CN.txt"),
            std::path::PathBuf::from("../geodata/CN.txt"),
        ];
        if let Some(p) = exe_dir.parent() {
            possible_paths.push(p.join("src-tauri/geodata/CN.txt"));
            if let Some(pp) = p.parent() {
                possible_paths.push(pp.join("src-tauri/geodata/CN.txt"));
            }
        }

        log::debug!("地理数据搜索路径: {possible_paths:?}");

        for path in &possible_paths {
            log::debug!("尝试加载地理数据: {}", path.display());
            if let Ok(content) = std::fs::read_to_string(path) {
                let lines = content.lines().count();
                log::debug!("找到文件，行数: {lines}");
                if lines > 100 && geocode::init_geocoder(&content).is_ok() {
                    log::info!("地理编码数据已从 {} 加载成功 ({} 条记录)", path.display(), lines);
                    return;
                }
            }
        }

        log::warn!("无法加载地理编码数据文件，尝试的路径: {possible_paths:?}");
    });
}
