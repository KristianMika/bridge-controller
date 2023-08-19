use tokio::time;
use tonic::{
    async_trait,
    transport::{Certificate, Channel, ClientTlsConfig, Uri},
};

use crate::communicator::meesign::proto::{mpc_client::MpcClient, GroupsRequest, KeyType};
use std::{str::FromStr, time::Duration};

use self::proto::{task::TaskState, SignRequest, TaskRequest};
use super::{
    communicator_error::CommunicatorError, group::Group, Communicator, GroupId, RequestData, TaskId,
};
use crate::communicator::AuthResponse;

mod proto {
    tonic::include_proto!("meesign");
}
pub(crate) struct Meesign {
    client: MpcClient<Channel>,
}

static MAX_ATTEMPT_COUNT: usize = 10;
static ATTEMPT_SLEEP_SEC: u64 = 5;

impl Meesign {
    pub async fn new(
        hostname: String,
        port: u32,
        certificate: Certificate,
    ) -> Result<Self, CommunicatorError> {
        let server_uri = Uri::from_str(&format!("https://{}:{}", &hostname, port.to_string()))?;
        let client_tls_config = ClientTlsConfig::new()
            .domain_name(hostname)
            .ca_certificate(certificate);
        let channel = Channel::builder(server_uri)
            .tls_config(client_tls_config)?
            .connect()
            .await?;
        let client = MpcClient::new(channel);
        Ok(Self { client })
    }
}
#[async_trait]
impl Communicator for Meesign {
    async fn get_groups(&mut self) -> Result<Vec<Group>, CommunicatorError> {
        let request = tonic::Request::new(GroupsRequest { device_id: None });

        let response = self.client.get_groups(request).await?;
        let groups = &response.get_ref().groups;
        let groups = groups
            .iter()
            .filter(|group| group.key_type == KeyType::SignChallenge.into())
            .map(|group| Group::new(group.identifier.clone().into(), group.name.clone().into()))
            .collect();
        Ok(groups)
    }

    async fn send_auth_request(
        &mut self,
        group_id: GroupId,
        data: RequestData,
    ) -> Result<TaskId, CommunicatorError> {
        let request = tonic::Request::new(SignRequest {
            name: "PKCS#11 auth request".into(),
            group_id,
            data,
        });
        let response = self.client.sign(request).await?;

        Ok(response.get_ref().id.clone())
    }

    async fn get_auth_response(
        &mut self,
        task_id: TaskId,
    ) -> Result<Option<AuthResponse>, CommunicatorError> {
        for _attempt in 0..MAX_ATTEMPT_COUNT {
            let request = tonic::Request::new(TaskRequest {
                task_id: task_id.clone(),
                device_id: None,
            });
            let response = self.client.get_task(request).await?;
            if response.get_ref().state == TaskState::Finished as i32 {
                return Ok(response.get_ref().data.to_owned());
            }
            if response.get_ref().state == TaskState::Failed as i32 {
                return Err(CommunicatorError::TaskFailedError);
            }
            time::sleep(Duration::from_secs(ATTEMPT_SLEEP_SEC)).await;
        }

        Err(CommunicatorError::TaskTimedOutError(
            (MAX_ATTEMPT_COUNT as u64) * ATTEMPT_SLEEP_SEC,
        ))
    }
}
