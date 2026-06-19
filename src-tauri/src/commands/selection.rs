/// 获取当前选中的文本
/// M1.3 将集成 selection crate + 剪贴板回退
#[tauri::command]
pub async fn get_selected_text() -> Result<String, String> {
    // Stub: M1.3 实现
    crate::selection::get_selected_text()
}
