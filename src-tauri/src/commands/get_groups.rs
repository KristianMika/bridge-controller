use std::str::FromStr;

use actix_web::http::Uri;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};

use crate::{
    proto::{mpc_client::MpcClient, Group as ProtoGroup, GroupsRequest, KeyType},
    Group,
};

#[tauri::command]
#[specta::specta]
pub(crate) async fn get_groups(controller_url: String) -> Result<Vec<Group>, String> {
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
