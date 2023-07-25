use tonic::transport::Channel;

use crate::meesign::proto::{mpc_client::MpcClient, GroupsRequest, KeyType};
use std::error::Error;

mod proto {
    tonic::include_proto!("meesign");
}
struct Meesign {
    client: MpcClient<Channel>,
}

impl Meesign {
    // TODO: custom error handling
    pub async fn new(server_url: String) -> Result<Self, Box<dyn Error>> {
        let mut client = MpcClient::connect(server_url).await?;

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
}
