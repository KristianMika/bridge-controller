use crate::cryptoki::bindings::{CK_ATTRIBUTE, CK_ATTRIBUTE_TYPE, CK_BBOOL, CK_FALSE, CK_TRUE};

use super::{data_object::DataObject, template::Template, CryptokiObject};

// TODO: consider using bool at this lvl of abstraction
pub(crate) struct SecretKeyObject {
    data: DataObject,
    is_sensitive: CK_BBOOL,
    supports_encryption: CK_BBOOL,
    supports_decryption: CK_BBOOL,
    supports_signatures: CK_BBOOL,
    supports_verification: CK_BBOOL,
    supports_wrapping: CK_BBOOL,
    supports_unwrapping: CK_BBOOL,
    is_exctractable: CK_BBOOL,
    is_always_sensitive: CK_BBOOL,
    is_never_exctractable: CK_BBOOL,
    key_checksum: Vec<u8>,
    only_wrap_with_trusted_key: CK_BBOOL,
    is_key_trusted: CK_BBOOL,
    wrap_template: Option<Template>,
    unwrap_template: Option<Template>,
}

impl CryptokiObject for SecretKeyObject {
    fn does_template_match(&self, template: &Template) -> bool {
        self.data.does_template_match(template)
        // TODO: apply other filters
    }

    fn store_data(&mut self, data: Vec<u8>) {
        self.data.store_data(data)
    }

    fn from_template(template: Template) -> Self
    where
        Self: Sized,
    {
        let data = DataObject::from_template(template);
        // todo: create from template
        Self {
            data,
            is_sensitive: CK_FALSE as CK_BBOOL,
            supports_encryption: CK_FALSE as CK_BBOOL,
            supports_decryption: CK_FALSE as CK_BBOOL,
            supports_signatures: CK_FALSE as CK_BBOOL,
            supports_verification: CK_FALSE as CK_BBOOL,
            supports_wrapping: CK_FALSE as CK_BBOOL,
            supports_unwrapping: CK_FALSE as CK_BBOOL,
            is_exctractable: CK_FALSE as CK_BBOOL,
            is_always_sensitive: CK_TRUE as CK_BBOOL,
            is_never_exctractable: CK_TRUE as CK_BBOOL,
            key_checksum: vec![],
            only_wrap_with_trusted_key: CK_FALSE as CK_BBOOL,
            is_key_trusted: CK_FALSE as CK_BBOOL,
            wrap_template: None,
            unwrap_template: None,
        }
    }

    fn get_attribute(&self, attribute_type: CK_ATTRIBUTE_TYPE) -> Option<Vec<u8>> {
        // todo
        None
    }

    fn get_data(&self) -> Vec<u8> {
        self.data.get_data()
    }
}

impl From<Vec<CK_ATTRIBUTE>> for SecretKeyObject {
    fn from(value: Vec<CK_ATTRIBUTE>) -> Self {
        let template = Template::from_vec(value.into_iter().map(|t| t.into()).collect());
        Self::from_template(template)
    }
}

impl From<Template> for SecretKeyObject {
    fn from(value: Template) -> Self {
        Self::from_template(value)
    }
}
