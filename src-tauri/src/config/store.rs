//! 配置持久化 — JSON 文件存储
//!
//! 文件路径: ~/.config/transight/config.json
//! 线程安全: Arc<RwLock<Config>>

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

// ─── 数据模型 ────────────────────────────────────────────────────────────────

/// 全局配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub services: Vec<ServiceConfig>,
    #[serde(default)]
    pub shortcuts: ShortcutConfig,
}

fn default_version() -> u32 { 1 }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeneralConfig {
    #[serde(default = "default_lang")]
    pub default_source_lang: String,
    #[serde(default = "default_lang_zh")]
    pub default_target_lang: String,
    #[serde(default = "default_true")]
    pub auto_detect_lang: bool,
    #[serde(default)]
    pub auto_copy_result: bool,
    /// 翻译窗口默认是否固定 (pin)
    #[serde(default)]
    pub default_pin: bool,
    /// 主题: "light" | "dark" | "auto"
    #[serde(default = "default_theme")]
    pub theme: String,
}

fn default_lang() -> String { "auto".into() }
fn default_lang_zh() -> String { "zh".into() }
fn default_true() -> bool { true }
fn default_theme() -> String { "auto".into() }

/// 翻译服务配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub id: String,
    pub plugin_id: String,
    pub name: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub config: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShortcutConfig {
    #[serde(default = "default_translate_shortcut")]
    pub translate_selected: String,
    #[serde(default = "default_escape")]
    pub close_window: String,
}

fn default_translate_shortcut() -> String { "Ctrl+Alt+Q".into() }
fn default_escape() -> String { "Escape".into() }

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            general: GeneralConfig::default(),
            services: vec![
                ServiceConfig {
                    id: "svc-google".into(),
                    plugin_id: "google-translate".into(),
                    name: "Google 翻译".into(),
                    enabled: true,
                    config: serde_json::json!({}),
                },
                ServiceConfig {
                    id: "svc-deepl".into(),
                    plugin_id: "deepl".into(),
                    name: "DeepL".into(),
                    enabled: false,
                    config: serde_json::json!({
                        "api_key": ""
                    }),
                },
                ServiceConfig {
                    id: "svc-llm".into(),
                    plugin_id: "openai-compat".into(),
                    name: "LLM 翻译".into(),
                    enabled: false,
                    config: serde_json::json!({
                        "api_url": "https://api.openai.com/v1/chat/completions",
                        "api_key": "",
                        "model": "gpt-4o-mini"
                    }),
                },
            ],
            shortcuts: ShortcutConfig::default(),
        }
    }
}

// ─── ConfigStore ─────────────────────────────────────────────────────────────

/// 配置存储 (线程安全)
pub type ConfigStore = Arc<RwLock<Config>>;

/// 获取配置文件路径
fn config_path() -> PathBuf {
    let mut path = dirs_next().unwrap_or_else(|| PathBuf::from("."));
    path.push("transight");
    path.push("config.json");
    path
}

fn dirs_next() -> Option<PathBuf> {
    #[cfg(target_os = "linux")]
    {
        std::env::var("XDG_CONFIG_HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var("HOME")
                    .ok()
                    .map(|h| PathBuf::from(h).join(".config"))
            })
    }
    #[cfg(target_os = "macos")]
    {
        std::env::var("HOME")
            .ok()
            .map(|h| PathBuf::from(h).join("Library").join("Application Support"))
    }
    #[cfg(target_os = "windows")]
    {
        std::env::var("APPDATA")
            .ok()
            .map(PathBuf::from)
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        None
    }
}

/// 从文件加载配置，不存在则返回默认值
pub fn load_or_default() -> Config {
    let path = config_path();
    if path.exists() {
        fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    } else {
        let config = Config::default();
        // 创建目录和默认配置文件
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(&config) {
            let _ = fs::write(&path, json);
        }
        config
    }
}

/// 保存配置到文件
pub fn save(config: &Config) -> Result<(), String> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("创建配置目录失败: {e}"))?;
    }
    let json = serde_json::to_string_pretty(config).map_err(|e| format!("序列化失败: {e}"))?;
    fs::write(&path, json).map_err(|e| format!("写入失败: {e}"))?;
    Ok(())
}
