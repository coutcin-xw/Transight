//! Linux 文本选择: X11 primary selection / Wayland primary selection

use std::io::Read;
use std::time::Duration;

pub fn get_text() -> Result<String, String> {
    match std::env::var("XDG_SESSION_TYPE").as_deref() {
        Ok("x11") => x11_primary(),
        Ok("wayland") => wayland_primary(),
        _ => Err("unknown Linux session type".into()),
    }
}

fn x11_primary() -> Result<String, String> {
    let clipboard = x11_clipboard::Clipboard::new()
        .map_err(|e| format!("x11 clipboard: {e}"))?;
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

    let (mut pipe, _) = get_contents(ClipboardType::Primary, Seat::Unspecified, MimeType::Text)
        .map_err(|e| format!("wayland primary: {e}"))?;
    let mut contents = vec![];
    pipe.read_to_end(&mut contents)
        .map_err(|e| format!("wayland read: {e}"))?;
    Ok(String::from_utf8_lossy(&contents)
        .trim_matches('\0')
        .trim()
        .to_string())
}
