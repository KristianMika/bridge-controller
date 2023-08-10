use std::{
    borrow::BorrowMut,
    sync::{Arc, Mutex, RwLock},
};

use self::{data_object::DataObject, key_object::SecretKeyObject, template::Template};
use crate::state::object::object_class::ObjectClass;
pub(crate) mod attribute;
pub(crate) mod data_object;
pub(crate) mod key_object;
pub(crate) mod object_class;
pub(crate) mod object_search;
pub(crate) mod template;

pub(crate) trait CryptokiObject {
    fn does_template_match(&self, template: &Template) -> bool;
    // TODO: refactor
    fn store_data(&mut self, data: Vec<u8>);
}

pub(crate) struct CryptokiArc {
    pub value: Arc<RwLock<dyn CryptokiObject + Send + Sync>>,
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
        let Some(class) = template.get_class() else {return None;};
        match class {
            ObjectClass::Data => Some(CryptokiArc {
                value: Arc::new(RwLock::new(DataObject::from_template(template))),
            }),
            ObjectClass::SecretKey => Some(CryptokiArc {
                value: (Arc::new(RwLock::new(SecretKeyObject::from_template(template)))),
            }),
            ObjectClass::PrivateKey => None,
        }
    }
}
