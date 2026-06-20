//! macOS 文本选择捕获
//!
//! 策略:
//!   1. Accessibility API (AXUIElement) — 直接读取选中文本
//!   2. 剪贴板回退: AppleScript 模拟 Cmd+C →
//!      NSPasteboard.changeCount 检测变化 → 恢复原剪贴板

use accessibility_ng::{AXAttribute, AXUIElement};
use accessibility_sys_ng::{kAXFocusedUIElementAttribute, kAXSelectedTextAttribute};
use core_foundation::string::CFString;

pub fn get_text() -> String {
    match get_text_by_accessibility() {
        Ok(text) if !text.is_empty() => return text,
        Ok(_) => eprintln!("[transight] Accessibility API returned empty selection"),
        Err(e) => eprintln!("[transight] Accessibility API error: {e}"),
    }

    eprintln!("[transight] falling back to clipboard copy (AppleScript)");
    match clipboard_fallback_macos() {
        Ok(text) if !text.is_empty() => text,
        Ok(_) => {
            eprintln!("[transight] AppleScript fallback returned empty");
            String::new()
        }
        Err(e) => {
            eprintln!("[transight] AppleScript fallback error: {e}");
            String::new()
        }
    }
}

fn get_text_by_accessibility() -> Result<String, String> {
    let system = AXUIElement::system_wide();
    let Some(el) = system
        .attribute(&AXAttribute::new(&CFString::from_static_string(
            kAXFocusedUIElementAttribute,
        )))
        .ok()
        .and_then(|e| e.downcast_into::<AXUIElement>())
    else {
        return Err("no focused element".into());
    };

    let Some(text) = el
        .attribute(&AXAttribute::new(&CFString::from_static_string(
            kAXSelectedTextAttribute,
        )))
        .ok()
        .and_then(|t| t.downcast_into::<CFString>())
    else {
        return Err("no selected text".into());
    };

    Ok(text.to_string())
}

/// 使用 AppleScript 模拟 Cmd+C 并检测 NSPasteboard.changeCount
fn clipboard_fallback_macos() -> Result<String, String> {
    let output = std::process::Command::new("osascript")
        .arg("-e")
        .arg(APPLE_SCRIPT)
        .output()
        .map_err(|e| format!("osascript spawn: {e}"))?;

    if output.status.success() {
        let content = String::from_utf8(output.stdout)
            .map_err(|e| format!("utf8: {e}"))?;
        Ok(content.trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("osascript failed: {stderr}"))
    }
}

const APPLE_SCRIPT: &str = r#"
use AppleScript version "2.4"
use scripting additions
use framework "Foundation"
use framework "AppKit"

set savedAlertVolume to alert volume of (get volume settings)

-- 备份剪贴板
set savedClipboard to the clipboard

set thePasteboard to current application's NSPasteboard's generalPasteboard()
set theCount to thePasteboard's changeCount()

tell application "System Events"
    set volume alert volume 0
end tell

-- 模拟 Cmd+C 复制选中文本
tell application "System Events" to keystroke "c" using {command down}
delay 0.1

tell application "System Events"
    set volume alert volume savedAlertVolume
end tell

-- 检测剪贴板是否真的发生了变化
if thePasteboard's changeCount() is theCount then
    return ""
end if

set theSelectedText to the clipboard

-- 恢复原剪贴板
set the clipboard to savedClipboard

theSelectedText
"#;
