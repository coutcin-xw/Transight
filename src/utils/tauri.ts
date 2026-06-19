import { invoke } from "@tauri-apps/api/core";

/** 显示翻译弹窗 */
export async function showTranslateWindow(): Promise<void> {
  return invoke("show_translate_window");
}

/** 隐藏翻译弹窗 */
export async function hideTranslateWindow(): Promise<void> {
  return invoke("hide_translate_window");
}

/** 打开设置窗口 */
export async function openSettingsWindow(): Promise<void> {
  return invoke("open_settings_window");
}

/** 关闭设置窗口 */
export async function closeSettingsWindow(): Promise<void> {
  return invoke("close_settings_window");
}

/** 执行翻译 */
export async function translate(
  text: string,
  sourceLang: string,
  targetLang: string,
): Promise<string> {
  return invoke("translate", { text, sourceLang, targetLang });
}

/** 检测语言 */
export async function detectLanguage(text: string): Promise<string> {
  return invoke("detect_language", { text });
}

/** 获取选中文本 */
export async function getSelectedText(): Promise<string> {
  return invoke("get_selected_text");
}
