// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::Error;
use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpServer};
use controller::{
    controller_repo::sled_controller_repo::SledControllerRepo,
    endpoints::{
        communicator_certificate_path::get_communicator_certificate_path,
        communicator_url::get_communicator_url, interface_configuration::get_configuration,
    },
    interface_configuration::InternalInterfaceConfiguration,
    state::State as ControllerState,
};
use filesystem::FileSystem;
use hex::ToHex;
use proto::Group as ProtoGroup;
use serde::{Deserialize, Serialize};
use specta::{collect_types, Type};
use state::State;
use system_tray::{create_tray_menu, system_tray_event_handler};
use tauri::async_runtime::JoinHandle;
use tauri::{generate_handler, GlobalWindowEvent, SystemTray, WindowEvent};
use tauri_specta::ts;

use crate::commands::get_groups::get_groups;
use crate::commands::{communicator_url::*, get_groups::*, interface_configuration::*};

mod commands;
mod controller;
mod filesystem;
mod interface;
mod state;
mod system_tray;

mod proto {
    tonic::include_proto!("meesign");
}

static CONTROLLER_PORT: u16 = 12345; // TODO
static SLED_DB_FILENAME: &str = "controller.sled";

#[derive(Type, Serialize)]
struct Group {
    name: String,
    group_id: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Type)]
pub(crate) struct FrontEndInterfaceConfiguration {
    isEnabled: bool,
    communicatorUrl: String,
    selectedGroup: String,
}

impl From<InternalInterfaceConfiguration> for FrontEndInterfaceConfiguration {
    fn from(value: InternalInterfaceConfiguration) -> Self {
        Self {
            isEnabled: value.is_enabled(),
            communicatorUrl: value.get_communicator_url().into(),
            selectedGroup: format!("0x{}", value.get_group_id().encode_hex_upper::<String>()),
        }
    }
}

impl From<ProtoGroup> for Group {
    fn from(value: ProtoGroup) -> Self {
        Self {
            name: value.name,
            group_id: format!("0x{}", value.identifier.encode_hex_upper::<String>()),
        }
    }
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

fn spawn_controller_server(
    wrapped_controller_state: Arc<Mutex<ControllerState>>,
    port: u16,
) -> JoinHandle<Result<(), Error>> {
    tauri::async_runtime::spawn(
        HttpServer::new(move || {
            let controller_state = wrapped_controller_state.as_ref().lock().unwrap().clone();
            App::new()
                .app_data(web::Data::new(controller_state))
                .service(get_communicator_url)
                .service(get_configuration)
                .service(get_communicator_certificate_path)
        })
        .bind(("127.0.0.1", port))
        .unwrap()
        .run(),
    )
}

fn main() {
    #[cfg(debug_assertions)]
    ts::export(
        collect_types![
            set_interface_configuration,
            get_interface_configuration,
            get_groups,
            set_communicator_certificate_path
        ],
        "../src/bindings.ts",
    )
    .unwrap();

    let filesystem = FileSystem {};
    filesystem
        .ensure_controller_directory_structure_exists()
        .expect("Couldn't create controller directory structure");
    let sled_filepath = filesystem.get_db_filepath(SLED_DB_FILENAME).unwrap();
    let db = sled::open(sled_filepath).unwrap();
    let controller_repo = SledControllerRepo::new(Arc::new(Mutex::new(db)));
    let tauri_state = State::new(Box::new(controller_repo.clone()), filesystem.clone());
    let controller_state = ControllerState::new(Arc::new(controller_repo), filesystem);
    // wrapped just so the the closure can take ownership of it multiple times
    let wrapped_controller_state = Arc::new(Mutex::new(controller_state));

    env_logger::init();
    tauri::Builder::default()
        .setup(|_app| {
            spawn_controller_server(wrapped_controller_state, CONTROLLER_PORT);
            Ok(())
        })
        .manage(tauri_state)
        .invoke_handler(generate_handler![
            set_interface_configuration,
            get_interface_configuration,
            get_groups,
            set_communicator_certificate_path
        ])
        .plugin(tauri_plugin_positioner::init())
        .system_tray(SystemTray::new().with_menu(create_tray_menu()))
        .on_system_tray_event(system_tray_event_handler)
        .on_window_event(window_event_handler)
        .run(tauri::generate_context!())
        .expect("Couldn't run application");
}
