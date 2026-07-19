unsafe fn get_wechat_id(automation: &IUIAutomation, window: &IUIAutomationElement) -> Result<String> {
    log_debug!("WeChat", "尝试获取微信号");

    // 方法1: 通过AutomationId查找 ProfileTextView
    if let Ok(profile_text) = find_first_by(
        automation,
        window,
        TreeScope_Descendants,
        UIA_AutomationIdPropertyId,
        "right_v_view.user_info_center_view.basic_line_view.ProfileTextView",
    ) {
        if let Ok(name) = profile_text.CurrentName() {
            let wechat_id = name.to_string();
            if !wechat_id.is_empty() {
                log_info!("WeChat", "从ProfileTextView获取微信号: {}", wechat_id);
                return Ok(wechat_id);
            }
        }
    }

    // 方法2: 查找微信号旁边的文本（"微信号："标签）
    if let Ok(wechat_label) = find_first_by(
        automation,
        window,
        TreeScope_Descendants,
        UIA_NamePropertyId,
        "微信号：",
    ) {
        // 微信号在"微信号："标签的下一个兄弟元素
        let true_condition = automation.CreateTrueCondition()?;
        if let Ok(parent) = wechat_label.GetCurrentPatternAs::<IUIAutomationLegacyIAccessiblePattern>(UIA_LegacyIAccessiblePatternId) {
            // 尝试查找相邻的文本元素
            if let Ok(siblings) = window.FindAll(TreeScope_Descendants, &true_condition) {
                let mut found_label = false;
                for i in 0..siblings.Length()? {
                    if let Ok(element) = siblings.GetElement(i) {
                        if let Ok(name) = element.CurrentName() {
                            if name.to_string() == "微信号：" {
                                found_label = true;
                                continue;
                            }
                            if found_label {
                                let wechat_id = name.to_string();
                                log_info!("WeChat", "从相邻元素获取微信号: {}", wechat_id);
                                return Ok(wechat_id);
                            }
                        }
                    }
                }
            }
        }
    }

    log_warn!("WeChat", "无法获取微信号");
    Err(Error::from(E_FAIL))
}