use tauri::{window, WebviewWindowBuilder};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::External("https://web.whatsapp.com".parse().unwrap()),
            )
            .title("WhatsApp")
            .inner_size(1100.0, 750.0)
            .min_inner_size(400.0, 500.0)
            .on_navigation(|url| {
                let url_str = url.as_str();
                if url_str.starts_with("https://web.whatsapp.com") {
                    true
                } else {
                    let _ = tauri_plugin_opener::open_url(url_str, None::<&str>);
                    false
                }
            })
            .on_new_window(|url, _features| {
                let _ = tauri_plugin_opener::open_url(url.as_str(), None::<&str>);
                tauri::webview::NewWindowResponse::Deny
            })
            .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
