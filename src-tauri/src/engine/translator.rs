//! 翻译器插件接口

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// 翻译结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationResult {
    pub source_text: String,
    pub translated_text: String,
    pub source_lang: String,
    pub target_lang: String,
    pub provider: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 语言对
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguagePair {
    pub source: String,
    pub target: String,
}

/// 翻译器类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TranslatorType {
    /// 传统翻译 API: 请求/响应格式固定, 直接返回翻译文本
    Traditional,
    /// LLM 翻译: 通过 Prompt 引导翻译, 需从响应中提取文本
    Llm,
}

/// 插件配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    // ── 通用 ──
    pub api_key: Option<String>,
    pub api_url: Option<String>,
    pub timeout_secs: u64,

    // ── LLM 专用 ──
    /// 模型名称 (gpt-4o-mini / llama3 / ...)
    pub model: Option<String>,
    /// Prompt 模板: {{source_lang}} {{target_lang}} {{text}} 为占位符
    pub prompt_template: Option<String>,
    /// JSONPath 提取翻译结果 (默认 $.choices[0].message.content)
    pub response_path: Option<String>,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            api_url: None,
            timeout_secs: 10,
            model: None,
            prompt_template: None,
            response_path: None,
        }
    }
}

impl PluginConfig {
    /// LLM 默认 Prompt
    pub fn default_llm_prompt() -> String {
        "你是一个专业翻译助手。将用户输入的文本从{{source_lang}}翻译成{{target_lang}}。\
         只输出翻译结果，不要添加任何解释、注释或额外内容。保持原文格式和语气。\
         \n\n用户输入: {{text}}".into()
    }

    /// LLM 默认响应提取路径
    pub fn default_llm_response_path() -> String {
        "$.choices[0].message.content".into()
    }
}

/// 翻译器插件接口
#[async_trait]
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
