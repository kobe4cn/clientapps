use tauri::{
    webview::PageLoadPayload, App, Webview, WebviewUrl, WebviewWindowBuilder, Window, WindowEvent,
};
use tauri_plugin_log::{Target, TargetKind};
use tracing::info;
use utils::{app_dir, log_dir};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod utils;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_app_dir() -> String {
    app_dir().display().to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(logger().build())
        .invoke_handler(tauri::generate_handler![greet, get_app_dir])
        .setup(setup)
        .on_page_load(page_load_handler)
        .on_window_event(window_event_handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn page_load_handler(webview: &Webview, _payload: &PageLoadPayload) {
    info!("Page loaded: {}", webview.label());
}

fn window_event_handler(window: &Window, event: &WindowEvent) {
    info!("Window event: {:?}", event);
    if let WindowEvent::CloseRequested { api, .. } = event {
        if window.label() == "main" {
            api.prevent_close();
            window.hide().unwrap();
        }
    }
}

fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up the app");
    let handle = app.handle();
    #[cfg(desktop)]
    {
        handle.plugin(tauri_plugin_window_state::Builder::default().build())?;
    }

    let mut builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default());
    #[cfg(desktop)]
    {
        builder = builder
            .title("Hacker News")
            .user_agent(&format!("Hn app - {}", std::env::consts::OS))
            .inner_size(1200., 800.)
            .min_inner_size(800., 600.)
            .resizable(true)
            .content_protected(true);
    }
    let webview = builder.build()?;
    #[cfg(debug_assertions)]
    {
        webview.open_devtools();
    }

    Ok(())
}

fn logger() -> tauri_plugin_log::Builder {
    tauri_plugin_log::Builder::default()
        .targets([
            Target::new(TargetKind::Webview),
            Target::new(TargetKind::Folder {
                path: log_dir(),
                file_name: None,
            }),
            Target::new(TargetKind::Stdout),
        ])
        .level(tracing::log::LevelFilter::Info)
}
