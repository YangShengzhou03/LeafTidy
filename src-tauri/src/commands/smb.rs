use crate::models::{SMBConnectionResult, SMBShareInfo};

#[cfg(target_os = "windows")]
#[tauri::command]
pub fn smb_connect(
    server: &str,
    share: &str,
    username: Option<&str>,
    password: Option<&str>,
    persistent: Option<bool>,
) -> Result<SMBConnectionResult, String> {
    let unc_path = format!(r"\\{server}\{share}");

    // 检查是否已可访问（可能已通过其他方式连接）
    if std::path::Path::new(&unc_path).exists() {
        log::info!("SMB 共享已可访问，无需重新连接: {unc_path}");
        return Ok(SMBConnectionResult {
            unc_path,
            newly_connected: false,
        });
    }

    // 构建 net use 命令
    let mut cmd = std::process::Command::new("net");
    cmd.args(["use", &unc_path]);

    if let Some(pass) = &password {
        cmd.arg(pass);
    }

    if let Some(user) = &username {
        cmd.args(["/user:", user]);
    }

    if persistent.unwrap_or(false) {
        cmd.arg("/persistent:yes");
    } else {
        cmd.arg("/persistent:no");
    }

    log::info!("执行 SMB 连接: net use {unc_path}");
    let output = cmd
        .output()
        .map_err(|e| format!("执行 net use 失败: {e}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        let msg = if stderr.trim().is_empty() {
            stdout.trim().to_string()
        } else {
            stderr.trim().to_string()
        };
        log::error!("SMB 连接失败: {msg}");
        return Err(format!("连接失败: {msg}"));
    }

    // 验证连接是否真正可用
    if !std::path::Path::new(&unc_path).exists() {
        log::warn!("net use 成功但路径仍不可访问: {unc_path}");
        return Err("连接成功但无法访问共享路径，请检查共享名和权限".to_string());
    }

    log::info!("SMB 连接成功: {unc_path}");
    Ok(SMBConnectionResult {
        unc_path,
        newly_connected: true,
    })
}

#[cfg(not(target_os = "windows"))]
#[tauri::command]
pub fn smb_connect(
    _server: &str,
    _share: &str,
    _username: Option<&str>,
    _password: Option<&str>,
    _persistent: Option<bool>,
) -> Result<SMBConnectionResult, String> {
    Err("SMB 连接功能目前仅支持 Windows".to_string())
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub fn smb_disconnect(unc_path: &str) -> Result<(), String> {
    log::info!("断开 SMB 共享: {unc_path}");

    let output = std::process::Command::new("net")
        .args(["use", unc_path, "/delete", "/y"])
        .output()
        .map_err(|e| format!("执行 net use /delete 失败: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let msg = if stderr.trim().is_empty() {
            stdout.trim().to_string()
        } else {
            stderr.trim().to_string()
        };
        log::error!("断开 SMB 失败: {msg}");
        return Err(format!("断开失败: {msg}"));
    }

    log::info!("SMB 断开成功: {unc_path}");
    Ok(())
}

#[cfg(not(target_os = "windows"))]
#[tauri::command]
pub fn smb_disconnect(_unc_path: &str) -> Result<(), String> {
    Err("SMB 断开功能目前仅支持 Windows".to_string())
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub fn smb_list_connected() -> Result<Vec<SMBShareInfo>, String> {
    log::info!("列出已连接的 SMB 共享");

    let output = std::process::Command::new("net")
        .args(["use"])
        .output()
        .map_err(|e| format!("执行 net use 失败: {e}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut shares = Vec::new();

    for line in stdout.lines() {
        // net use 输出行示例:
        // 好好    \\192.168.1.100\photo       Microsoft Windows Network
        // 或者:
        // \\192.168.1.100\photo            Microsoft Windows Network
        let line = line.trim();
        if line.starts_with(r"\\") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(unc) = parts.first() {
                // 解析 \\server\share
                let unc_str = unc.trim_start_matches(r"\\");
                if let Some((server, share_part)) = unc_str.split_once('\\') {
                    shares.push(SMBShareInfo {
                        unc_path: unc.to_string(),
                        server: server.to_string(),
                        share: share_part.to_string(),
                        accessible: std::path::Path::new(unc).exists(),
                    });
                }
            }
        }
    }

    log::info!("发现 {} 个已连接的 SMB 共享", shares.len());
    Ok(shares)
}

#[cfg(not(target_os = "windows"))]
#[tauri::command]
pub fn smb_list_connected() -> Result<Vec<SMBShareInfo>, String> {
    Ok(Vec::new())
}
