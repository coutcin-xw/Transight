//! Windows 文本选择捕获
//!
//! 策略:
//!   1. UI Automation TextPattern (原生控件)
//!   2. 剪贴板回退: 模拟 Ctrl+C → GetClipboardSequenceNumber 检测变化 →
//!      读取剪贴板 → 恢复原内容

use arboard::Clipboard;
use std::error::Error;
use windows::Win32::System::Com::{CoCreateInstance, CoInitialize, CLSCTX_ALL};
use windows::Win32::System::DataExchange::GetClipboardSequenceNumber;
use windows::Win32::UI::Accessibility::{
    CUIAutomation, IUIAutomation, IUIAutomationTextPattern, UIA_TextPatternId,
};

pub fn get_text() -> String {
    match get_text_by_automation() {
        Ok(text) if !text.is_empty() => return text,
        Ok(_) => eprintln!("[transight] UI Automation returned empty selection"),
        Err(e) => eprintln!("[transight] UI Automation failed: {e}"),
    }

    eprintln!("[transight] falling back to clipboard copy");
    match get_text_by_clipboard() {
        Ok(text) if !text.is_empty() => text,
        Ok(_) => {
            eprintln!("[transight] clipboard fallback returned empty");
            String::new()
        }
        Err(e) => {
            eprintln!("[transight] clipboard fallback error: {e}");
            String::new()
        }
    }
}

fn get_text_by_automation() -> Result<String, Box<dyn Error>> {
    let _ = unsafe { CoInitialize(None) };
    let auto: IUIAutomation = unsafe { CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL) }?;
    let el = unsafe { auto.GetFocusedElement() }?;
    let pattern: IUIAutomationTextPattern =
        unsafe { el.GetCurrentPatternAs(UIA_TextPatternId) }?;
    let text_array = unsafe { pattern.GetSelection() }?;
    let length = unsafe { text_array.Length() }?;

    let mut result = String::new();
    for i in 0..length {
        let text = unsafe { text_array.GetElement(i) }?;
        let s = unsafe { text.GetText(-1) }?;
        result.push_str(&s.to_string());
    }
    Ok(result.trim().to_string())
}

fn get_text_by_clipboard() -> Result<String, Box<dyn Error>> {
    // 同时保存文本和图片剪贴板内容
    let old_clipboard = (
        Clipboard::new()?.get_text(),
        Clipboard::new()?.get_image(),
    );

    if !do_copy() {
        return Err("copy simulation failed — clipboard unchanged".into());
    }

    // 读取新剪贴板文本
    let new_text = Clipboard::new()?.get_text();

    // 恢复原剪贴板内容
    let mut write_clipboard = Clipboard::new()?;
    match old_clipboard {
        (Ok(ref text), _) if !text.is_empty() => {
            write_clipboard.set_text(text)?;
        }
        (_, Ok(image)) => {
            // 图片剪贴板 — arboard 的 set_image 需要 ImageData，从 get_image
            // 获取的数据无法直接回写，故清除
            write_clipboard.clear()?;
            // 注意: 无法完美恢复图片，但至少清除不留脏数据
            eprintln!("[transight] old clipboard was image, cleared after copy");
        }
        _ => {
            write_clipboard.clear()?;
        }
    }

    match new_text {
        Ok(text) => Ok(text.trim().to_string()),
        Err(_) => Err("new clipboard is not text".into()),
    }
}

/// 模拟 Ctrl+C 并使用 GetClipboardSequenceNumber 检测剪贴板是否真的发生了变化
fn do_copy() -> bool {
    use enigo::{
        Direction::{Click, Press, Release},
        Enigo, Key, Keyboard, Settings,
    };

    let seq_before = unsafe { GetClipboardSequenceNumber() };

    let mut enigo = match Enigo::new(&Settings::default()) {
        Ok(e) => e,
        Err(_) => return false,
    };

    // 释放所有可能干扰的按键（包括全局快捷键残留和意外按下键）
    for key in [
        Key::Control,
        Key::Alt,
        Key::Shift,
        Key::Meta,
        Key::Space,
        Key::Tab,
        Key::Escape,
        Key::CapsLock,
        Key::Unicode('c'),
    ] {
        let _ = enigo.key(key, Release);
    }

    std::thread::sleep(std::time::Duration::from_millis(30));

    let _ = enigo.key(Key::Control, Press);
    let _ = enigo.key(Key::Unicode('c'), Click);
    let _ = enigo.key(Key::Control, Release);

    std::thread::sleep(std::time::Duration::from_millis(100));

    let seq_after = unsafe { GetClipboardSequenceNumber() };
    seq_after != seq_before
}
