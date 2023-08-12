use std::error::Error;

use super::{AuthResponse, Communicator, Group};
use p256::ecdsa::{
    signature::{Signer, Verifier},
    Signature, SigningKey, VerifyingKey,
};
use rand::rngs::OsRng;
use tonic::async_trait;

pub(crate) struct MockedMeesign {
    group_name: String,
    group_key: Vec<u8>,
    verifying_key: VerifyingKey,
    private_key: SigningKey,
    signature: Option<Vec<u8>>,
}

impl MockedMeesign {
    pub(crate) fn new(group_name: String) -> Self {
        let private_key = SigningKey::random(&mut OsRng);
        let verifying_key = VerifyingKey::from(&private_key);
        let public_key = verifying_key.to_encoded_point(false).to_bytes().into();
        Self {
            group_name,
            private_key,
            group_key: public_key,
            signature: None,
            verifying_key,
        }
    }
}

#[async_trait]
impl Communicator for MockedMeesign {
    async fn get_groups(&mut self) -> Result<Vec<Group>, Box<dyn Error>> {
        Ok(vec![Group::new(
            self.group_key.clone(),
            self.group_name.clone(),
        )])
    }

    async fn send_auth_request(
        &mut self,
        _group_id: Vec<u8>,
        data: Vec<u8>,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let signature: Signature = self.private_key.sign(&data);
        self.signature = Some(signature.to_bytes().to_vec());
        self.verifying_key.verify(&data, &signature).unwrap();
        Ok(vec![])
    }

    async fn get_auth_response(
        &mut self,
        _task_id: Vec<u8>,
    ) -> Result<Option<AuthResponse>, Box<dyn Error>> {
        Ok(self.signature.clone())
    }
}
