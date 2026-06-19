//! DeepL 翻译适配器

use async_trait::async_trait;
use crate::engine::translator::{PluginConfig, TranslationResult, Translator};

pub struct DeepLTranslator;

impl DeepLTranslator {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Translator for DeepLTranslator {
    fn id(&self) -> &str {
        "deepl"
    }

    fn name(&self) -> &str {
        "DeepL"
    }

    async fn translate(
        &self,
        text: &str,
        source_lang: &str,
        target_lang: &str,
        config: &PluginConfig,
    ) -> Result<TranslationResult, String> {
        let api_key = config
            .api_key
            .as_ref()
            .ok_or("DeepL 需要配置 API Key")?;

        let api_url = config
            .api_url
            .clone()
            .unwrap_or_else(|| "https://api-free.deepl.com/v2/translate".into());

        let sl = if source_lang == "auto" {
            String::new() // DeepL auto-detects when source_lang is omitted
        } else {
            source_lang.to_uppercase()
        };
        let tl = target_lang.to_uppercase();

        let client = reqwest::Client::new();
        let resp = client
            .post(&api_url)
            .header("Authorization", format!("DeepL-Auth-Key {api_key}"))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "text": [text],
                "source_lang": sl,
                "target_lang": tl,
            }))
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .send()
            .await
            .map_err(|e| format!("DeepL 请求失败: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("DeepL HTTP {status}: {body}"));
        }

        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("DeepL 响应解析失败: {e}"))?;

        let translations = body["translations"]
            .as_array()
            .ok_or("DeepL 响应格式错误: 缺少 translations")?;

        let first = translations
            .first()
            .ok_or("DeepL 返回空结果")?;

        let translated = first["text"]
            .as_str()
            .ok_or("DeepL 响应格式错误: 缺少 text")?
            .to_string();

        let detected = first["detected_source_language"]
            .as_str()
            .unwrap_or(source_lang)
            .to_lowercase();

        Ok(TranslationResult {
            source_text: text.to_string(),
            translated_text: translated,
            source_lang: detected,
            target_lang: target_lang.to_string(),
            provider: "DeepL".to_string(),
            error: None,
        })
    }
}
