use std::sync::Arc;

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
}

pub(crate) struct CryptokiArc {
    pub value: Arc<dyn CryptokiObject + Send + Sync>,
}

impl CryptokiArc {
    pub(crate) fn does_template_match(&self, template: &Template) -> bool {
        self.value.as_ref().does_template_match(template)
    }
}

impl From<Template> for Option<CryptokiArc> {
    fn from(template: Template) -> Self {
        let Some(class) = template.get_class() else {return None;};
        match class {
            ObjectClass::Data => Some(CryptokiArc {
                value: Arc::new(DataObject::from_template(template)),
            }),
            ObjectClass::SecretKey => Some(CryptokiArc {
                value: (Arc::new(SecretKeyObject::from_template(template))),
            }),
            ObjectClass::PrivateKey => None,
        }
    }
}
