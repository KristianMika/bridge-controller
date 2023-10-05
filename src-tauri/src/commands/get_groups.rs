use std::str::FromStr;

use actix_web::http::Uri;
use log::debug;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};

use crate::{
    proto::{mpc_client::MpcClient, Group as ProtoGroup, GroupsRequest, KeyType},
    state::State,
    Group,
};

static MEESIGN_GRPC_PORT: &str = "1337";

#[tauri::command]
#[specta::specta]
pub(crate) async fn get_groups(
    communicator_url: String,
    state: tauri::State<'_, State>,
) -> Result<Vec<Group>, String> {
    debug!("A command for getting groups for communicator {communicator_url} has been invoked");
    // TODO: consider storing into db as well
    // TODO: make sure we have the cert
    let certificate_path = state
        .get_filesystem()
        .get_certificate_filepath(&communicator_url)
        .unwrap();
    let cert = Certificate::from_pem(std::fs::read(certificate_path).unwrap());
    let server_uri = Uri::from_str(&format!(
        "https://{}:{}",
        &communicator_url, MEESIGN_GRPC_PORT
    ))
    .unwrap();
    let client_tls_config = ClientTlsConfig::new()
        .domain_name(&communicator_url)
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
