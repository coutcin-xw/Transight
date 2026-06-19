mod commands;
mod config;
mod engine;
mod selection;
mod services;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use config::store::{load_or_default, ConfigStore};
use engine::registry::PluginRegistry;
use engine::translator::PluginConfig;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager,
};

const TRANSLATE_WIN: &str = "translate";
const SETTINGS_WIN: &str = "settings";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let is_pinned = Arc::new(AtomicBool::new(false));
    let config_store: ConfigStore = Arc::new(std::sync::RwLock::new(load_or_default()));
    let plugin_registry = Arc::new(PluginRegistry::new());

    // 注册内置插件
    {
        use engine::deepl::DeepLTranslator;
        use engine::google::GoogleTranslator;
        use engine::openai::OpenAICompatTranslator;
        use engine::registry::{AdapterType, ConfigField, PluginInfo};

        // Google 翻译
        plugin_registry.register(
            Arc::new(GoogleTranslator::new()),
            PluginInfo {
                id: "google-translate".into(),
                name: "Google 翻译".into(),
                description: "Google 免费翻译 Web API".into(),
                adapter_type: AdapterType::Builtin,
                config_schema: vec![],
            },
        );

        // DeepL
        plugin_registry.register(
            Arc::new(DeepLTranslator::new()),
            PluginInfo {
                id: "deepl".into(),
                name: "DeepL".into(),
                description: "DeepL 翻译 REST API".into(),
                adapter_type: AdapterType::Builtin,
                config_schema: vec![ConfigField {
                    key: "api_key".into(),
                    field_type: "string".into(),
                    label: "API Key".into(),
                    required: true,
                    secret: true,
                    default: None,
                }],
            },
        );

        // OpenAI 兼容
        plugin_registry.register(
            Arc::new(OpenAICompatTranslator::new()),
            PluginInfo {
                id: "openai-compat".into(),
                name: "OpenAI 兼容".into(),
                description: "OpenAI / Ollama / 兼容接口".into(),
                adapter_type: AdapterType::Builtin,
                config_schema: vec![
                    ConfigField {
                        key: "api_url".into(),
                        field_type: "string".into(),
                        label: "API 地址 (Chat Completions)".into(),
                        required: false,
                        secret: false,
                        default: Some(serde_json::json!("https://api.openai.com/v1/chat/completions")),
                    },
                    ConfigField {
                        key: "api_key".into(),
                        field_type: "string".into(),
                        label: "API Key (Ollama 可留空)".into(),
                        required: false,
                        secret: true,
                        default: None,
                    },
                    ConfigField {
                        key: "model".into(),
                        field_type: "string".into(),
                        label: "模型".into(),
                        required: false,
                        secret: false,
                        default: Some(serde_json::json!("gpt-4o-mini")),
                    },
                    ConfigField {
                        key: "prompt_template".into(),
                        field_type: "text".into(),
                        label: "Prompt 模板".into(),
                        required: false,
                        secret: false,
                        default: Some(serde_json::json!(PluginConfig::default_llm_prompt())),
                    },
                    ConfigField {
                        key: "response_path".into(),
                        field_type: "string".into(),
                        label: "响应提取 JSONPath".into(),
                        required: false,
                        secret: false,
                        default: Some(serde_json::json!("$.choices[0].message.content")),
                    },
                ],
            },
        );
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(is_pinned.clone())
        .manage(config_store.clone())
        .manage(plugin_registry.clone())
        .invoke_handler(tauri::generate_handler![
            commands::window::show_translate_window,
            commands::window::hide_translate_window,
            commands::window::open_settings_window,
            commands::window::close_settings_window,
            commands::window::set_pin_window,
            commands::translate::translate,
            commands::translate::detect_language,
            commands::translate::list_plugins,
            commands::translate::list_services,
            commands::translate::add_service,
            commands::translate::update_service,
            commands::translate::delete_service,
            commands::translate::toggle_service,
            commands::translate::get_config,
            commands::translate::update_config,
            commands::selection::get_selected_text,
        ])
        .setup(move |app| {
            let pinned = is_pinned.clone();

            // ── 翻译弹窗 ──
            let translate = tauri::WebviewWindowBuilder::new(
                app,
                TRANSLATE_WIN,
                tauri::WebviewUrl::App("/".into()),
            )
            .title("Transight")
            .inner_size(300.0, 540.0)
            .decorations(false)
            .always_on_top(false)
            .visible(false)
            .skip_taskbar(true)
            .shadow(true)
            .build()?;

            let pinned_focus = pinned.clone();
            let handle = app.handle().clone();
            translate.on_window_event(move |event| {
                if let tauri::WindowEvent::Focused(false) = event {
                    if !pinned_focus.load(Ordering::Relaxed) {
                        if let Some(w) = handle.get_webview_window(TRANSLATE_WIN) {
                            let _ = w.hide();
                        }
                    }
                }
            });

            // ── 设置窗口 ──
            let _settings = tauri::WebviewWindowBuilder::new(
                app,
                SETTINGS_WIN,
                tauri::WebviewUrl::App("/#/settings".into()),
            )
            .title("Transight - 设置")
            .inner_size(700.0, 540.0)
            .decorations(false)
            .visible(false)
            .center()
            .shadow(true)
            .build()?;

            // ── 系统托盘 ──
            let show_item = MenuItemBuilder::with_id("show", "显示翻译窗口").build(app)?;
            let settings_item =
                MenuItemBuilder::with_id("settings", "打开设置").build(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "退出").build(app)?;

            let menu = MenuBuilder::new(app)
                .item(&show_item)
                .item(&settings_item)
                .separator()
                .item(&quit_item)
                .build()?;

            let tray_icon =
                tauri::image::Image::from_bytes(include_bytes!("../icons/32x32.png"))
                    .expect("Failed to load tray icon");

            let pinned_tray = pinned.clone();
            let cfg_tray = config_store.clone();
            let _tray = TrayIconBuilder::new()
                .icon(tray_icon)
                .menu(&menu)
                .on_menu_event(move |app, event| {
                    let p = pinned_tray.clone();
                    let cfg = cfg_tray.clone();
                    match event.id().as_ref() {
                        "show" => {
                            // 托盘打开: 跟随配置 default_pin
                            let pin = cfg
                                .read()
                                .map(|c| c.general.default_pin)
                                .unwrap_or(false);
                            if let Some(w) = app.get_webview_window(TRANSLATE_WIN) {
                                let _ = w.show();
                                let _ = w.set_focus();
                                p.store(pin, Ordering::Relaxed);
                                let _ = app.emit("pin-changed", pin);
                            }
                        }
                        "settings" => {
                            if let Some(w) = app.get_webview_window(SETTINGS_WIN) {
                                let _ = w.show();
                                let _ = w.set_focus();
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(w) = app.get_webview_window(TRANSLATE_WIN) {
                            let _ = w.show();
                            let _ = w.set_focus();
                        }
                    }
                })
                .build(app)?;

            // ── 全局快捷键 ──
            use tauri_plugin_global_shortcut::GlobalShortcutExt;

            {
                let handle = app.handle().clone();
                let pinned_s = pinned.clone();
                let cfg_s = config_store.clone();
                let _ = app.global_shortcut().on_shortcut(
                    "Ctrl+Alt+Q",
                    move |_app, _sc, _event| {
                        let h = handle.clone();
                        let text_result = crate::selection::get_selected_text();
                        // 读取配置的默认 pin 状态
                        let default_pin = cfg_s
                            .read()
                            .map(|c| c.general.default_pin)
                            .unwrap_or(false);
                        if let Some(w) = h.get_webview_window(TRANSLATE_WIN) {
                            let _ = w.show();
                            let _ = w.set_focus();
                            pinned_s.store(default_pin, Ordering::Relaxed);
                            let _ = h.emit("pin-changed", default_pin);
                            match text_result {
                                Ok(text) if !text.is_empty() => {
                                    eprintln!(
                                        "[transight] selected: {}",
                                        &text[..text.len().min(50)]
                                    );
                                    let _ = h.emit("selected-text", text);
                                }
                                Ok(_) => eprintln!("[transight] selection empty"),
                                Err(e) => eprintln!("[transight] selection failed: {e}"),
                            }
                        }
                    },
                );
            }

            {
                let handle = app.handle().clone();
                let _ = app.global_shortcut().on_shortcut(
                    "Escape",
                    move |_app, _sc, _event| {
                        if let Some(w) = handle.get_webview_window(TRANSLATE_WIN) {
                            let _ = w.hide();
                        }
                    },
                );
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
