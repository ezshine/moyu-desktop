// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu,SystemTrayEvent};
use webbrowser;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let tray_menu = SystemTrayMenu::new()
                                   .add_item(CustomMenuItem::new("website".to_string(), "Website"))
                                   .add_item(CustomMenuItem::new("twitter".to_string(), "Twitter"))
                                   .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));
    let system_tray = SystemTray::new()
                                 .with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|_app, event| {
            match event {
                SystemTrayEvent::MenuItemClick { id, .. } => {
                    match id.as_str() {
                        "quit" => {
                            std::process::exit(0);
                        }
                        "website" => {
                            if let Err(e) = webbrowser::open("https://desktop.dashu.ai") {
                                eprintln!("Failed to open browser: {}", e);
                            }
                        }
                        "twitter" => {
                            if let Err(e) = webbrowser::open("https://x.com/@ezshine") {
                                eprintln!("Failed to open browser: {}", e);
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
