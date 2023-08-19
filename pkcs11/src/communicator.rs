use tonic::async_trait;

use self::{communicator_error::CommunicatorError, group::Group};

pub(crate) mod communicator_error;
pub(crate) mod group;
pub(crate) mod meesign;
#[cfg(feature = "mocked_meesign")]
pub(crate) mod mocked_meesign;

type ByteVector = Vec<u8>;
pub(crate) type AuthResponse = ByteVector;
pub(crate) type GroupId = ByteVector;
pub(crate) type TaskId = ByteVector;
pub(crate) type RequestData = ByteVector;

// TODO: remove once rust 1.74 is released
#[async_trait]
pub(crate) trait Communicator: Send + Sync {
    async fn get_groups(&mut self) -> Result<Vec<Group>, CommunicatorError>;

    async fn send_auth_request(
        &mut self,
        group_id: GroupId,
        data: RequestData,
    ) -> Result<TaskId, CommunicatorError>;

    async fn get_auth_response(
        &mut self,
        task_id: TaskId,
    ) -> Result<Option<AuthResponse>, CommunicatorError>;
}
