use crate::cryptoki::bindings::{CKA_EC_POINT, CK_ATTRIBUTE_TYPE};

use super::{object_class::ObjectClass, template::Template, CryptokiObject};

pub(crate) struct PublicKeyObject {
    data: Vec<u8>,
}

impl PublicKeyObject {
    pub(crate) fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}
impl CryptokiObject for PublicKeyObject {
    fn does_template_match(&self, template: &Template) -> bool {
        if let Some(class) = template.get_class() {
            class == ObjectClass::PublicKey
        } else {
            false
        }
    }

    fn store_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }
    fn from_template(template: Template) -> Self
    where
        Self: Sized,
    {
        // TODO

        Self { data: vec![] }
    }

    fn get_attribute(&self, attribute_type: CK_ATTRIBUTE_TYPE) -> Option<Vec<u8>> {
        // todo implement
        if attribute_type != CKA_EC_POINT as u64 {
            return None;
        }
        let mut data = self.data.clone();
        data.insert(0, 0x04);
        data.insert(1, 65); // TODO: into a method

        Some(data)
    }
}
