//! 自动更新：对接服务端 `/api/public/updates` 接口。
//! 启动前装待装包 -> 运行中检查（强制更新后台下载，可选更新仅提示） -> 退出时启动安装界面。

use std::fs;
use std::io::{Read, Write};
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use tauri::Emitter;

const SERVER_ADDRESS: &str = "https://leafboss.top";
/// 服务端更新接口按此商品名匹配（与后台发布的商品名称一致）
const PRODUCT_NAME: &str = "LeafTidy";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
/// cmd 隐藏窗口标志（PowerShell 亦适用）
const CREATE_NO_WINDOW: u32 = 0x0800_0000;
/// 下载进度事件最小间隔
const PROGRESS_EMIT_INTERVAL: Duration = Duration::from_millis(500);

fn update_dir() -> Option<PathBuf> {
    dirs::data_local_dir().map(|d| d.join("leaftidy").join("update"))
}

fn pending_marker(dir: &Path) -> PathBuf {
    dir.join("pending_version.txt")
}

/// 在更新目录中查找已下载的安装包（update.exe / update.msi）
fn find_installer(dir: &Path) -> Option<PathBuf> {
    for name in ["update.exe", "update.msi"] {
        let p = dir.join(name);
        if p.exists() {
            return Some(p);
        }
    }
    None
}

/// 延迟启动安装器：等待 2 秒让本进程退出释放文件锁，再以被动模式启动（/P）。
/// 启动失败会返回错误，调用方据此决定是否继续退出主进程。
fn spawn_delayed_installer(installer: &Path) -> Result<(), String> {
    let path = installer.to_string_lossy().to_string();
    let is_msi = installer
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("msi"));
    let script = if is_msi {
        "& {param($p) Start-Sleep 2; Start-Process msiexec -ArgumentList @('/i', $p, '/qb-')}"
    } else {
        "& {param($p) Start-Sleep 2; Start-Process -FilePath $p -ArgumentList '/P'}"
    };
    log::info!("启动安装程序: {path}");
    std::process::Command::new("powershell")
        .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", script, &path])
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .map(|_| ())
        .map_err(|e| format!("启动安装程序失败: {e}"))
}

fn cleanup_update_dir(dir: &Path) {
    for name in ["update.exe", "update.msi", "pending_version.txt", "update.part", "release_notes.txt"] {
        let _ = fs::remove_file(dir.join(name));
    }
}

/// 启动前处理待安装的更新。返回 true 表示主进程应直接退出、交给安装器接管。
#[must_use]
pub fn handle_pending_install() -> bool {
    let Some(dir) = update_dir() else { return false };
    let marker = pending_marker(&dir);
    if !marker.exists() {
        return false;
    }
    let pending_version = fs::read_to_string(&marker).unwrap_or_default();
    let pending_version = pending_version.trim();
    // 标记为空说明上次写入不完整，不应启动安装器
    if pending_version.is_empty() || pending_version == CURRENT_VERSION {
        return false;
    }
    let Some(installer) = find_installer(&dir) else {
        let _ = fs::remove_file(&marker);
        return false;
    };
    log::info!(
        "发现待安装更新（当前 {CURRENT_VERSION} -> 目标 {pending_version}），启动时安装"
    );
    if let Err(e) = spawn_delayed_installer(&installer) {
        // 启动失败时不退出主进程，保留标记等下次启动重试
        log::error!("{e}");
        return false;
    }
    true
}

/// 更新已装好（当前版本 == 待装版本）的首次启动：向前端推送更新内容并清理更新目录。
pub fn notify_update_installed(app: &tauri::AppHandle) {
    let Some(dir) = update_dir() else { return };
    let marker = pending_marker(&dir);
    if !marker.exists() {
        return;
    }
    let pending_version = fs::read_to_string(&marker).unwrap_or_default();
    if pending_version.trim() != CURRENT_VERSION {
        return;
    }
    let notes = fs::read_to_string(dir.join("release_notes.txt")).unwrap_or_default();
    log::info!("更新 {CURRENT_VERSION} 已安装完成，清理更新目录");
    cleanup_update_dir(&dir);
    let notice = serde_json::json!({
        "version": CURRENT_VERSION,
        "releaseNotes": notes.trim(),
    });
    // setup 阶段前端可能尚未挂载监听器，先把通知存入内存供启动后主动领取，
    // 同时照常 emit 以覆盖前端已就绪的情况。
    if let Ok(mut slot) = UPDATE_NOTICE.lock() {
        *slot = Some(notice.clone());
    }
    let _ = app.emit("update-installed", notice);
}

/// 启动完成后由前端调用，取出 `notify_update_installed` 暂存的更新通知。
#[tauri::command]
pub fn take_update_notice() -> Option<serde_json::Value> {
    UPDATE_NOTICE.lock().ok().and_then(|mut slot| slot.take())
}

static UPDATE_NOTICE: std::sync::Mutex<Option<serde_json::Value>> = std::sync::Mutex::new(None);

/// 退出时启动已下载更新的安装界面。返回 true 表示已安排安装。
#[must_use]
pub fn install_on_exit() -> bool {
    let Some(dir) = update_dir() else { return false };
    if !pending_marker(&dir).exists() {
        return false;
    }
    let Some(installer) = find_installer(&dir) else { return false };
    if let Err(e) = spawn_delayed_installer(&installer) {
        log::error!("{e}");
        return false;
    }
    true
}

/// 启动后台线程：检查更新并下载安装包
pub fn spawn_background_check(app: tauri::AppHandle) {
    thread::spawn(move || {
        if let Err(e) = check_and_download(&app, false) {
            log::warn!("检查/下载更新失败: {e}");
        }
    });
}

/// 立即触发一次更新检查（用户从设置中手动调用）。
#[tauri::command]
pub fn check_for_updates(app: tauri::AppHandle) {
    thread::spawn(move || {
        if let Err(e) = check_and_download(&app, true) {
            log::warn!("检查/下载更新失败: {e}");
            let _ = app.emit("update-error", serde_json::json!({ "message": e }));
        }
    });
}

/// 下载互斥：后台检查与手动检查并发时后到者放弃
static CHECK_IN_PROGRESS: AtomicBool = AtomicBool::new(false);

struct CheckInProgressGuard;
impl Drop for CheckInProgressGuard {
    fn drop(&mut self) {
        CHECK_IN_PROGRESS.store(false, Ordering::Release);
    }
}

fn check_and_download(app: &tauri::AppHandle, notify_no_update: bool) -> Result<(), String> {
    if CHECK_IN_PROGRESS
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        .is_err()
    {
        if notify_no_update {
            return Err("已有更新检查进行中，请稍后重试".to_string());
        }
        log::info!("已有更新检查进行中，跳过本次后台检查");
        return Ok(());
    }
    let _guard = CheckInProgressGuard;
    check_and_download_inner(app, notify_no_update)
}

fn check_and_download_inner(app: &tauri::AppHandle, notify_no_update: bool) -> Result<(), String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {e}"))?;

    let body = client
        .get(format!("{SERVER_ADDRESS}/api/public/updates"))
        .query(&[("productName", PRODUCT_NAME), ("version", CURRENT_VERSION)])
        .send()
        .map_err(|e| format!("请求更新接口失败: {e}"))?
        .text()
        .map_err(|e| format!("读取更新接口响应失败: {e}"))?;
    let resp: serde_json::Value =
        serde_json::from_str(&body).map_err(|e| format!("解析更新接口响应失败: {e}"))?;

    if resp["code"].as_i64() != Some(200) {
        return Err(format!("更新接口返回异常: {resp}"));
    }
    let data = &resp["data"];
    if !data["hasUpdate"].as_bool().unwrap_or(false) {
        log::info!("当前已是最新版本 {CURRENT_VERSION}");
        if notify_no_update {
            let _ = app.emit(
                "update-not-available",
                serde_json::json!({ "version": CURRENT_VERSION }),
            );
        }
        return Ok(());
    }
    let latest_version = data["latestVersion"].as_str().unwrap_or_default().to_string();
    if latest_version.is_empty() {
        return Err("更新接口未返回版本号".to_string());
    }

    let force_update = data["forceUpdate"].as_bool().unwrap_or(false);
    if !notify_no_update && !force_update {
        log::info!("发现可选更新 {latest_version}（当前 {CURRENT_VERSION}），仅通知不下载");
        let _ = app.emit(
            "update-available",
            serde_json::json!({
                "version": latest_version,
                "releaseNotes": data["releaseNotes"].as_str().unwrap_or_default(),
            }),
        );
        return Ok(());
    }

    let url = data["downloadUrl"].as_str().unwrap_or_default().to_string();
    if !url.starts_with("https://") {
        return Err(format!("安装包地址非法: {url}"));
    }
    log::info!("发现新版本 {latest_version}（当前 {CURRENT_VERSION}），开始后台下载");

    let Some(dir) = update_dir() else {
        return Err("无法定位更新目录".to_string());
    };
    fs::create_dir_all(&dir).map_err(|e| format!("创建更新目录失败: {e}"))?;
    let url_path = url.split(['?', '#']).next().unwrap_or(&url);
    let ext = if Path::new(url_path)
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("msi"))
    {
        "msi"
    } else {
        "exe"
    };
    let dest = dir.join(format!("update.{ext}"));
    // 清掉上一次遗留的另一种安装包，避免这次下载失败时仍安装到旧版本
    let stale = dir.join(if ext == "msi" { "update.exe" } else { "update.msi" });
    let _ = fs::remove_file(&stale);
    let _ = fs::remove_file(dir.join("pending_version.txt"));
    download_with_progress(&client, &url, &dest, app)?;

    fs::write(pending_marker(&dir), &latest_version).map_err(|e| format!("写入版本标记失败: {e}"))?;
    fs::write(dir.join("release_notes.txt"), data["releaseNotes"].as_str().unwrap_or_default())
        .map_err(|e| format!("写入更新说明失败: {e}"))?;

    log::info!("新版本 {latest_version} 下载完成，退出时将自动安装");
    let _ = app.emit(
        "update-downloaded",
        serde_json::json!({
            "version": latest_version,
            "releaseNotes": data["releaseNotes"].as_str().unwrap_or_default(),
        }),
    );
    Ok(())
}

fn download_with_progress(
    client: &reqwest::blocking::Client,
    url: &str,
    dest: &Path,
    app: &tauri::AppHandle,
) -> Result<(), String> {
    let mut resp = client
        .get(url)
        .timeout(Duration::from_secs(600))
        .send()
        .map_err(|e| format!("下载安装包失败: {e}"))?;
    if !resp.status().is_success() {
        return Err(format!("下载安装包失败: HTTP {}", resp.status().as_u16()));
    }
    // 跟随重定向后仍需确认最终地址是 HTTPS，防止 https 起始地址被跳到明文/本地文件
    if resp.url().scheme() != "https" {
        return Err(format!("安装包下载被重定向到非 HTTPS 地址: {}", resp.url()));
    }
    let total = resp.content_length().unwrap_or(0);
    let part = dest.with_extension("part");
    let mut file = fs::File::create(&part).map_err(|e| format!("创建临时文件失败: {e}"))?;
    let mut downloaded: u64 = 0;
    let mut last_emit = Instant::now().checked_sub(PROGRESS_EMIT_INTERVAL).unwrap();
    let mut buf = vec![0u8; 64 * 1024];
    let transfer = (|| -> Result<(), String> {
        loop {
            let n = resp.read(&mut buf).map_err(|e| format!("下载中断: {e}"))?;
            if n == 0 {
                break;
            }
            file.write_all(&buf[..n]).map_err(|e| format!("写入临时文件失败: {e}"))?;
            downloaded += n as u64;
            if last_emit.elapsed() >= PROGRESS_EMIT_INTERVAL && total > 0 {
                last_emit = Instant::now();
                let _ = app.emit(
                    "update-download-progress",
                    serde_json::json!({ "percent": (downloaded * 100 / total).min(99) }),
                );
            }
        }
        file.flush().map_err(|e| format!("刷盘失败: {e}"))
    })();
    drop(file);
    if let Err(e) = transfer {
        // 失败时清掉半截文件，避免下次启动误把损坏包当安装包
        let _ = fs::remove_file(&part);
        return Err(e);
    }
    fs::rename(&part, dest).map_err(|e| format!("重命名安装包失败: {e}"))?;
    Ok(())
}
