use hex::ToHex;
use serde::{Deserialize, Serialize};
use specta::Type;

type ByteVector = Vec<u8>;
pub(crate) type GroupId = ByteVector;

#[derive(Serialize, Deserialize, Debug, Type, Clone)]

pub(crate) struct InternalInterfaceConfiguration {
    communicator_hostname: String,
    group_id: GroupId,
    is_enabled: bool,
}

impl InternalInterfaceConfiguration {
    #[cfg(test)]
    pub fn new(communicator_hostname: String, group_id: GroupId, is_enabled: bool) -> Self {
        Self {
            communicator_hostname,
            group_id,
            is_enabled,
        }
    }
    pub fn get_communicator_hostname(&self) -> &str {
        &self.communicator_hostname
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
            communicator_hostname: value.communicatorHostname,
            is_enabled: value.isEnabled,
            group_id,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Type, Debug)]
pub(crate) struct FrontEndInterfaceConfiguration {
    isEnabled: bool,
    communicatorHostname: String,
    selectedGroup: String,
}

impl From<InternalInterfaceConfiguration> for FrontEndInterfaceConfiguration {
    fn from(value: InternalInterfaceConfiguration) -> Self {
        Self {
            isEnabled: value.is_enabled(),
            communicatorHostname: value.get_communicator_hostname().into(),
            selectedGroup: format!("0x{}", value.get_group_id().encode_hex_upper::<String>()),
        }
    }
}
