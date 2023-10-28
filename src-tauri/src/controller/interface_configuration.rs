use hex::ToHex;
use serde::{Deserialize, Serialize};
use specta::Type;

type ByteVector = Vec<u8>;
pub(crate) type GroupId = ByteVector;

#[derive(Serialize, Deserialize, Debug, Type, Clone)]

pub(crate) struct InternalInterfaceConfiguration {
    communicator_url: String,
    group_id: GroupId,
    is_enabled: bool,
}

impl InternalInterfaceConfiguration {
    #[cfg(test)]
    pub fn new(communicator_url: String, group_id: GroupId, is_enabled: bool) -> Self {
        Self {
            communicator_url,
            group_id,
            is_enabled,
        }
    }
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

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Type, Debug)]
pub(crate) struct FrontEndInterfaceConfiguration {
    isEnabled: bool,
    communicatorUrl: String,
    selectedGroup: String,
}

impl From<InternalInterfaceConfiguration> for FrontEndInterfaceConfiguration {
    fn from(value: InternalInterfaceConfiguration) -> Self {
        Self {
            isEnabled: value.is_enabled(),
            communicatorUrl: value.get_communicator_url().into(),
            selectedGroup: format!("0x{}", value.get_group_id().encode_hex_upper::<String>()),
        }
    }
}
