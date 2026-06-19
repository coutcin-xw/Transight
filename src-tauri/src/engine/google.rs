//! Google Translate 适配器 (免费 Web API)

use crate::engine::translator::{PluginConfig, TranslationResult, Translator};

pub struct GoogleTranslator;

impl GoogleTranslator {
    pub fn new() -> Self {
        Self
    }
}

impl Translator for GoogleTranslator {
    fn id(&self) -> &str {
        "google-translate"
    }

    fn name(&self) -> &str {
        "Google Translate"
    }

    async fn translate(
        &self,
        text: &str,
        source_lang: &str,
        target_lang: &str,
        _config: &PluginConfig,
    ) -> Result<TranslationResult, String> {
        let client = reqwest::Client::new();

        let sl = if source_lang == "auto" {
            "auto"
        } else {
            source_lang
        };

        // Google Translate 免费 Web API
        let url = format!(
            "https://translate.googleapis.com/translate_a/single\
             ?client=gtx&sl={sl}&tl={target_lang}&dt=t&q={}",
            urlencoding(text)
        );

        let resp = client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .timeout(std::time::Duration::from_secs(_config.timeout_secs))
            .send()
            .await
            .map_err(|e| format!("请求失败: {e}"))?;

        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("解析响应失败: {e}"))?;

        // Google API 响应格式: [[["translated text", "source", ...]], ...]
        let translated = body[0][0][0]
            .as_str()
            .ok_or("无法解析翻译结果")?
            .to_string();

        // 提取检测到的源语言
        let detected_lang = body[2]
            .as_str()
            .unwrap_or(sl)
            .to_string();

        Ok(TranslationResult {
            source_text: text.to_string(),
            translated_text: translated,
            source_lang: detected_lang,
            target_lang: target_lang.to_string(),
            provider: "Google Translate".to_string(),
        })
    }
}

/// URL 编码 (简单实现)
fn urlencoding(s: &str) -> String {
    let mut result = String::with_capacity(s.len() * 3);
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9'
            | b'-' | b'_' | b'.' | b'~' => result.push(byte as char),
            _ => {
                result.push('%');
                result.push(hex(byte >> 4));
                result.push(hex(byte & 0x0F));
            }
        }
    }
    result
}

fn hex(n: u8) -> char {
    match n {
        0..=9 => (b'0' + n) as char,
        _ => (b'A' + n - 10) as char,
    }
}
