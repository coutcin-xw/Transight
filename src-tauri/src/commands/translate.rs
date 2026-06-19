use crate::config::store::{save, ConfigStore, ServiceConfig};
use crate::engine::registry::PluginRegistry;
use crate::engine::translator::TranslationResult;
use serde::Serialize;
use std::sync::Arc;
use tauri::State;

/// 执行翻译 — 从 ConfigStore 读取启用的服务，通过 PluginRegistry 并行翻译
#[tauri::command]
pub async fn translate(
    text: String,
    source_lang: String,
    target_lang: String,
    store: State<'_, ConfigStore>,
    registry: State<'_, Arc<PluginRegistry>>,
) -> Result<Vec<TranslationResultDto>, String> {
    let services = {
        let config = store.read().map_err(|e| format!("读取配置失败: {e}"))?;
        config.services.clone()
    };

    let enabled: Vec<ServiceConfig> =
        services.into_iter().filter(|s| s.enabled).collect();

    if enabled.is_empty() {
        return Err("没有启用的翻译服务".into());
    }

    let results = registry
        .translate_all(&text, &source_lang, &target_lang, &enabled)
        .await;

    Ok(results.into_iter().map(Into::into).collect())
}

/// 自动检测文本语言
#[tauri::command]
pub async fn detect_language(text: String) -> Result<String, String> {
    if text.is_empty() {
        return Ok("auto".into());
    }
    let has_cjk = text
        .chars()
        .any(|c| ('\u{4E00}'..='\u{9FFF}').contains(&c));
    let has_ja = text
        .chars()
        .any(|c| ('\u{3040}'..='\u{309F}').contains(&c) || ('\u{30A0}'..='\u{30FF}').contains(&c));
    let has_ko = text
        .chars()
        .any(|c| ('\u{AC00}'..='\u{D7AF}').contains(&c));
    if has_ja { Ok("ja".into()) }
    else if has_ko { Ok("ko".into()) }
    else if has_cjk { Ok("zh".into()) }
    else { Ok("en".into()) }
}

// ─── 插件/服务管理命令 ──────────────────────────────────────────────────────

/// 列出所有已注册的插件
#[tauri::command]
pub async fn list_plugins(
    registry: State<'_, Arc<PluginRegistry>>,
) -> Result<Vec<PluginInfoDto>, String> {
    Ok(registry
        .list_plugins()
        .into_iter()
        .map(|p| PluginInfoDto {
            id: p.id,
            name: p.name,
            description: p.description,
            adapter_type: format!("{:?}", p.adapter_type).to_lowercase(),
            config_schema: p
                .config_schema
                .into_iter()
                .map(|f| ConfigFieldDto {
                    key: f.key,
                    field_type: f.field_type,
                    label: f.label,
                    required: f.required,
                    secret: f.secret,
                    default: f.default,
                })
                .collect(),
        })
        .collect())
}

/// 列出所有服务
#[tauri::command]
pub async fn list_services(
    store: State<'_, ConfigStore>,
) -> Result<Vec<ServiceConfigDto>, String> {
    let config = store.read().map_err(|e| format!("{e}"))?;
    Ok(config.services.iter().map(|s| ServiceConfigDto {
        id: s.id.clone(),
        plugin_id: s.plugin_id.clone(),
        name: s.name.clone(),
        enabled: s.enabled,
        config: s.config.clone(),
    }).collect())
}

/// 添加服务
#[tauri::command]
pub async fn add_service(
    store: State<'_, ConfigStore>,
    service: ServiceConfigDto,
) -> Result<Vec<ServiceConfigDto>, String> {
    let results = crate::services::manager::add_service(
        &store,
        ServiceConfig {
            id: service.id,
            plugin_id: service.plugin_id,
            name: service.name,
            enabled: service.enabled,
            config: service.config,
        },
    )?;
    Ok(results.into_iter().map(Into::into).collect())
}

/// 更新服务
#[tauri::command]
pub async fn update_service(
    store: State<'_, ConfigStore>,
    id: String,
    service: ServiceConfigDto,
) -> Result<Vec<ServiceConfigDto>, String> {
    let results = crate::services::manager::update_service(&store, &id, |s| {
        s.name = service.name.clone();
        s.plugin_id = service.plugin_id.clone();
        s.enabled = service.enabled;
        s.config = service.config.clone();
    })?;
    Ok(results.into_iter().map(Into::into).collect())
}

/// 删除服务
#[tauri::command]
pub async fn delete_service(
    store: State<'_, ConfigStore>,
    id: String,
) -> Result<Vec<ServiceConfigDto>, String> {
    let results = crate::services::manager::delete_service(&store, &id)?;
    Ok(results.into_iter().map(Into::into).collect())
}

/// 切换服务启用/禁用
#[tauri::command]
pub async fn toggle_service(
    store: State<'_, ConfigStore>,
    id: String,
    enabled: bool,
) -> Result<Vec<ServiceConfigDto>, String> {
    let results = crate::services::manager::toggle_service(&store, &id, enabled)?;
    // 保存到文件
    {
        let config = store.read().map_err(|e| format!("{e}"))?;
        save(&config)?;
    }
    Ok(results.into_iter().map(Into::into).collect())
}

// ─── DTO 类型 (前端友好) ────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct TranslationResultDto {
    pub source_text: String,
    pub translated_text: String,
    pub source_lang: String,
    pub target_lang: String,
    pub provider: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl From<TranslationResult> for TranslationResultDto {
    fn from(r: TranslationResult) -> Self {
        Self {
            source_text: r.source_text,
            translated_text: r.translated_text,
            source_lang: r.source_lang,
            target_lang: r.target_lang,
            provider: r.provider,
            error: r.error,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PluginInfoDto {
    pub id: String,
    pub name: String,
    pub description: String,
    pub adapter_type: String,
    pub config_schema: Vec<ConfigFieldDto>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConfigFieldDto {
    pub key: String,
    pub field_type: String,
    pub label: String,
    pub required: bool,
    pub secret: bool,
    pub default: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct ServiceConfigDto {
    pub id: String,
    pub plugin_id: String,
    pub name: String,
    pub enabled: bool,
    pub config: serde_json::Value,
}

// ─── 全局配置命令 ──────────────────────────────────────────────────────────

/// 获取全局配置
#[tauri::command]
pub async fn get_config(
    store: State<'_, ConfigStore>,
) -> Result<serde_json::Value, String> {
    let config = store.read().map_err(|e| format!("{e}"))?;
    serde_json::to_value(&*config).map_err(|e| format!("{e}"))
}

/// 更新全局配置
#[tauri::command]
pub async fn update_config(
    store: State<'_, ConfigStore>,
    general: Option<serde_json::Value>,
) -> Result<(), String> {
    let mut config = store.write().map_err(|e| format!("{e}"))?;
    if let Some(g) = general {
        if let Ok(general_cfg) = serde_json::from_value(g) {
            config.general = general_cfg;
        }
    }
    save(&config)?;
    Ok(())
}

impl From<ServiceConfig> for ServiceConfigDto {
    fn from(s: ServiceConfig) -> Self {
        Self {
            id: s.id,
            plugin_id: s.plugin_id,
            name: s.name,
            enabled: s.enabled,
            config: s.config,
        }
    }
}
