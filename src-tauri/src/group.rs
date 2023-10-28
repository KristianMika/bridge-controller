use hex::ToHex;
use serde::Serialize;
use specta::Type;

use super::proto::Group as ProtoGroup;

#[derive(Type, Serialize, Debug)]
pub(crate) struct Group {
    name: String,
    group_id: String,
}

impl From<ProtoGroup> for Group {
    fn from(value: ProtoGroup) -> Self {
        Self {
            name: value.name,
            group_id: format!("0x{}", value.identifier.encode_hex_upper::<String>()),
        }
    }
}
