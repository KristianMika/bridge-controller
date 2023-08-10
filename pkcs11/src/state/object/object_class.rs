use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::cryptoki::bindings::{
    CKA_CLASS, CKO_DATA, CKO_PRIVATE_KEY, CKO_PUBLIC_KEY, CKO_SECRET_KEY, CK_OBJECT_CLASS,
};

use super::attribute::Attribute;

#[derive(Eq, PartialEq)]
pub(crate) enum ObjectClass {
    Data,
    SecretKey,
    PrivateKey,
    PublicKey,
}
impl ObjectClass {
    pub(crate) fn from_vec(value: &[u8]) -> Option<Self> {
        if value.len() != (CK_OBJECT_CLASS::BITS / 8) as usize {
            return None;
        }

        let mut cursor = Cursor::new(value);
        let Ok(value) = cursor.read_u64::<LittleEndian>() else{ // TODO: get endianity programatically
            return None;
        };
        match value as u32 {
            CKO_SECRET_KEY => Some(ObjectClass::SecretKey),
            CKO_DATA => Some(ObjectClass::Data),
            CKO_PRIVATE_KEY => Some(ObjectClass::PrivateKey),
            CKO_PUBLIC_KEY => Some(ObjectClass::PublicKey),
            _ => None,
        }
    }
}

impl From<Attribute> for Option<ObjectClass> {
    fn from(value: Attribute) -> Self {
        if value.get_attribute_type() as u32 != CKA_CLASS || value.get_attribute_value().is_none() {
            return None;
        }

        let value = value.get_attribute_value().unwrap();
        ObjectClass::from_vec(value)
    }
}
