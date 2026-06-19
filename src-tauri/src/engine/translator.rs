//! 翻译器插件接口 (Rust edition 2024 — 原生 async trait)

use serde::{Deserialize, Serialize};

/// 翻译结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationResult {
    pub source_text: String,
    pub translated_text: String,
    pub source_lang: String,
    pub target_lang: String,
    pub provider: String,
}

/// 语言对
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguagePair {
    pub source: String,
    pub target: String,
}

/// 插件配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub api_key: Option<String>,
    pub api_url: Option<String>,
    pub model: Option<String>,
    pub timeout_secs: u64,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            api_url: None,
            model: None,
            timeout_secs: 10,
        }
    }
}

/// 翻译器插件接口
pub trait Translator: Send + Sync {
    /// 插件唯一标识
    fn id(&self) -> &str;
    /// 插件名称
    fn name(&self) -> &str;
    /// 执行翻译
    async fn translate(
        &self,
        text: &str,
        source_lang: &str,
        target_lang: &str,
        config: &PluginConfig,
    ) -> Result<TranslationResult, String>;
}
