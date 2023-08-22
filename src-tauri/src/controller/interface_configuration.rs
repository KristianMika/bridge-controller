use serde::{Deserialize, Serialize};
use specta::Type;

use crate::FrontEndInterfaceConfiguration;

type ByteVector = Vec<u8>;
pub(crate) type GroupId = ByteVector;

#[derive(Serialize, Deserialize, Debug, Type)]

pub(crate) struct InterfaceConfiguration {
    controller_url: String,
    group_id: GroupId,
    is_enabled: bool,
}

impl InterfaceConfiguration {
    pub fn get_controller_url(&self) -> &str {
        &self.controller_url
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    pub fn get_group_id(&self) -> &[u8] {
        &self.group_id
    }
}

impl From<FrontEndInterfaceConfiguration> for InterfaceConfiguration {
    fn from(value: FrontEndInterfaceConfiguration) -> Self {
        let prefix_free_hex_id = &value.selectedGroup[2..];
        let group_id = hex::decode(prefix_free_hex_id).unwrap();

        Self {
            controller_url: value.controllerUrl,
            is_enabled: value.isEnabled,
            group_id,
        }
    }
}
