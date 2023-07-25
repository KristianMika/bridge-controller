use tokio::{task, time};
use tonic::transport::Channel;

use crate::meesign::proto::{mpc_client::MpcClient, GroupsRequest, KeyType};
use std::{error::Error, time::Duration};

use self::proto::{task::TaskState, SignRequest, TaskRequest};

mod proto {
    tonic::include_proto!("meesign");
}
struct Meesign {
    client: MpcClient<Channel>,
}

static MAX_ATTEMPT_COUNT: usize = 10;
static ATTEMPT_SLEEP_SEC: u64 = 5;
type AuthResponse = Vec<u8>;
impl Meesign {
    // TODO: custom error handling
    pub async fn new(server_url: String) -> Result<Self, Box<dyn Error>> {
        let client = MpcClient::connect(server_url).await?;

        Ok(Self { client })
    }

    pub async fn get_groups(&mut self) -> Result<Vec<String>, Box<dyn Error>> {
        let request = tonic::Request::new(GroupsRequest { device_id: None });

        let response = self.client.get_groups(request).await?;
        let groups = &response.get_ref().groups;
        let groups = groups
            .iter()
            .filter(|group| group.key_type == KeyType::SignChallenge.into())
            .map(|group| hex::encode(&group.identifier))
            .collect();
        Ok(groups)
    }

    pub async fn send_auth_request(
        &mut self,
        group_id: Vec<u8>,
        data: Vec<u8>,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let request = tonic::Request::new(SignRequest {
            name: "PKCS#11 auth request".into(),
            group_id,
            data,
        });
        let response = self.client.sign(request).await?;

        Ok(response.get_ref().id.clone())
    }

    pub async fn get_auth_response(
        &mut self,
        task_id: Vec<u8>,
    ) -> Result<Option<AuthResponse>, Box<dyn Error>> {
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
                return Ok(None); // TODO: custom error enum
            }
            time::sleep(Duration::from_secs(ATTEMPT_SLEEP_SEC)).await;
        }

        Ok(None)
    }
}
