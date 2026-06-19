//! Windows 文本选择: UI Automation (IUIAutomation TextPattern)

use windows::Win32::System::Com::{CoCreateInstance, CoInitialize, CLSCTX_ALL};
use windows::Win32::UI::Accessibility::{
    CUIAutomation, IUIAutomation, IUIAutomationTextPattern, UIA_TextPatternId,
};

pub fn get_text() -> Result<String, String> {
    let _ = unsafe { CoInitialize(None) };
    let auto: IUIAutomation =
        unsafe { CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL) }
            .map_err(|e| format!("COM: {e}"))?;
    let el = unsafe { auto.GetFocusedElement() }
        .map_err(|e| format!("GetFocusedElement: {e}"))?;
    let pattern: IUIAutomationTextPattern =
        unsafe { el.GetCurrentPatternAs(UIA_TextPatternId) }
            .map_err(|e| format!("TextPattern: {e}"))?;
    let text_array = unsafe { pattern.GetSelection() }
        .map_err(|e| format!("GetSelection: {e}"))?;
    let length = unsafe { text_array.Length() }
        .map_err(|e| format!("Length: {e}"))?;

    let mut result = String::new();
    for i in 0..length {
        let text = unsafe { text_array.GetElement(i) }
            .map_err(|e| format!("GetElement: {e}"))?;
        let s = unsafe { text.GetText(-1) }
            .map_err(|e| format!("GetText: {e}"))?;
        result.push_str(&s.to_string());
    }
    Ok(result.trim().to_string())
}
