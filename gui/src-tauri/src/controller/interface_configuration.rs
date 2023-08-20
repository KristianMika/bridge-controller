use serde::{Deserialize, Serialize};
use specta::Type;

type ByteVector = Vec<u8>;
pub(crate) type GroupId = ByteVector;

#[derive(Serialize, Deserialize, Debug, Type)]

pub(crate) struct InterfaceConfiguration {
    controller_url: String,
    group_id: GroupId,
}

impl InterfaceConfiguration {
    pub fn get_controller_url(&self) -> &str {
        &self.controller_url
    }
}
