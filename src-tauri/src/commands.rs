use serde::{Deserialize, Serialize};
use std::path::Path;
use windows::{
    core::*,
    Win32::{
        Foundation::*,
        Graphics::Gdi::*,
        System::{
            Com::*,
            Memory::*,
            DataExchange::*,
        },
        UI::{
            Accessibility::*,
            Input::KeyboardAndMouse::*,
            WindowsAndMessaging::*,
        },
    },
};

const DEFAULT_DELAY: std::time::Duration = std::time::Duration::from_millis(500);
const SHORT_DELAY: std::time::Duration = std::time::Duration::from_millis(100);

#[derive(Debug, Serialize, Deserialize)]
pub struct WeChatStatus {
    pub online: bool,
    pub username: Option<String>,
    pub login_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendFileResponse {
    pub success: bool,
    pub message: String,
}

#[tauri::command]
pub fn get_wechat_status() -> WeChatStatus {
    unsafe {
        match detect_wechat() {
            Ok(username) => WeChatStatus {
                online: true,
                username: Some(username),
                login_time: Some(chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()),
            },
            Err(_) => WeChatStatus {
                online: false,
                username: None,
                login_time: None,
            },
        }
    }
}

#[tauri::command]
pub fn send_message(recipient: String, message: String) -> SendMessageResponse {
    unsafe {
        match send_wechat_message(&recipient, &message) {
            Ok(_) => SendMessageResponse {
                success: true,
                message: format!("已发送给 {}", recipient),
            },
            Err(e) => SendMessageResponse {
                success: false,
                message: format!("发送失败: {:?}", e),
            },
        }
    }
}

#[tauri::command]
pub fn send_file(recipient: String, filepath: String) -> SendFileResponse {
    unsafe {
        match send_wechat_file(&recipient, &filepath) {
            Ok(_) => SendFileResponse {
                success: true,
                message: format!("文件已发送给 {}", recipient),
            },
            Err(e) => SendFileResponse {
                success: false,
                message: format!("发送文件失败: {:?}", e),
            },
        }
    }
}

// ============ 微信检测 ============

unsafe fn detect_wechat() -> Result<String> {
    let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

    let automation: IUIAutomation = CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER)?;
    let root = automation.GetRootElement()?;

    let window = locate_wechat_window(&automation, &root)?;
    activate_wechat_window(&window)?;

    if let Ok(username) = get_avatar_username(&automation, &window) {
        return Ok(username);
    }

    if let Ok(username) = get_username_from_tab(&automation, &window, &root) {
        return Ok(username);
    }

    Err(Error::from(E_FAIL))
}

unsafe fn get_avatar_username(automation: &IUIAutomation, window: &IUIAutomationElement) -> Result<String> {
    let avatar = find_first_by(
        automation,
        window,
        TreeScope_Descendants,
        UIA_AutomationIdPropertyId,
        "head_image_v_view.head_view_",
    )?;

    let name = avatar.CurrentName()?.to_string();
    if name.is_empty() {
        return Err(Error::from(E_FAIL));
    }
    Ok(name)
}

unsafe fn get_username_from_tab(
    automation: &IUIAutomation,
    window: &IUIAutomationElement,
    root: &IUIAutomationElement,
) -> Result<String> {
    let name_cond = automation.CreatePropertyCondition(UIA_NamePropertyId, &VARIANT::from("微信"))?;
    let class_cond = automation.CreatePropertyCondition(UIA_ClassNamePropertyId, &VARIANT::from("mmui::XTabBarItem"))?;
    let and_cond = automation.CreateAndCondition(&name_cond, &class_cond)?;

    let wechat_tab = window.FindFirst(TreeScope_Descendants, &and_cond)?;
    let rect = wechat_tab.CurrentBoundingRectangle()?;
    let center_x = rect.left + (rect.right - rect.left) / 2;
    let center_y = rect.top + (rect.bottom - rect.top) / 2 - (rect.bottom - rect.top);

    click_point(center_x, center_y);
    std::thread::sleep(DEFAULT_DELAY);

    let username = get_nickname_from_profile(automation, root);

    click_point(center_x, center_y);
    username
}

unsafe fn get_nickname_from_profile(automation: &IUIAutomation, root: &IUIAutomationElement) -> Result<String> {
    let profile_window = find_first_by(
        automation,
        root,
        TreeScope_Children,
        UIA_ClassNamePropertyId,
        "mmui::ProfileUniquePop",
    )?;

    if profile_window.CurrentClassName()?.is_empty() {
        return Err(Error::from(E_FAIL));
    }

    let button_cond = automation.CreatePropertyCondition(
        UIA_ControlTypePropertyId,
        &VARIANT::from(UIA_ButtonControlTypeId.0 as i32),
    )?;
    let buttons = profile_window.FindAll(TreeScope_Descendants, &button_cond)?;

    for i in 0..buttons.Length()? {
        let button = buttons.GetElement(i)?;
        let name = button.CurrentName()?.to_string();
        if !name.is_empty() {
            return Ok(name);
        }
    }

    Err(Error::from(E_FAIL))
}

// ============ 发送消息与文件 ============

unsafe fn send_wechat_message(recipient: &str, message: &str) -> Result<()> {
    let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

    let automation: IUIAutomation = CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER)?;
    let root = automation.GetRootElement()?;

    let window = locate_wechat_window(&automation, &root)?;
    activate_wechat_window(&window)?;

    locate_contact_and_enter_chat(&automation, &window, recipient)?;

    set_chat_input_value(&automation, &window, message)?;
    click_send_button(&automation, &window)
}

unsafe fn send_wechat_file(recipient: &str, filepath: &str) -> Result<()> {
    let path = Path::new(filepath);
    if !path.exists() {
        return Err(Error::from(E_FAIL));
    }

    let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

    let automation: IUIAutomation = CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER)?;
    let root = automation.GetRootElement()?;

    let window = locate_wechat_window(&automation, &root)?;
    activate_wechat_window(&window)?;

    locate_contact_and_enter_chat(&automation, &window, recipient)?;

    set_clipboard_file(&path)?;
    paste_to_chat_input(&automation, &window)?;
    let result = click_send_button(&automation, &window);

    // 发送完成后立即清空剪贴板，避免污染用户使用
    clear_clipboard();

    result
}

// ============ 三级优先级定位联系人 ============

unsafe fn locate_contact_and_enter_chat(
    automation: &IUIAutomation,
    window: &IUIAutomationElement,
    recipient: &str,
) -> Result<()> {
    // 优先级 1：当前聊天窗口已是目标联系人
    if let Ok(current_title) = get_current_chat_title(automation, window) {
        if current_title == recipient {
            return Ok(());
        }
    }

    // 优先级 2：在会话列表中查找
    if let Ok(true) = find_and_click_in_session_list(automation, window, recipient) {
        std::thread::sleep(DEFAULT_DELAY);
        return Ok(());
    }

    // 优先级 3：搜索联系人
    search_and_click_contact(automation, window, recipient)?;
    std::thread::sleep(DEFAULT_DELAY);
    Ok(())
}

unsafe fn get_current_chat_title(automation: &IUIAutomation, window: &IUIAutomationElement) -> Result<String> {
    let title_control = find_first_by(
        automation,
        window,
        TreeScope_Descendants,
        UIA_AutomationIdPropertyId,
        "content_view.top_content_view.title_h_view.left_v_view.left_content_v_view.left_ui_.big_title_line_h_view",
    )?;

    let name = title_control.CurrentName()?.to_string();
    Ok(clean_trailing_count(&name))
}

// 去除末尾的 (数字) 后缀，如 "群名(5)" -> "群名"
fn clean_trailing_count(s: &str) -> String {
    let s = s.trim();
    if let Some(open_idx) = s.rfind('(') {
        let inside = &s[open_idx + 1..s.len() - 1];
        if s.ends_with(')') && !inside.is_empty() && inside.chars().all(|c| c.is_ascii_digit()) {
            return s[..open_idx].trim().to_string();
        }
    }
    s.to_string()
}

unsafe fn find_and_click_in_session_list(
    automation: &IUIAutomation,
    window: &IUIAutomationElement,
    recipient: &str,
) -> Result<bool> {
    let session_list = match find_first_by(
        automation,
        window,
        TreeScope_Descendants,
        UIA_AutomationIdPropertyId,
        "session_list",
    ) {
        Ok(list) => list,
        Err(_) => return Ok(false),
    };

    let true_condition = automation.CreateTrueCondition()?;
    let items = match session_list.FindAll(TreeScope_Children, &true_condition) {
        Ok(items) => items,
        Err(_) => return Ok(false),
    };

    let count = items.Length()?;
    for i in 0..count {
        if let Ok(item) = items.GetElement(i) {
            if let Ok(name) = item.CurrentName() {
                let name_str = name.to_string();
                let first_line = name_str.split('\n').next().unwrap_or("").trim();
                if first_line == recipient {
                    click_element_center(&item)?;
                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}

unsafe fn search_and_click_contact(
    automation: &IUIAutomation,
    window: &IUIAutomationElement,
    recipient: &str,
) -> Result<()> {
    let search_edit = find_first_by(
        automation,
        window,
        TreeScope_Descendants,
        UIA_NamePropertyId,
        "搜索",
    )?;

    click_element_center(&search_edit)?;
    std::thread::sleep(SHORT_DELAY);

    let value_pattern = search_edit.GetCurrentPatternAs::<IUIAutomationValuePattern>(UIA_ValuePatternId)?;
    value_pattern.SetValue(&recipient.into())?;
    std::thread::sleep(DEFAULT_DELAY);

    let search_popover = wait_for_element(
        automation,
        window,
        TreeScope_Children,
        UIA_ClassNamePropertyId,
        "mmui::SearchContentPopover",
        20,
    )?;

    let table_view = find_first_by(
        automation,
        &search_popover,
        TreeScope_Descendants,
        UIA_ClassNamePropertyId,
        "mmui::XTableView",
    )?;

    let true_condition = automation.CreateTrueCondition()?;
    let cells = table_view.FindAll(TreeScope_Children, &true_condition)?;

    let valid_groups = ["联系人", "群聊", "功能", "最常使用", "聊天记录"];
    let skip_groups = ["搜索网络结果"];
    let mut current_group = String::new();

    for i in 0..cells.Length()? {
        let cell = cells.GetElement(i)?;
        let name = cell.CurrentName()?.to_string();

        if name.trim().is_empty() {
            continue;
        }

        if valid_groups.contains(&name.as_str()) || skip_groups.contains(&name.as_str()) {
            current_group = name;
            continue;
        }

        if skip_groups.contains(&current_group.as_str()) {
            continue;
        }

        if valid_groups.contains(&current_group.as_str()) && name == recipient {
            click_element_center(&cell)?;
            std::thread::sleep(std::time::Duration::from_millis(300));
            return Ok(());
        }
    }

    press_key(VK_ESCAPE);
    Err(Error::from(E_FAIL))
}

// ============ 发送内容 ============

// 通过 ValuePattern 直接设置控件内容（不污染剪贴板）
unsafe fn set_chat_input_value(
    automation: &IUIAutomation,
    window: &IUIAutomationElement,
    message: &str,
) -> Result<()> {
    let chat_input = find_first_by(
        automation,
        window,
        TreeScope_Descendants,
        UIA_AutomationIdPropertyId,
        "chat_input_field",
    )?;

    let value_pattern = chat_input.GetCurrentPatternAs::<IUIAutomationValuePattern>(UIA_ValuePatternId)?;
    value_pattern.SetValue(&message.into())?;
    std::thread::sleep(std::time::Duration::from_millis(200));
    Ok(())
}

unsafe fn click_send_button(automation: &IUIAutomation, window: &IUIAutomationElement) -> Result<()> {
    let name_cond = automation.CreatePropertyCondition(UIA_NamePropertyId, &VARIANT::from("发送"))?;
    let class_cond = automation.CreatePropertyCondition(UIA_ClassNamePropertyId, &VARIANT::from("mmui::XOutlineButton"))?;
    let and_cond = automation.CreateAndCondition(&name_cond, &class_cond)?;

    if let Ok(send_button) = window.FindFirst(TreeScope_Descendants, &and_cond) {
        if let Ok(enabled) = send_button.CurrentIsEnabled() {
            if enabled.as_bool() {
                click_element_center(&send_button)?;
                std::thread::sleep(std::time::Duration::from_millis(200));
                return Ok(());
            }
        }
    }

    press_key(VK_RETURN);
    std::thread::sleep(std::time::Duration::from_millis(200));
    Ok(())
}

// ============ 文件发送 ============

unsafe fn set_clipboard_file(path: &Path) -> Result<()> {
    let absolute_path = path.canonicalize()?;
    let path_str = absolute_path.to_string_lossy().replace("\\", "\\\\");

    let files = format!("{}\0\0", path_str);
    let files_utf16: Vec<u16> = files.encode_utf16().collect();

    // DROPFILES 结构体：20 字节，fWide = 1（UTF-16）
    let mut dropfiles_data: Vec<u8> = vec![0u8; 20];
    dropfiles_data[0] = 20;
    dropfiles_data[16] = 1;

    for byte in files_utf16.iter().flat_map(|c| c.to_le_bytes()) {
        dropfiles_data.push(byte);
    }

    if OpenClipboard(None).is_ok() {
        let _ = EmptyClipboard();

        let hmem = GlobalAlloc(GLOBAL_ALLOC_FLAGS(0x0002), dropfiles_data.len())?;
        if !hmem.is_invalid() {
            let ptr = GlobalLock(hmem);
            if !ptr.is_null() {
                std::ptr::copy_nonoverlapping(dropfiles_data.as_ptr(), ptr as *mut u8, dropfiles_data.len());
                let _ = GlobalUnlock(hmem);
                let _ = SetClipboardData(0x000F, HANDLE(hmem.0));
            }
        }
        let _ = CloseClipboard();
    }

    std::thread::sleep(SHORT_DELAY);
    Ok(())
}

// 清空剪贴板，避免污染用户使用
unsafe fn clear_clipboard() {
    if OpenClipboard(None).is_ok() {
        let _ = EmptyClipboard();
        let _ = CloseClipboard();
    }
}

unsafe fn paste_to_chat_input(automation: &IUIAutomation, window: &IUIAutomationElement) -> Result<()> {
    let chat_input = find_first_by(
        automation,
        window,
        TreeScope_Descendants,
        UIA_AutomationIdPropertyId,
        "chat_input_field",
    )?;

    click_element_center(&chat_input)?;
    std::thread::sleep(SHORT_DELAY);

    press_key_combo(VK_CONTROL, VIRTUAL_KEY(0x56)); // Ctrl+V
    std::thread::sleep(DEFAULT_DELAY);
    Ok(())
}

// ============ 通用辅助函数 ============

unsafe fn locate_wechat_window(automation: &IUIAutomation, root: &IUIAutomationElement) -> Result<IUIAutomationElement> {
    let window = find_first_by(
        automation,
        root,
        TreeScope_Children,
        UIA_ClassNamePropertyId,
        "mmui::MainWindow",
    )?;

    if window.CurrentClassName()?.is_empty() {
        return Err(Error::from(E_FAIL));
    }

    Ok(window)
}

// 激活微信窗口并居中显示，确保发送按钮等控件在可见区域内
unsafe fn activate_wechat_window(window: &IUIAutomationElement) -> Result<()> {
    let hwnd = HWND(window.CurrentNativeWindowHandle()?.0);

    // 还原最小化状态
    if IsIconic(hwnd).as_bool() {
        let _ = ShowWindow(hwnd, SW_RESTORE);
    }

    // 把窗口移到屏幕中央（保留原大小），确保所有控件可见
    center_window_on_screen(hwnd);

    // 置顶并激活
    if !SetForegroundWindow(hwnd).as_bool() {
        let _ = SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW);
        std::thread::sleep(SHORT_DELAY);
        let _ = SetWindowPos(hwnd, HWND_NOTOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW);
    }
    let _ = BringWindowToTop(hwnd);
    std::thread::sleep(DEFAULT_DELAY);

    Ok(())
}

// 把窗口移到屏幕中央（保留原大小）
unsafe fn center_window_on_screen(hwnd: HWND) {
    let mut rect = RECT::default();
    if GetWindowRect(hwnd, &mut rect).is_err() {
        return;
    }

    let win_w = rect.right - rect.left;
    let win_h = rect.bottom - rect.top;

    // 获取主工作区（排除任务栏）
    let mut mi = MONITORINFO {
        cbSize: std::mem::size_of::<MONITORINFO>() as u32,
        ..Default::default()
    };
    let monitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
    if GetMonitorInfoW(monitor, &mut mi).as_bool() {
        let work_w = mi.rcWork.right - mi.rcWork.left;
        let work_h = mi.rcWork.bottom - mi.rcWork.top;
        let new_x = mi.rcWork.left + (work_w - win_w) / 2;
        let new_y = mi.rcWork.top + (work_h - win_h) / 2;
        let _ = SetWindowPos(hwnd, HWND_TOP, new_x, new_y, win_w, win_h, SWP_SHOWWINDOW);
    }
}

// 模拟鼠标点击元素中心
unsafe fn click_element_center(element: &IUIAutomationElement) -> Result<()> {
    let rect = element.CurrentBoundingRectangle()?;
    let center_x = rect.left + (rect.right - rect.left) / 2;
    let center_y = rect.top + (rect.bottom - rect.top) / 2;
    click_point(center_x, center_y);
    Ok(())
}

// 模拟鼠标点击指定坐标
unsafe fn click_point(x: i32, y: i32) {
    let _ = SetCursorPos(x, y);
    std::thread::sleep(std::time::Duration::from_millis(50));
    mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
    std::thread::sleep(std::time::Duration::from_millis(50));
    mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
    std::thread::sleep(SHORT_DELAY);
}

unsafe fn find_first_by(
    automation: &IUIAutomation,
    parent: &IUIAutomationElement,
    scope: TreeScope,
    property_id: UIA_PROPERTY_ID,
    value: &str,
) -> Result<IUIAutomationElement> {
    let condition = automation.CreatePropertyCondition(property_id, &VARIANT::from(value))?;
    parent.FindFirst(scope, &condition)
}

unsafe fn wait_for_element(
    automation: &IUIAutomation,
    parent: &IUIAutomationElement,
    scope: TreeScope,
    property_id: UIA_PROPERTY_ID,
    value: &str,
    max_attempts: u32,
) -> Result<IUIAutomationElement> {
    for _ in 0..max_attempts {
        std::thread::sleep(SHORT_DELAY);
        if let Ok(element) = find_first_by(automation, parent, scope, property_id, value) {
            return Ok(element);
        }
    }
    Err(Error::from(E_FAIL))
}

unsafe fn press_key(vk: VIRTUAL_KEY) {
    let mut inputs = [INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vk,
                wScan: 0,
                dwFlags: KEYBD_EVENT_FLAGS(0),
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }; 2];

    inputs[1].Anonymous.ki.dwFlags = KEYEVENTF_KEYUP;
    SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    std::thread::sleep(SHORT_DELAY);
}

unsafe fn press_key_combo(modifier: VIRTUAL_KEY, key: VIRTUAL_KEY) {
    let down = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: modifier,
                wScan: 0,
                dwFlags: KEYBD_EVENT_FLAGS(0),
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    let key_down = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: key,
                wScan: 0,
                dwFlags: KEYBD_EVENT_FLAGS(0),
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    let key_up = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: key,
                wScan: 0,
                dwFlags: KEYEVENTF_KEYUP,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    let modifier_up = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: modifier,
                wScan: 0,
                dwFlags: KEYEVENTF_KEYUP,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    let inputs = [down, key_down, key_up, modifier_up];
    SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    std::thread::sleep(SHORT_DELAY);
}
