use crate::models::{SMBConnectionResult, SMBShareInfo};

/// 服务端/共享名会被拼进 UNC 路径并交给 net use，必须在信任边界拒绝路径穿越字符
#[cfg(target_os = "windows")]
fn valid_unc_part(value: &str) -> bool {
    !value.is_empty()
        && !value.contains(['\\', '/', ':', '*', '?', '"', '<', '>', '|', ' '])
        && value != "."
        && value != ".."
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub fn smb_connect(
    server: &str,
    share: &str,
    username: Option<&str>,
    password: Option<&str>,
    persistent: Option<bool>,
) -> Result<SMBConnectionResult, String> {
    let server = server.trim();
    let share = share.trim();
    if !valid_unc_part(server) || !valid_unc_part(share) {
        return Err(format!("非法的服务器或共享名: {server}/{share}"));
    }
    // net use 在缺少密码时会进入交互式提示，GUI 进程没有 stdin 会一直等待
    match (username.filter(|u| !u.trim().is_empty()), password) {
        (Some(_), None) => {
            return Err("使用用户名连接时必须提供密码（空密码请传空字符串）".to_string());
        }
        (None, Some(_)) => return Err("提供密码时必须同时提供用户名".to_string()),
        _ => {}
    }

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

    // ponytail: 密码走 stdin 不走命令行参数，避免被 wmic process / Process Explorer 枚举暴露
    if password.is_some() {
        cmd.stdin(std::process::Stdio::piped());
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
    let mut child = cmd
        .spawn()
        .map_err(|e| format!("执行 net use 失败: {e}"))?;

    if let Some(pass) = password {
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            writeln!(stdin, "{pass}").map_err(|e| format!("写入密码失败: {e}"))?;
        }
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("等待 net use 完成失败: {e}"))?;

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
    // 只接受 \\server\share 形式的 UNC，拒绝任意参数注入
    let unc = unc_path.trim();
    let parts: Vec<&str> = unc
        .strip_prefix(r"\\")
        .unwrap_or("")
        .split('\\')
        .filter(|s| !s.is_empty())
        .collect();
    if parts.len() != 2 || !parts.iter().all(|p| valid_unc_part(p)) {
        return Err(format!("非法的 SMB 路径: {unc_path}"));
    }
    log::info!("断开 SMB 共享: {unc}");

    let output = std::process::Command::new("net")
        .args(["use", unc, "/delete", "/y"])
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
