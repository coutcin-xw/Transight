//! LLM 翻译适配器
//! 通过 Prompt 模板引导大模型翻译，JSONPath 提取结果。
//! 支持 OpenAI / Ollama / vLLM / 任何 Chat Completions 兼容接口。

use async_trait::async_trait;
use crate::engine::translator::{PluginConfig, TranslationResult, Translator};

pub struct OpenAICompatTranslator;

impl OpenAICompatTranslator {
    pub fn new() -> Self {
        Self
    }

    /// 渲染 Prompt 模板: 替换 {{source_lang}} {{target_lang}} {{text}}
    fn render_prompt(template: &str, source_lang: &str, target_lang: &str, text: &str) -> String {
        template
            .replace("{{source_lang}}", &lang_name(source_lang))
            .replace("{{target_lang}}", &lang_name(target_lang))
            .replace("{{text}}", text)
    }

    /// 用 JSONPath 从响应中提取翻译文本
    fn extract_text(body: &serde_json::Value, path: &str) -> Result<String, String> {
        // 简单 JSONPath: $.choices[0].message.content
        // 支持格式: $.a.b[0].c
        let parts: Vec<&str> = path
            .trim_start_matches("$.")
            .split('.')
            .collect();

        let mut current = body;
        for part in &parts {
            // 解析 xxx[N] 格式
            if let Some(bracket) = part.find('[') {
                let key = &part[..bracket];
                let idx_str = &part[bracket + 1..part.len() - 1];
                let idx: usize = idx_str
                    .parse()
                    .map_err(|_| format!("JSONPath 索引无效: {idx_str}"))?;

                if !key.is_empty() {
                    current = current
                        .get(key)
                        .ok_or_else(|| format!("JSONPath 缺少键: {key}"))?;
                }
                current = current
                    .get(idx)
                    .ok_or_else(|| format!("JSONPath 索引越界: [{idx}]"))?;
            } else {
                current = current
                    .get(part)
                    .ok_or_else(|| format!("JSONPath 缺少键: {part}"))?;
            }
        }

        current
            .as_str()
            .map(|s| s.trim().to_string())
            .ok_or_else(|| "JSONPath 提取结果不是字符串".into())
    }
}

#[async_trait]
impl Translator for OpenAICompatTranslator {
    fn id(&self) -> &str {
        "openai-compat"
    }

    fn name(&self) -> &str {
        "LLM 翻译"
    }

    async fn translate(
        &self,
        text: &str,
        source_lang: &str,
        target_lang: &str,
        config: &PluginConfig,
    ) -> Result<TranslationResult, String> {
        let api_url = config
            .api_url
            .clone()
            .unwrap_or_else(|| "https://api.openai.com/v1/chat/completions".into());

        let model = config
            .model
            .clone()
            .unwrap_or_else(|| "gpt-4o-mini".into());

        let api_key = config.api_key.clone().unwrap_or_default();

        let prompt_template = config
            .prompt_template
            .clone()
            .unwrap_or_else(PluginConfig::default_llm_prompt);

        let response_path = config
            .response_path
            .clone()
            .unwrap_or_else(PluginConfig::default_llm_response_path);

        let system_prompt =
            Self::render_prompt(&prompt_template, source_lang, target_lang, text);

        let client = reqwest::Client::new();
        let mut req = client
            .post(&api_url)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model": model,
                "messages": [
                    { "role": "user", "content": system_prompt }
                ],
                "temperature": 0.3,
                "max_tokens": 4096,
            }))
            .timeout(std::time::Duration::from_secs(config.timeout_secs));

        if !api_key.is_empty() {
            req = req.header("Authorization", format!("Bearer {api_key}"));
        }

        let resp = req
            .send()
            .await
            .map_err(|e| format!("请求失败: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("HTTP {status}: {body}"));
        }

        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("响应解析失败: {e}"))?;

        let translated = Self::extract_text(&body, &response_path)?;

        Ok(TranslationResult {
            source_text: text.to_string(),
            translated_text: translated,
            source_lang: source_lang.to_string(),
            target_lang: target_lang.to_string(),
            provider: format!("LLM ({model})"),
            error: None,
        })
    }
}

fn lang_name(code: &str) -> &str {
    match code {
        "zh" => "中文",
        "en" => "英语",
        "ja" => "日语",
        "ko" => "韩语",
        "fr" => "法语",
        "de" => "德语",
        "es" => "西班牙语",
        "ru" => "俄语",
        "pt" => "葡萄牙语",
        "it" => "意大利语",
        "ar" => "阿拉伯语",
        "th" => "泰语",
        "vi" => "越南语",
        "auto" => "源语言",
        _ => code,
    }
}
