// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpServer};
use controller::{
    controller_repo::sled_controller_repo::SledControllerRepo,
    endpoints::communicator_url::get_communicator_url,
    interface_configuration::InterfaceConfiguration, state::State as ControllerState,
};
use interface::CryptographicInterface;
use specta::collect_types;
use state::State;
use system_tray::{create_tray_menu, system_tray_event_handler};
use tauri::{generate_handler, GlobalWindowEvent, SystemTray, WindowEvent};
use tauri_specta::ts;

mod controller;
mod interface;
mod state;
mod system_tray;

#[tauri::command]
#[specta::specta]
async fn set_interface_configuration(
    state: tauri::State<'_, State>,
    cryptographic_interface: CryptographicInterface,
    configuration: InterfaceConfiguration,
) -> Result<(), String> {
    let repo = state.get_controller_repo();
    repo.set_interface_configuration(configuration, cryptographic_interface)
        .unwrap();
    Ok(())
}

#[tauri::command]
#[specta::specta]
async fn get_interface_configuration(
    state: tauri::State<'_, State>,
    cryptographic_interface: CryptographicInterface,
) -> Result<Option<InterfaceConfiguration>, String> {
    let repo = state.get_controller_repo();
    let configuration = repo
        .get_interface_configuration(&cryptographic_interface)
        .unwrap();
    Ok(configuration)
}

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
    #[cfg(debug_assertions)]
    ts::export(
        collect_types![set_interface_configuration, get_interface_configuration],
        "../src/bindings.ts",
    )
    .unwrap();

    let db = sled::open("/home/kiko/Desktop/controller.sled").unwrap();
    let controller_repo = SledControllerRepo::new(Arc::new(Mutex::new(db)));
    let tauri_state = State::new(Box::new(controller_repo.clone()));
    let controller_state = ControllerState::new(Arc::new(controller_repo));
    // wrapped just so the the closure can take ownership of it multiple times
    let wrapped_controller_state = Arc::new(Mutex::new(controller_state));

    env_logger::init();
    tauri::Builder::default()
        .setup(|_app| {
            tauri::async_runtime::spawn(
                HttpServer::new(move || {
                    let controller_state =
                        wrapped_controller_state.as_ref().lock().unwrap().clone();
                    App::new()
                        .app_data(web::Data::new(controller_state))
                        .service(get_communicator_url)
                })
                .bind(("127.0.0.1", 12345))?
                .run(),
            );
            Ok(())
        })
        .manage(tauri_state)
        .invoke_handler(generate_handler![
            set_interface_configuration,
            get_interface_configuration
        ])
        .plugin(tauri_plugin_positioner::init())
        .system_tray(SystemTray::new().with_menu(create_tray_menu()))
        .on_system_tray_event(system_tray_event_handler)
        .on_window_event(window_event_handler)
        .run(tauri::generate_context!())
        .expect("Couldn't run application");
}
