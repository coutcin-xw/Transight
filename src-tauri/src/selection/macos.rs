//! macOS 文本选择: Accessibility API (AXUIElement)

use accessibility_ng::{AXAttribute, AXUIElement};
use accessibility_sys_ng::{kAXFocusedUIElementAttribute, kAXSelectedTextAttribute};
use core_foundation::string::CFString;

pub fn get_text() -> Result<String, String> {
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
