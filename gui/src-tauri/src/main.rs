// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use actix_web::{App, HttpServer};
use controller::endpoints::communicator_url::get_communicator_url;
use system_tray::{create_tray_menu, system_tray_event_handler};
use tauri::{GlobalWindowEvent, SystemTray, WindowEvent};

mod controller;
mod system_tray;

/// Handles window events, such as clicks outside the window
fn window_event_handler(event: GlobalWindowEvent) {
    match event.event() {
        WindowEvent::Focused(is_focused) => {
            if !is_focused {
                // event.window().hide().unwrap();
            }
        }
        _ => {}
    }
}
fn main() {
    env_logger::init();
    tauri::Builder::default()
        .setup(|_app| {
            tauri::async_runtime::spawn(
                HttpServer::new(|| App::new().service(get_communicator_url))
                    .bind(("127.0.0.1", 12345))?
                    .run(),
            );
            Ok(())
        })
        .plugin(tauri_plugin_positioner::init())
        .system_tray(SystemTray::new().with_menu(create_tray_menu()))
        .on_system_tray_event(system_tray_event_handler)
        .on_window_event(window_event_handler)
        .run(tauri::generate_context!())
        .expect("Couldn't run application");
}
