use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{Emitter, LogicalSize, Manager, State};

/// 翻译窗口默认尺寸
const TRANSLATE_WIDTH: f64 = 350.0;
const TRANSLATE_HEIGHT: f64 = 540.0;

/// 显示翻译弹窗（每次打开时重置为默认大小）
#[tauri::command]
pub async fn show_translate_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("translate") {
        // 重置为默认大小，关闭/隐藏期间的手动调整不保留
        let _ = window.set_size(LogicalSize::new(TRANSLATE_WIDTH, TRANSLATE_HEIGHT));
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 隐藏翻译弹窗
#[tauri::command]
pub async fn hide_translate_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("translate") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 打开设置窗口
#[tauri::command]
pub async fn open_settings_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("settings") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 关闭设置窗口
#[tauri::command]
pub async fn close_settings_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("settings") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 设置固定状态
#[tauri::command]
pub async fn set_pin_window(
    app: tauri::AppHandle,
    state: State<'_, Arc<AtomicBool>>,
    pinned: bool,
) -> Result<(), String> {
    state.store(pinned, Ordering::Relaxed);
    let _ = app.emit("pin-changed", pinned);
    Ok(())
}

/// 广播主题变更到所有窗口
#[tauri::command]
pub async fn broadcast_theme(app: tauri::AppHandle, theme: String) -> Result<(), String> {
    let _ = app.emit("theme-changed", theme);
    Ok(())
}
