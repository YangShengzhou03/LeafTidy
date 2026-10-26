#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // 启动时安装更新：存在待装安装包（版本未对齐）则延迟启动安装器并退出主进程
    if leaf_tidy::commands::updater::handle_pending_install() {
        return;
    }
    leaf_tidy::run();
}
