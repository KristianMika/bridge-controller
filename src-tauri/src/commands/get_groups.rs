use std::{error::Error, str::FromStr};

use actix_web::http::Uri;
use log::{debug, error};
use tonic::transport::{Certificate, Channel, ClientTlsConfig};

use crate::{
    group::Group,
    proto::{mpc_client::MpcClient, Group as ProtoGroup, GroupsRequest, KeyType},
    state::State,
};

static MEESIGN_GRPC_PORT: &str = "1337";

/// Fetches and filters authentication groups present on the specified communicator
///
/// # Arguments
///
/// * `communicator_url` - The url of the communicator.
/// * `state` - The state of the application.
#[tauri::command]
#[specta::specta]
pub(crate) async fn get_groups(
    communicator_url: String,
    state: tauri::State<'_, State>,
) -> Result<Vec<Group>, String> {
    // TODO: make sure we have the cert
    let certificate_path = state
        .get_filesystem()
        .get_certificate_filepath(&communicator_url)
        .map_err(|err| {
            error!("{err}");
            String::from("Could not get certificate file")
        })?;
    let certificate_contents = std::fs::read(certificate_path).map_err(|err| {
        error!("{err}");
        String::from("Could not read certificate file")
    })?;
    let certificate = Certificate::from_pem(certificate_contents);
    let server_uri = Uri::from_str(&format!(
        "https://{}:{}",
        &communicator_url, MEESIGN_GRPC_PORT
    ))
    .unwrap();
    let groups = get_authentication_groups(&communicator_url, certificate, server_uri)
        .await
        .map_err(|err| {
            error!("Couldn't get groups: {err}");
            String::from("Could not get groups")
        })?;
    debug!("Command get_groups for url {communicator_url:?} returning {groups:?}");
    Ok(groups)
}

async fn get_authentication_groups(
    communicator_url: &str,
    cert: Certificate,
    server_uri: Uri,
) -> Result<Vec<Group>, Box<dyn Error>> {
    // TODO: consider caching in the state so we don't create new instances of the client
    let client_tls_config = ClientTlsConfig::new()
        .domain_name(communicator_url)
        .ca_certificate(cert);
    let channel = Channel::builder(server_uri)
        .tls_config(client_tls_config)?
        .connect()
        .await?;
    let mut client = MpcClient::new(channel);
    let request = tonic::Request::new(GroupsRequest { device_id: None });

    let response = client.get_groups(request).await?;
    let groups = &response.get_ref().groups;
    let groups: Vec<Group> = groups
        .iter()
        // TODO: update meesign server to filter groups?
        .filter(|group| group.key_type == KeyType::SignChallenge as i32)
        .map(|group: &ProtoGroup| group.to_owned().into())
        .collect();

    Ok(groups)
}
