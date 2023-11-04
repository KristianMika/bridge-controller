use serde::Serialize;

use crate::interface::CryptographicInterface;

/// This struct is used as a key for the sled database.
/// Values are then individually-stored interface configurations.
#[derive(Serialize)]
pub(super) struct ConfigurationKey {
    interface: CryptographicInterface,
    tool: Option<String>,
}

impl ConfigurationKey {
    pub(crate) fn new(interface: CryptographicInterface, tool: Option<String>) -> Self {
        Self { interface, tool }
    }
}
