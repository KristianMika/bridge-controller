use std::sync::{Arc, Mutex};

use crate::{
    controller::interface_configuration::InterfaceConfiguration, interface::CryptographicInterface,
};

use super::{controller_repo_error::ControllerRepoError, ControllerRepo};

#[derive(Clone)]
pub(crate) struct SledControllerRepo {
    db: Arc<Mutex<sled::Db>>,
}

impl SledControllerRepo {
    pub fn new(db: Arc<Mutex<sled::Db>>) -> Self {
        Self { db }
    }
}

// TODO: optimize using the provided example: https://github.com/spacejam/sled/blob/main/examples/structured.rs
impl ControllerRepo for SledControllerRepo {
    fn set_interface_configuration(
        &self,
        configuration: InterfaceConfiguration,
        interface: CryptographicInterface,
    ) -> Result<(), ControllerRepoError> {
        let key: Vec<u8> = bincode::serialize(&interface)?;
        let value: Vec<u8> = bincode::serialize(&configuration)?;
        self.db.lock()?.insert(&key, value)?;
        Ok(())
    }

    fn get_interface_configuration(
        &self,
        interface: &CryptographicInterface,
    ) -> Result<Option<InterfaceConfiguration>, ControllerRepoError> {
        let key: Vec<u8> = bincode::serialize(interface)?;
        let Some(value) = self.db.lock()?.get(&key)? else {
            return Ok(None);
        };
        let configuration: InterfaceConfiguration = bincode::deserialize(&value[..])?;
        Ok(Some(configuration))
    }
}
