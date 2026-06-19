use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{Emitter, Manager, State};

/// 显示翻译弹窗
#[tauri::command]
pub async fn show_translate_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("translate") {
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
    // 启用 pin 时，窗口不自动隐藏（由失焦事件处理器检查此状态）
    state.store(pinned, Ordering::Relaxed);
    // 通知所有窗口 pin 状态变化
    let _ = app.emit("pin-changed", pinned);
    Ok(())
}
