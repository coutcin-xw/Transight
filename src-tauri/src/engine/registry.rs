//! 插件注册表 — 管理所有可用的翻译插件
//!
//! 启动时注册所有内置插件。自定义 HTTP 插件通过 register_http() 动态添加。

use crate::config::store::ServiceConfig;
use crate::engine::translator::{TranslationResult, Translator};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// 插件元信息 (不包含实例，用于列表展示)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub adapter_type: AdapterType,
    /// 配置项 schema (前端用于生成表单)
    pub config_schema: Vec<ConfigField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterType {
    Builtin,
    GenericHttp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigField {
    pub key: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub label: String,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub secret: bool,
    #[serde(default)]
    pub default: Option<serde_json::Value>,
}

// ─── 注册表 ──────────────────────────────────────────────────────────────────

/// 插件注册表 (线程安全)
pub struct PluginRegistry {
    plugins: RwLock<HashMap<String, Arc<dyn Translator>>>,
    infos: RwLock<Vec<PluginInfo>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: RwLock::new(HashMap::new()),
            infos: RwLock::new(Vec::new()),
        }
    }

    /// 注册一个内置插件
    pub fn register(&self, translator: Arc<dyn Translator>, info: PluginInfo) {
        self.plugins
            .write()
            .unwrap()
            .insert(info.id.clone(), translator);
        self.infos.write().unwrap().push(info);
    }

    /// 遍历已启用的服务，并行执行翻译
    pub async fn translate_all(
        &self,
        text: &str,
        source_lang: &str,
        target_lang: &str,
        enabled_services: &[ServiceConfig],
    ) -> Vec<TranslationResult> {
        use crate::engine::translator::PluginConfig;

        // 收集任务 (先 clone Arc, 再释放锁)
        let mut tasks = Vec::new();
        {
            let plugins = self.plugins.read().unwrap();
            for service in enabled_services {
                if !service.enabled {
                    continue;
                }
                if let Some(translator) = plugins.get(&service.plugin_id) {
                    let translator = Arc::clone(translator);
                    let text = text.to_string();
                    let sl = source_lang.to_string();
                    let tl = target_lang.to_string();
                    let cfg = PluginConfig {
                        api_key: service.config.get("api_key").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        api_url: service.config.get("api_url").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        timeout_secs: service.config.get("timeout_secs").and_then(|v| v.as_u64()).unwrap_or(10),
                        model: service.config.get("model").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        prompt_template: service.config.get("prompt_template").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        response_path: service.config.get("response_path").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    };
                    let provider = translator.name().to_string();

                    tasks.push(tokio::spawn(async move {
                        match translator.translate(&text, &sl, &tl, &cfg).await {
                            Ok(mut r) => {
                                r.provider = provider;
                                r
                            }
                            Err(e) => TranslationResult {
                                source_text: text.clone(),
                                translated_text: String::new(),
                                source_lang: sl.clone(),
                                target_lang: tl.clone(),
                                provider,
                                error: Some(format!("翻译失败: {e}")),
                            },
                        }
                    }));
                }
            }
        } // 释放 plugins 锁

        // 等待所有任务
        let mut results = Vec::new();
        for task in tasks {
            if let Ok(result) = task.await {
                results.push(result);
            }
        }
        results
    }

    /// 列出所有已注册插件的信息
    pub fn list_plugins(&self) -> Vec<PluginInfo> {
        self.infos.read().unwrap().clone()
    }
}
