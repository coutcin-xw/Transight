mod commands;
mod engine;
mod selection;

use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager,
};

const TRANSLATE_WIN: &str = "translate";
const SETTINGS_WIN: &str = "settings";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            commands::window::show_translate_window,
            commands::window::hide_translate_window,
            commands::window::open_settings_window,
            commands::window::close_settings_window,
            commands::translate::translate,
            commands::translate::detect_language,
            commands::selection::get_selected_text,
        ])
        .setup(|app| {
            // ── 翻译弹窗 ──
            let _translate = tauri::WebviewWindowBuilder::new(
                app,
                TRANSLATE_WIN,
                tauri::WebviewUrl::App("/".into()),
            )
            .title("Transight")
            .inner_size(300.0, 540.0)
            .decorations(false)
            .always_on_top(true)
            .visible(false)
            .skip_taskbar(true)
            .shadow(true)
            .build()?;

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

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "show" => {
                        if let Some(w) = app.get_webview_window(TRANSLATE_WIN) {
                            let _ = w.show();
                            let _ = w.set_focus();
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

            // Ctrl+Shift+T: 翻译选中文本
            {
                let handle = app.handle().clone();
                let _ = app.global_shortcut().on_shortcut(
                    "Ctrl+Shift+T",
                    move |_app, _sc, _event| {
                        let h = handle.clone();
                        tauri::async_runtime::spawn(async move {
                            let text = crate::selection::get_selected_text().ok();
                            if let Some(w) = h.get_webview_window(TRANSLATE_WIN) {
                                let _ = w.show();
                                let _ = w.set_focus();
                                if let Some(text) = text {
                                    let _ = w.emit("selected-text", text);
                                }
                            }
                        });
                    },
                );
            }

            // Escape: 隐藏翻译弹窗
            {
                let handle = app.handle().clone();
                let _ = app.global_shortcut().on_shortcut("Escape", move |_app, _sc, _event| {
                    if let Some(w) = handle.get_webview_window(TRANSLATE_WIN) {
                        let _ = w.hide();
                    }
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
