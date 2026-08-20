#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Url, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_opener::OpenerExt;

const START_URL: &str = "https://syndix.tijay006.de/os/teams";
const ALLOWED_PREFIXES: [&str; 4] = [
    "https://syndix.tijay006.de/os/",
    "https://syndix.tijay006.de/auth/",
    "https://syndix.tijay006.de/api/",
    "https://discord.com/",
];
const LEGAL_PATHS: [&str; 2] = ["/legal/terms", "/legal/privacy"];

const INIT_SCRIPT: &str = r#"
document.addEventListener('click', function (event) {
  var target = event.target;
  if (!(target instanceof Element)) return;
  var anchor = target.closest('a[href]');
  if (!anchor) return;
  var path = '';
  try { path = new URL(anchor.href, window.location.origin).pathname.replace(/\/+$/, ''); }
  catch (e) { return; }
  if (path === '/legal/terms' || path === '/legal/privacy') {
    event.preventDefault();
    event.stopPropagation();
    window.open(anchor.href);
  }
}, true);
"#;

fn is_allowed(url: &Url) -> bool {
    let s = url.as_str();
    ALLOWED_PREFIXES.iter().any(|prefix| s.starts_with(prefix))
}

fn is_legal(url: &Url) -> bool {
    let path = url.path().trim_end_matches('/');
    LEGAL_PATHS.contains(&path)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let nav_handle = app.handle().clone();
            let popup_handle = app.handle().clone();
            let start: Url = START_URL.parse().unwrap();

            let mut builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::External(start))
                .title("Syndix")
                .inner_size(1280.0, 800.0)
                .resizable(true)
                .initialization_script(INIT_SCRIPT);

            if let Ok(base) = app.path().local_data_dir() {
                builder = builder.data_directory(base.join("SyndixData"));
            }

            builder
                .on_navigation(move |url| {
                    if is_legal(url) {
                        let _ = nav_handle.opener().open_url(url.as_str(), None::<&str>);
                        false
                    } else if is_allowed(url) {
                        true
                    } else {
                        if let Some(window) = nav_handle.get_webview_window("main") {
                            let _ = window.navigate(Url::parse(START_URL).unwrap());
                        }
                        false
                    }
                })
                .on_new_window(move |url, _features| {
                    if is_legal(&url) {
                        let _ = popup_handle.opener().open_url(url.as_str(), None::<&str>);
                    } else if is_allowed(&url) {
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
