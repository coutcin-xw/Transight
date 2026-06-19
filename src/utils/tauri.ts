import { invoke } from "@tauri-apps/api/core";
import type { TranslationResult, PluginDefinition, ServiceConfig } from "../types";

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

/** 执行翻译 — 返回多个翻译源的结果 */
export async function translate(
  text: string,
  sourceLang: string,
  targetLang: string,
): Promise<TranslationResult[]> {
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

/** 设置翻译窗口固定状态 */
export async function setPinWindow(pinned: boolean): Promise<void> {
  return invoke("set_pin_window", { pinned });
}

// ─── 插件/服务管理 ──────────────────────────────────────────────────────────

/** 列出所有已注册的翻译插件 */
export async function listPlugins(): Promise<PluginDefinition[]> {
  return invoke("list_plugins");
}

/** 列出所有服务 */
export async function listServices(): Promise<ServiceConfig[]> {
  return invoke("list_services");
}

/** 添加服务 */
export async function addService(service: ServiceConfig): Promise<ServiceConfig[]> {
  return invoke("add_service", { service });
}

/** 更新服务 */
export async function updateService(id: string, service: ServiceConfig): Promise<ServiceConfig[]> {
  return invoke("update_service", { id, service });
}

/** 删除服务 */
export async function deleteService(id: string): Promise<ServiceConfig[]> {
  return invoke("delete_service", { id });
}

/** 切换服务启用/禁用 */
export async function toggleService(id: string, enabled: boolean): Promise<ServiceConfig[]> {
  return invoke("toggle_service", { id, enabled });
}

// ─── 全局配置 ──────────────────────────────────────────────────────────────

/** 获取全局配置 */
export async function getConfig(): Promise<Record<string, unknown>> {
  return invoke("get_config");
}

/** 更新全局配置 */
export async function updateConfig(general: Record<string, unknown>): Promise<void> {
  return invoke("update_config", { general });
}
