use serde::{Deserialize, Serialize};
use specta::Type;

use crate::FrontEndInterfaceConfiguration;

type ByteVector = Vec<u8>;
pub(crate) type GroupId = ByteVector;

#[derive(Serialize, Deserialize, Debug, Type)]

pub(crate) struct InternalInterfaceConfiguration {
    communicator_url: String,
    group_id: GroupId,
    is_enabled: bool,
}

impl InternalInterfaceConfiguration {
    pub fn get_communicator_url(&self) -> &str {
        &self.communicator_url
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    pub fn get_group_id(&self) -> &[u8] {
        &self.group_id
    }
    pub fn into_group_id(self) -> GroupId {
        self.group_id
    }
}

impl From<FrontEndInterfaceConfiguration> for InternalInterfaceConfiguration {
    fn from(value: FrontEndInterfaceConfiguration) -> Self {
        let prefix_free_hex_id = &value.selectedGroup[2..];
        let group_id = hex::decode(prefix_free_hex_id).unwrap();

        Self {
            communicator_url: value.communicatorUrl,
            is_enabled: value.isEnabled,
            group_id,
        }
    }
}
