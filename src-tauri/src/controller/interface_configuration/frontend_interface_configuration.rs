use hex::ToHex;
use serde::{Deserialize, Serialize};
use specta::Type;

use super::internal_interface_configuration::InternalInterfaceConfiguration;

/// Interface configuration used within the front-end
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

impl From<FrontEndInterfaceConfiguration> for InternalInterfaceConfiguration {
    fn from(value: FrontEndInterfaceConfiguration) -> Self {
        let prefix_free_hex_id = &value.selectedGroup[2..];
        let group_id = hex::decode(prefix_free_hex_id).unwrap();

        InternalInterfaceConfiguration::new(value.communicatorHostname, group_id, value.isEnabled)
    }
}
