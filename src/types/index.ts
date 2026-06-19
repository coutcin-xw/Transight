// Tauri API 类型定义

/** 翻译结果 */
export interface TranslationResult {
  source_text: string;
  translated_text: string;
  source_lang: string;
  target_lang: string;
  provider: string;
  error?: string;  // 翻译失败时的错误信息
}

/** 语言对 */
export interface LanguagePair {
  source: string;
  target: string;
}

/** 插件配置 */
export interface PluginConfig {
  api_key?: string;
  api_url?: string;
  model?: string;
  timeout_secs: number;
}

/** 全局配置 */
export interface GlobalConfig {
  general: {
    defaultSourceLang: string;
    defaultTargetLang: string;
    autoDetectLang: boolean;
    autoCopyResult: boolean;
    autoShowOnStartup: boolean;
    stayOnTop: boolean;
    pinWindow: boolean;
  };
  hotkeys: Record<string, string>;
}

/** 服务 */
export interface Service {
  id: string;
  pluginId: string;
  name: string;
  enabled: boolean;
  config: Record<string, unknown>;
  priority: number;
  createdAt: string;
  updatedAt: string;
}

/** 插件定义 */
export interface PluginDefinition {
  id: string;
  name: string;
  version: string;
  description: string;
  adapterType: "builtin" | "generic_http";
}

/** 支持的语言 */
export const LANGUAGES: Record<string, string> = {
  auto: "自动检测",
  zh: "中文",
  en: "英语",
  ja: "日语",
  ko: "韩语",
  fr: "法语",
  de: "德语",
  es: "西班牙语",
  ru: "俄语",
  pt: "葡萄牙语",
  it: "意大利语",
  ar: "阿拉伯语",
  th: "泰语",
  vi: "越南语",
};
