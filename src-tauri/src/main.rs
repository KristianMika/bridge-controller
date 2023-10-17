// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io;
#[cfg(debug_assertions)]
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::{error::Error, io::ErrorKind};

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
use env_logger::Target;
use filesystem::FileSystem;
use hex::ToHex;
use log::{error, info};
use process_manager::ProcessManager;
use proto::Group as ProtoGroup;
use serde::{Deserialize, Serialize};
use specta::Type;
#[cfg(debug_assertions)]
use specta::{collect_types, ts::TsExportError};
use state::State;
use system_tray::{create_tray_menu, system_tray_event_handler, window_event_handler};
use tauri::async_runtime::JoinHandle;
use tauri::{generate_handler, SystemTray};
#[cfg(debug_assertions)]
use tauri_specta::ts;

use crate::commands::get_groups::get_groups;
use crate::commands::process_management::kill_interface_process;
use crate::commands::{
    communicator_url::*, get_groups::*, interface_configuration::*, process_management::*,
};
use crate::process_manager::{PlatformSpecificProcessExecutor, ProcessExecutor};

mod commands;
mod controller;
mod filesystem;
mod interface;
mod process_manager;
mod state;
mod system_tray;

mod proto {
    tonic::include_proto!("meesign");
}

static CONTROLLER_PORT: u16 = 11115;
static SLED_DB_FILENAME: &str = "controller.sled";

#[derive(Type, Serialize)]
struct Group {
    name: String,
    group_id: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Type, Debug)]
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

fn spawn_controller_server(
    wrapped_controller_state: Arc<Mutex<ControllerState>>,
    port: u16,
) -> JoinHandle<Result<(), io::Error>> {
    info!("Spawning controller server on port {}...", port);
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
        .expect("Couldn't bind controller server to port")
        .run(),
    )
}

#[cfg(debug_assertions)]
fn get_logger_target(_filesystem: &FileSystem) -> Result<Target, Box<dyn Error>> {
    Ok(Target::Stderr)
}

#[cfg(not(debug_assertions))]
fn get_logger_target(filesystem: &FileSystem) -> Result<Target, Box<dyn Error>> {
    let log_file = filesystem.get_log_file()?;
    Ok(Target::Pipe(Box::new(log_file)))
}

fn init_logger(filesystem: &FileSystem) -> Result<(), Box<dyn Error>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .target(get_logger_target(filesystem)?)
        .init();
    Ok(())
}
fn main() {
    let filesystem = FileSystem {};
    filesystem
        .ensure_controller_directory_structure_exists()
        .expect("Couldn't create controller directory structure");

    init_logger(&filesystem).expect("Couldn't initialize logger");

    #[cfg(debug_assertions)]
    generate_typescript_bindings("bindings.ts").expect("Couldn't export bindings");

    let sled_filepath = filesystem.get_db_filepath(SLED_DB_FILENAME).unwrap();
    let db = match sled::open(sled_filepath) {
        Ok(db) => db,
        Err(err) => {
            if err.kind() == ErrorKind::WouldBlock {
                eprintln!("There seems to be another instance already running. Exitting...");
                return;
            }
            error!("Failed opening sled DB: {:?}. Panicking...", err);
            panic!("Couldn't open sled database");
        }
    };
    let controller_repo = SledControllerRepo::new(Arc::new(Mutex::new(db)));
    let process_executor = PlatformSpecificProcessExecutor::new();
    let process_manager = ProcessManager::new(process_executor);
    let tauri_state = State::new(
        Box::new(controller_repo.clone()),
        filesystem.clone(),
        Arc::new(process_manager),
    );
    let controller_state = ControllerState::new(Arc::new(controller_repo), filesystem);
    // wrapped just so the the closure can take ownership of it multiple times
    let wrapped_controller_state = Arc::new(Mutex::new(controller_state));

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
            set_communicator_certificate_path,
            spawn_interface_process,
            kill_interface_process
        ])
        .plugin(tauri_plugin_positioner::init())
        .system_tray(SystemTray::new().with_menu(create_tray_menu()))
        .on_system_tray_event(system_tray_event_handler)
        .on_window_event(window_event_handler)
        .run(tauri::generate_context!())
        .expect("Couldn't run application");
}

#[cfg(debug_assertions)]
fn generate_typescript_bindings(bindings_filename: &str) -> Result<(), TsExportError> {
    let current_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let bindings_path = current_dir.join("..").join("src").join(bindings_filename);
    info!("Generating typescript bindings at {:?}", bindings_path);
    ts::export(
        collect_types![
            set_interface_configuration,
            get_interface_configuration,
            get_groups,
            set_communicator_certificate_path,
            spawn_interface_process,
            kill_interface_process
        ],
        bindings_path,
    )
}
