use serde::{Deserialize, Serialize};
use specta::Type;

use super::GroupId;

/// Interface configuration used within the back-end (DB, and the rust app)
#[derive(Serialize, Deserialize, Debug, Type, Clone)]
pub(crate) struct InternalInterfaceConfiguration {
    communicator_hostname: String,
    group_id: GroupId,
    is_enabled: bool,
}

impl InternalInterfaceConfiguration {
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
