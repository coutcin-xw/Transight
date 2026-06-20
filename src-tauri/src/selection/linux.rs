//! Linux 文本选择: X11 primary selection / Wayland primary selection
//!
//! 直接读取 X11 PRIMARY 或 Wayland primary selection，无需模拟 Ctrl+C。
//! Wayland 下若 primary selection 不支持则回退到 X11 模式。

use std::io::Read;
use std::time::Duration;

pub fn get_text() -> String {
    match std::env::var("XDG_SESSION_TYPE").as_deref() {
        Ok("x11") => match x11_primary() {
            Ok(text) => return text,
            Err(e) => eprintln!("[transight] x11 primary error: {e}"),
        },
        Ok("wayland") => match wayland_primary() {
            Ok(text) => return text,
            Err(e) => eprintln!("[transight] wayland primary error: {e}"),
        },
        Ok(other) => eprintln!("[transight] unknown Linux session type: {other}"),
        Err(e) => eprintln!("[transight] XDG_SESSION_TYPE not set: {e}"),
    }
    String::new()
}

fn x11_primary() -> Result<String, String> {
    let clipboard =
        x11_clipboard::Clipboard::new().map_err(|e| format!("x11 clipboard: {e}"))?;
    let primary = clipboard
        .load(
            clipboard.getter.atoms.primary,
            clipboard.getter.atoms.utf8_string,
            clipboard.getter.atoms.property,
            Duration::from_millis(100),
        )
        .map_err(|e| format!("x11 load: {e}"))?;
    Ok(String::from_utf8_lossy(&primary)
        .trim_matches('\0')
        .trim()
        .to_string())
}

fn wayland_primary() -> Result<String, String> {
    use wl_clipboard_rs::paste::{get_contents, ClipboardType, MimeType, Seat};
    use wl_clipboard_rs::utils::is_primary_selection_supported;

    // 若 Wayland compositor 不支持 primary selection，设环境变量回退到 X11
    match is_primary_selection_supported() {
        Ok(true) => {}
        _ => {
            eprintln!("[transight] Wayland primary selection not supported, fallback to X11");
            unsafe {
                std::env::set_var("XDG_SESSION_TYPE", "x11");
                std::env::set_var("GDK_BACKEND", "x11");
            }
            return x11_primary();
        }
    }

    let (mut pipe, _) =
        get_contents(ClipboardType::Primary, Seat::Unspecified, MimeType::Text)
            .map_err(|e| format!("wayland primary: {e}"))?;
    let mut contents = vec![];
    pipe.read_to_end(&mut contents)
        .map_err(|e| format!("wayland read: {e}"))?;
    Ok(String::from_utf8_lossy(&contents)
        .trim_matches('\0')
        .trim()
        .to_string())
}
