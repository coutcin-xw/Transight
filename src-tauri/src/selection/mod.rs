//! 文本选择捕获模块
//! 策略: 平台原生 API → 模拟 Ctrl+C → 读取剪贴板 → 恢复剪贴板

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

/// 获取当前选中的文本 (平台最佳路径 + 剪贴板回退)
pub fn get_selected_text() -> Result<String, String> {
    platform_get_text().or_else(|_| clipboard_fallback())
}

#[cfg(target_os = "linux")]
fn platform_get_text() -> Result<String, String> {
    linux::get_text()
}

#[cfg(target_os = "macos")]
fn platform_get_text() -> Result<String, String> {
    macos::get_text()
}

#[cfg(target_os = "windows")]
fn platform_get_text() -> Result<String, String> {
    windows::get_text()
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn platform_get_text() -> Result<String, String> {
    Err("unsupported platform".into())
}

// ─── 剪贴板回退 (跨平台) ────────────────────────────────────────────────────

fn clipboard_fallback() -> Result<String, String> {
    let saved = read_clipboard().unwrap_or_default();
    simulate_copy()?;
    std::thread::sleep(std::time::Duration::from_millis(80));
    let selected = read_clipboard()?;
    if !saved.is_empty() {
        let _ = write_clipboard(&saved);
    }
    if selected.is_empty() {
        Err("no text selected".into())
    } else {
        Ok(selected)
    }
}

fn read_clipboard() -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("pbpaste")
            .output()
            .map_err(|e| format!("pbpaste: {e}"))?;
        return Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
    }
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    {
        let mut clipboard =
            arboard::Clipboard::new().map_err(|e| format!("arboard: {e}"))?;
        clipboard.get_text().map_err(|e| format!("get_text: {e}"))
    }
}

fn write_clipboard(text: &str) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        use std::io::Write;
        let mut child = std::process::Command::new("pbcopy")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("pbcopy spawn: {e}"))?;
        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(text.as_bytes())
                .map_err(|e| format!("pbcopy write: {e}"))?;
        }
        child.wait().map_err(|e| format!("pbcopy wait: {e}"))?;
        return Ok(());
    }
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    {
        let mut clipboard =
            arboard::Clipboard::new().map_err(|e| format!("arboard: {e}"))?;
        clipboard
            .set_text(text)
            .map_err(|e| format!("set_text: {e}"))
    }
}

fn simulate_copy() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        let script =
            r#"tell application "System Events" to keystroke "c" using {command down}"#;
        std::process::Command::new("osascript")
            .arg("-e")
            .arg(script)
            .output()
            .map_err(|e| format!("osascript: {e}"))?;
        return Ok(());
    }
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    {
        use enigo::{Direction, Enigo, Key, Keyboard, Settings};
        let mut enigo =
            Enigo::new(&Settings::default()).map_err(|e| format!("enigo: {e}"))?;
        for key in [Key::Control, Key::Alt, Key::Shift, Key::Meta] {
            let _ = enigo.key(key, Direction::Release);
        }
        enigo
            .key(Key::Control, Direction::Press)
            .map_err(|e| format!("press: {e}"))?;
        enigo
            .key(Key::Unicode('c'), Direction::Click)
            .map_err(|e| format!("click c: {e}"))?;
        enigo
            .key(Key::Control, Direction::Release)
            .map_err(|e| format!("release: {e}"))?;
        Ok(())
    }
}
