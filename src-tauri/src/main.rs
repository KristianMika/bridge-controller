// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

mod system_tray;
use system_tray::{create_tray_menu, system_tray_event_handler};
use tauri::SystemTray;

fn main() {
    env_logger::init();
    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(create_tray_menu()))
        .on_system_tray_event(system_tray_event_handler)
        .run(tauri::generate_context!())
        .expect("Couldn't run application");
}
