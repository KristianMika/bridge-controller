use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use serde::Serialize;

use crate::{
    controller::interface_configuration::InternalInterfaceConfiguration,
    interface::CryptographicInterface,
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

impl SledControllerRepo {
    fn store_configured_tools(
        &self,
        tools: &HashSet<Option<String>>,
        interface: &CryptographicInterface,
    ) -> Result<(), ControllerRepoError> {
        let key: Vec<u8> = bincode::serialize(interface)?;
        let value: Vec<u8> = bincode::serialize(tools)?;
        self.db.lock()?.insert(key, value)?;
        Ok(())
    }

    fn add_configured_tool_entry(
        &self,
        tool: Option<String>,
        interface: &CryptographicInterface,
    ) -> Result<(), ControllerRepoError> {
        let key: Vec<u8> = bincode::serialize(interface)?;

        let Some(tools) = self.db.lock()?.get(key)? else {
            let mut tools: HashSet<Option<String>> = HashSet::new();
            tools.insert(tool);
            return self.store_configured_tools(&tools, interface);
        };
        let mut tools: HashSet<Option<String>> = bincode::deserialize(&tools)?;
        if tools.insert(tool) {
            self.store_configured_tools(&tools, interface)?;
        }
        Ok(())
    }

    fn remove_configured_tool_entry(
        &self,
        tool: &Option<String>,
        interface: &CryptographicInterface,
    ) -> Result<(), ControllerRepoError> {
        let key: Vec<u8> = bincode::serialize(interface)?;

        let Some(tools) = self.db.lock()?.get(key)? else {
            // this is an inconsistency
            return Ok(());
        };
        let mut tools: HashSet<Option<String>> = bincode::deserialize(&tools)?;
        if tools.remove(tool) {
            self.store_configured_tools(&tools, interface)?;
        }
        Ok(())
    }
}

// TODO: optimize using the provided example: https://github.com/spacejam/sled/blob/main/examples/structured.rs
impl ControllerRepo for SledControllerRepo {
    fn set_interface_configuration(
        &self,
        configuration: InternalInterfaceConfiguration,
        interface: CryptographicInterface,
        tool: Option<String>,
    ) -> Result<(), ControllerRepoError> {
        let key = ConfigurationKey::new(interface.clone(), tool.clone());
        let key: Vec<u8> = bincode::serialize(&key)?;
        let value: Vec<u8> = bincode::serialize(&configuration)?;
        self.db.lock()?.insert(key, value)?;
        self.add_configured_tool_entry(tool, &interface)?;
        Ok(())
    }

    fn get_interface_configuration(
        &self,
        interface: &CryptographicInterface,
        tool: &Option<String>,
    ) -> Result<Option<InternalInterfaceConfiguration>, ControllerRepoError> {
        let key = ConfigurationKey::new(interface.clone(), tool.clone());
        let key: Vec<u8> = bincode::serialize(&key)?;
        let Some(value) = self.db.lock()?.get(key)? else {
            return Ok(None);
        };
        let configuration: InternalInterfaceConfiguration = bincode::deserialize(&value[..])?;
        Ok(Some(configuration))
    }

    fn get_configured_tools(
        &self,
        interface: &CryptographicInterface,
    ) -> Result<Vec<Option<String>>, ControllerRepoError> {
        let key: Vec<u8> = bincode::serialize(interface)?;
        let Some(configured_tools) = self.db.lock()?.get(key)? else {
            return Ok(vec![]);
        };
        let configured_tools: Vec<Option<String>> = bincode::deserialize(&configured_tools[..])?;
        Ok(configured_tools)
    }

    fn remove_interface_configuration(
        &self,
        interface: &CryptographicInterface,
        tool: &Option<String>,
    ) -> Result<(), ControllerRepoError> {
        let key = ConfigurationKey::new(interface.clone(), tool.clone());
        let key: Vec<u8> = bincode::serialize(&key)?;
        self.db.lock()?.remove(key)?;
        self.remove_configured_tool_entry(tool, interface)?;
        Ok(())
    }
}

#[derive(Serialize)]
struct ConfigurationKey {
    interface: CryptographicInterface,
    tool: Option<String>,
}

impl ConfigurationKey {
    fn new(interface: CryptographicInterface, tool: Option<String>) -> Self {
        Self { interface, tool }
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use super::*;
    use crate::controller::interface_configuration::InternalInterfaceConfiguration;

    fn init_controller_repo() -> Result<SledControllerRepo, Box<dyn Error>> {
        let db = Arc::new(Mutex::new(sled::Config::new().temporary(true).open()?));
        let repo = SledControllerRepo::new(db);
        Ok(repo)
    }
    #[test]
    fn test_sled_controller_repo() -> Result<(), Box<dyn Error>> {
        let repo = init_controller_repo()?;
        let interface = CryptographicInterface::Cryptoki;
        let communicator_url = "meesign.crocs.fi.muni.cz".into();
        let group_id = vec![1, 2, 3, 4, 5];
        let is_enabled = false;
        let configuration =
            InternalInterfaceConfiguration::new(communicator_url, group_id, is_enabled);

        repo.set_interface_configuration(configuration.clone(), interface.clone(), None)?;
        repo.set_interface_configuration(configuration, interface.clone(), Some("ssh".into()))?;

        let mut configured_tools = repo.get_configured_tools(&interface)?;
        configured_tools.sort();
        let mut expected_tools = vec![None, Some(String::from("ssh"))];
        expected_tools.sort();

        assert_eq!(configured_tools, expected_tools);

        repo.remove_configured_tool_entry(&Some("ssh".into()), &interface)?;
        let configured_tools = repo.get_configured_tools(&interface)?;

        assert_eq!(configured_tools, vec![None]);

        Ok(())
    }
}
