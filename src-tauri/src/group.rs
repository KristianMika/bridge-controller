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
            group_id: public_key_as_hex(&value.identifier),
        }
    }
}

fn public_key_as_hex(identifier: &[u8]) -> String {
    format!("0x{}", identifier.encode_hex_upper::<String>())
}
