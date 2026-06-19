use crate::engine::google::GoogleTranslator;
use crate::engine::translator::Translator;

/// 执行翻译
#[tauri::command]
pub async fn translate(
    text: String,
    source_lang: String,
    target_lang: String,
) -> Result<String, String> {
    let translator = GoogleTranslator::new();
    let config = Default::default();
    let result = translator
        .translate(&text, &source_lang, &target_lang, &config)
        .await
        .map_err(|e| format!("翻译失败: {e}"))?;
    Ok(result.translated_text)
}

/// 自动检测文本语言
#[tauri::command]
pub async fn detect_language(text: String) -> Result<String, String> {
    // Google Translate 在翻译时可自动检测，这里快速返回
    if text.is_empty() {
        return Ok("auto".into());
    }
    // 简单启发式检测 (M2 将使用 whatlang)
    let has_cjk = text
        .chars()
        .any(|c| ('\u{4E00}'..='\u{9FFF}').contains(&c));
    let has_japanese = text
        .chars()
        .any(|c| ('\u{3040}'..='\u{309F}').contains(&c) || ('\u{30A0}'..='\u{30FF}').contains(&c));
    let has_korean = text
        .chars()
        .any(|c| ('\u{AC00}'..='\u{D7AF}').contains(&c));

    if has_japanese {
        Ok("ja".into())
    } else if has_korean {
        Ok("ko".into())
    } else if has_cjk {
        Ok("zh".into())
    } else {
        Ok("en".into())
    }
}
