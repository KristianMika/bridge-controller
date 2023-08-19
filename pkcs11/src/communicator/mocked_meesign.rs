use std::error::Error;

use super::{AuthResponse, ByteVector, Communicator, Group, GroupId, RequestData, TaskId};
use p256::ecdsa::{
    signature::hazmat::{PrehashSigner, PrehashVerifier},
    SigningKey, VerifyingKey,
};
use rand::rngs::OsRng;
use tonic::async_trait;

type GroupPublicKey = ByteVector;

pub(crate) struct MockedMeesign {
    group_name: String,
    group_public_key: GroupPublicKey,
    private_key: SigningKey,
    signature: Option<AuthResponse>,
}

impl MockedMeesign {
    pub(crate) fn new(group_name: String) -> Self {
        let private_key = SigningKey::random(&mut OsRng);
        let verifying_key = VerifyingKey::from(&private_key);
        let group_public_key = verifying_key.to_encoded_point(false).as_bytes().into();
        Self {
            group_name,
            private_key,
            group_public_key,
            signature: None,
        }
    }
}

#[async_trait]
impl Communicator for MockedMeesign {
    async fn get_groups(&mut self) -> Result<Vec<Group>, Box<dyn Error>> {
        Ok(vec![Group::new(
            self.group_public_key.clone(),
            self.group_name.clone(),
        )])
    }

    async fn send_auth_request(
        &mut self,
        _group_id: GroupId,
        data: RequestData,
    ) -> Result<TaskId, Box<dyn Error>> {
        let (signature, _) = self.private_key.sign_prehash(&data)?;
        self.signature = Some(signature.to_vec());
        Ok(vec![])
    }

    async fn get_auth_response(
        &mut self,
        _task_id: TaskId,
    ) -> Result<Option<AuthResponse>, Box<dyn Error>> {
        Ok(self.signature.clone())
    }
}
