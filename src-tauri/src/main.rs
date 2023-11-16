// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod proto {
    tonic::include_proto!("meesign");
}

mod commands;
mod controller;
mod filesystem;
pub(crate) mod group;
mod interface;
mod process;
mod state;
mod system_tray;

use std::error::Error;
use std::io;
#[cfg(debug_assertions)]
use std::path::Path;
use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpServer};
use controller::{
    controller_repo::{sled_controller_repo::SledControllerRepo, ControllerRepo},
    endpoints::{get_communicator_certificate_path, get_configuration},
    state::State as ControllerState,
};
use env_logger::Target;
use filesystem::FileSystem;
use log::info;
use process::spawn_enabled_interfaces;
#[cfg(debug_assertions)]
use specta::{collect_types, ts::TsExportError};
use state::State;
use system_tray::{create_tray_menu, system_tray_event_handler, window_event_handler};
use tauri::async_runtime::JoinHandle;
use tauri::{generate_handler, SystemTray};
#[cfg(debug_assertions)]
use tauri_specta::ts;

use crate::commands::process_management::kill_interface_process;
use crate::commands::{
    certificates::*, get_groups::*, interface_configuration::*, process_management::*,
    tool_configurations::*,
};
use crate::process::process_executor::{PlatformSpecificProcessExecutor, ProcessExecutor};
use crate::process::process_manager::ProcessManager;

static CONTROLLER_PORT: u16 = 11115;
static SLED_DB_FILENAME: &str = "controller.sled";

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
                .service(get_configuration)
                .service(get_communicator_certificate_path)
        })
        .bind(("127.0.0.1", port))
        .expect("Couldn't bind controller server to port")
        .run(),
    )
}

/// Returns a logger target for debug build, which is the standard error output
#[cfg(debug_assertions)]
fn get_logger_target(_filesystem: &FileSystem) -> Result<Target, Box<dyn Error>> {
    Ok(Target::Stderr)
}

/// Returns a logger target for release build, which is a pipe to the log file
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

/// Generates typescript bindings for better and safer front-end back-end integration
///
/// # Arguments
///
/// * `bindings_filename` - The name of the file to which the bindings will be written
#[cfg(debug_assertions)]
fn generate_typescript_bindings(bindings_filename: &str) -> Result<(), TsExportError> {
    let current_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let bindings_path = current_dir.join("..").join("src").join(bindings_filename);
    info!("Generating typescript bindings at {:?}", bindings_path);
    ts::export(
        collect_types![
            set_interface_configuration,
            get_interface_configuration,
            remove_interface_configuration,
            get_groups,
            store_communicator_certificate,
            spawn_interface_process,
            kill_interface_process,
            is_interface_process_running,
            is_certificate_present,
            get_configured_tools
        ],
        bindings_path,
    )
}

fn main() {
    let filesystem = FileSystem {};
    filesystem
        .ensure_controller_directory_structure_exists()
        .expect("Couldn't create controller directory structure");

    init_logger(&filesystem).expect("Couldn't initialize logger");

    #[cfg(debug_assertions)]
    generate_typescript_bindings("bindings.ts").expect("Couldn't export bindings");

    let sled_filepath = filesystem
        .get_db_filepath(SLED_DB_FILENAME)
        .expect("Couldn't get DB path");
    let db = sled::open(sled_filepath).expect("Can't open sled DB");
    let controller_repo = SledControllerRepo::new(Arc::new(Mutex::new(db)));
    let process_executor = PlatformSpecificProcessExecutor::new();
    let process_manager = ProcessManager::new(process_executor);
    let process_manager = Arc::new(process_manager);

    let repo_arc: Arc<dyn ControllerRepo> = Arc::new(controller_repo);
    let tauri_state = State::new(
        repo_arc.clone(),
        filesystem.clone(),
        process_manager.clone(),
    );
    let controller_state = ControllerState::new(repo_arc.clone(), filesystem);
    // wrapped just so the the closure can take ownership of it multiple times
    let wrapped_controller_state = Arc::new(Mutex::new(controller_state));

    tauri::Builder::default()
        .setup(move |_app| {
            spawn_controller_server(wrapped_controller_state, CONTROLLER_PORT);
            let _ = spawn_enabled_interfaces(&repo_arc, &process_manager);
            Ok(())
        })
        .manage(tauri_state)
        .invoke_handler(generate_handler![
            set_interface_configuration,
            get_interface_configuration,
            remove_interface_configuration,
            get_groups,
            store_communicator_certificate,
            spawn_interface_process,
            kill_interface_process,
            is_interface_process_running,
            is_certificate_present,
            get_configured_tools
        ])
        .plugin(tauri_plugin_positioner::init())
        .system_tray(SystemTray::new().with_menu(create_tray_menu()))
        .on_system_tray_event(system_tray_event_handler)
        .on_window_event(window_event_handler)
        .run(tauri::generate_context!())
        .expect("Couldn't run application");
}
