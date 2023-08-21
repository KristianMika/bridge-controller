// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

use actix_web::{http::Uri, web, App, HttpServer};
use controller::{
    controller_repo::sled_controller_repo::SledControllerRepo,
    endpoints::communicator_url::get_communicator_url,
    interface_configuration::InterfaceConfiguration, state::State as ControllerState,
};
use hex::ToHex;
use interface::CryptographicInterface;
use proto::{mpc_client::MpcClient, Group as ProtoGroup, GroupsRequest, KeyType};
use serde::Serialize;
use specta::{collect_types, Type};
use state::State;
use system_tray::{create_tray_menu, system_tray_event_handler};
use tauri::{generate_handler, GlobalWindowEvent, SystemTray, WindowEvent};
use tauri_specta::ts;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};

mod controller;
mod interface;
mod state;
mod system_tray;

mod proto {
    tonic::include_proto!("meesign");
}

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
#[derive(Type, Serialize)]
struct Group {
    name: String,
    group_id: String,
}

impl From<ProtoGroup> for Group {
    fn from(value: ProtoGroup) -> Self {
        Self {
            name: value.name,
            group_id: format!("0x{}", value.identifier.encode_hex_upper::<String>()),
        }
    }
}

#[tauri::command]
#[specta::specta]
async fn get_groups(controller_url: String) -> Result<Vec<Group>, String> {
    let cert = Certificate::from_pem(
        std::fs::read("/home/kiko/Desktop/tmp/meesign-server/keys/meesign-ca-cert.pem").unwrap(),
    );
    let server_uri = Uri::from_str(&format!("https://{}:{}", &controller_url, "1337")).unwrap();
    let client_tls_config = ClientTlsConfig::new()
        .domain_name(&controller_url)
        .ca_certificate(cert);
    let channel = Channel::builder(server_uri)
        .tls_config(client_tls_config)
        .unwrap()
        .connect()
        .await
        .unwrap();
    let mut client = MpcClient::new(channel);
    let request = tonic::Request::new(GroupsRequest { device_id: None });

    let response = client.get_groups(request).await.unwrap();
    let groups = &response.get_ref().groups;
    let groups = groups
        .into_iter()
        // TODO: update meesign server to filter groups?
        .filter(|group| group.key_type == KeyType::SignChallenge as i32)
        // TODO: don't clone
        .map(|group: &ProtoGroup| group.to_owned().into())
        .collect();
    Ok(groups)
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
        collect_types![
            set_interface_configuration,
            get_interface_configuration,
            get_groups
        ],
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
            get_interface_configuration,
            get_groups
        ])
        .plugin(tauri_plugin_positioner::init())
        .system_tray(SystemTray::new().with_menu(create_tray_menu()))
        .on_system_tray_event(system_tray_event_handler)
        .on_window_event(window_event_handler)
        .run(tauri::generate_context!())
        .expect("Couldn't run application");
}
