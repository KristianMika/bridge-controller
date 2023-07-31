use std::error::Error;

use tonic::async_trait;

pub(crate) mod meesign;

pub(crate) type AuthResponse = Vec<u8>;
pub(crate) type GroupId = Vec<u8>;

// TODO: remove once rust 1.74 is released
#[async_trait]
pub(crate) trait Communicator {
    async fn get_groups(&mut self) -> Result<Vec<GroupId>, Box<dyn Error>>;

    async fn send_auth_request(
        &mut self,
        group_id: Vec<u8>,
        data: Vec<u8>,
    ) -> Result<Vec<u8>, Box<dyn Error>>;

    async fn get_auth_response(
        &mut self,
        task_id: Vec<u8>,
    ) -> Result<Option<AuthResponse>, Box<dyn Error>>;
}
