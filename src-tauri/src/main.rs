#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Url, WebviewUrl, WebviewWindowBuilder};

const START_URL: &str = "https://syndix.tijay006.de/os/teams";
const ALLOWED_PREFIXES: [&str; 4] = [
    "https://syndix.tijay006.de/os/",
    "https://syndix.tijay006.de/auth/",
    "https://syndix.tijay006.de/api/",
    "https://discord.com/",
];

fn is_allowed(url: &Url) -> bool {
    let s = url.as_str();
    ALLOWED_PREFIXES.iter().any(|prefix| s.starts_with(prefix))
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let nav_handle = app.handle().clone();
            let popup_handle = app.handle().clone();
            let start: Url = START_URL.parse().unwrap();

            WebviewWindowBuilder::new(app, "main", WebviewUrl::External(start))
                .title("Syndix")
                .inner_size(1280.0, 800.0)
                .resizable(true)
                .on_navigation(move |url| {
                    if is_allowed(url) {
                        true
                    } else {
                        if let Some(window) = nav_handle.get_webview_window("main") {
                            let _ = window.navigate(Url::parse(START_URL).unwrap());
                        }
                        false
                    }
                })
                .on_new_window(move |url, _features| {
                    if is_allowed(&url) {
                        if let Some(window) = popup_handle.get_webview_window("main") {
                            let _ = window.navigate(url);
                        }
                    } else if let Some(window) = popup_handle.get_webview_window("main") {
                        let _ = window.navigate(Url::parse(START_URL).unwrap());
                    }
                    tauri::webview::NewWindowResponse::Deny
                })
                .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Fehler beim Starten der Syndix-App");
}
