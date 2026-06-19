// Tauri API 类型定义

/**
 * 窗口 resize 拖拽方向。
 * 来源: @tauri-apps/api/window.d.ts:72（上游未 export，故在此声明）。
 * 上游导出后删除此声明即可。
 */
export type ResizeDirection =
  | 'East'
  | 'North'
  | 'NorthEast'
  | 'NorthWest'
  | 'South'
  | 'SouthEast'
  | 'SouthWest'
  | 'West';

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

/** 服务配置 (与后端 ServiceConfigDto 对应) */
export interface ServiceConfig {
  id: string;
  plugin_id: string;
  name: string;
  enabled: boolean;
  config: Record<string, unknown>;
}

/** 插件定义 (与后端 PluginInfoDto 对应) */
export interface PluginDefinition {
  id: string;
  name: string;
  description: string;
  adapter_type: string;
  config_schema: ConfigField[];
}

/** 插件配置字段 */
export interface ConfigField {
  key: string;
  field_type: string;
  label: string;
  required: boolean;
  secret: boolean;
  default?: unknown;
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
