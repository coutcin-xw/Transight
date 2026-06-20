//! 文本选择捕获模块
//!
//! 各平台独立策略:
//!   Windows: UI Automation → 剪贴板 (GetClipboardSequenceNumber 检测)
//!   macOS:   Accessibility API → AppleScript (NSPasteboard.changeCount 检测)
//!   Linux:   X11 PRIMARY / Wayland primary selection

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

/// 获取当前选中的文本
pub fn get_selected_text() -> Result<String, String> {
    let text = platform_get_text();
    if text.is_empty() {
        Err("no text selected".into())
    } else {
        Ok(text)
    }
}

#[cfg(target_os = "linux")]
fn platform_get_text() -> String {
    linux::get_text()
}

#[cfg(target_os = "macos")]
fn platform_get_text() -> String {
    macos::get_text()
}

#[cfg(target_os = "windows")]
fn platform_get_text() -> String {
    windows::get_text()
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn platform_get_text() -> String {
    String::new()
}
