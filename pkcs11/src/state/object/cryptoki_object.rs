use std::sync::{Arc, RwLock};

use super::{
    data_object::DataObject, private_key_object::PrivateKeyObject,
    public_key_object::PublicKeyObject, secret_key_object::SecretKeyObject, template::Template,
};
use crate::{cryptoki::bindings::CK_ATTRIBUTE_TYPE, state::object::object_class::ObjectClass};

pub(crate) trait CryptokiObject: Sync + Send {
    fn does_template_match(&self, template: &Template) -> bool;
    // TODO: refactor
    fn store_data(&mut self, data: Vec<u8>);
    fn get_data(&self) -> Vec<u8>;

    fn from_template(template: Template) -> Self
    where
        Self: Sized;

    fn get_attribute(&self, attribute_type: CK_ATTRIBUTE_TYPE) -> Option<Vec<u8>>;
}

#[derive(Clone)]
pub(crate) struct CryptokiArc {
    pub value: Arc<RwLock<dyn CryptokiObject>>,
}

impl CryptokiArc {
    pub(crate) fn does_template_match(&self, template: &Template) -> bool {
        self.value.read().unwrap().does_template_match(template)
    }

    pub(crate) fn store_data(&mut self, data: Vec<u8>) {
        self.value.write().unwrap().store_data(data)
    }
}

impl From<Template> for Option<CryptokiArc> {
    fn from(template: Template) -> Self {
        // TODO: refactor!!!!!
        let Some(class) = template.get_class() else {return None;};
        match class {
            ObjectClass::Data => Some(CryptokiArc {
                value: Arc::new(RwLock::new(DataObject::from_template(template))),
            }),
            ObjectClass::SecretKey => Some(CryptokiArc {
                value: (Arc::new(RwLock::new(SecretKeyObject::from_template(template)))),
            }),
            ObjectClass::PublicKey => Some(CryptokiArc {
                value: Arc::new(RwLock::new(PublicKeyObject::from_template(template))),
            }),
            ObjectClass::PrivateKey => Some(CryptokiArc {
                value: Arc::new(RwLock::new(PrivateKeyObject::from_template(template))),
            }),
        }
    }
}
