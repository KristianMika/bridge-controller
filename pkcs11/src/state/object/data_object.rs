use std::vec;

use crate::cryptoki::bindings::{
    CKA_COPYABLE, CKA_DESTROYABLE, CKA_LABEL, CKA_MODIFIABLE, CKA_PRIVATE, CKA_TOKEN,
    CKA_UNIQUE_ID, CK_ATTRIBUTE, CK_ATTRIBUTE_TYPE, CK_BBOOL, CK_FALSE, CK_TRUE,
};

use super::{attribute::Attribute, template::Template, CryptokiObject};

#[derive(PartialEq, Eq, Hash, Default)]
pub(crate) struct DataObject {
    is_token: CK_BBOOL,
    is_private: CK_BBOOL,
    is_modifiable: CK_BBOOL,
    label: Vec<u8>,
    is_copyable: CK_BBOOL,
    is_destroyable: CK_BBOOL,
    unique_id: Vec<u8>,
    data: Vec<u8>,
}

impl CryptokiObject for DataObject {
    fn does_template_match(&self, template: &Template) -> bool {
        // TODO: class
        if let Some(label) = template.get_value(&(CKA_LABEL as CK_ATTRIBUTE_TYPE)) {
            if label != self.label {
                return false;
            }
        };

        if let Some(unique_id) = template.get_value(&(CKA_UNIQUE_ID as CK_ATTRIBUTE_TYPE)) {
            if unique_id != self.unique_id {
                return false;
            }
        }
        // TODO: apply other filters

        true
    }

    fn store_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    fn from_template(template: Template) -> Self
    where
        Self: Sized,
    {
        // TODO: check class
        // if template.get(&(CKA_CLASS as u64)).unwrap() != CKO_DATA {
        //     unimplemented!()
        // }
        Self {
            is_token: template
                .get_bool(&(CKA_TOKEN as CK_ATTRIBUTE_TYPE))
                .unwrap_or(CK_TRUE as u8),
            is_private: template
                .get_bool(&(CKA_PRIVATE as CK_ATTRIBUTE_TYPE))
                .unwrap_or(CK_TRUE as u8),
            is_modifiable: template
                .get_bool(&(CKA_MODIFIABLE as CK_ATTRIBUTE_TYPE))
                .unwrap_or(CK_FALSE as u8),
            is_copyable: template
                .get_bool(&(CKA_COPYABLE as CK_ATTRIBUTE_TYPE))
                .unwrap_or(CK_FALSE as u8),
            is_destroyable: template
                .get_bool(&(CKA_DESTROYABLE as CK_ATTRIBUTE_TYPE))
                .unwrap_or(CK_TRUE as u8),
            label: template
                .get_value(&(CKA_LABEL as CK_ATTRIBUTE_TYPE))
                .unwrap_or(vec![]),
            unique_id: template
                .get_value(&(CKA_UNIQUE_ID as CK_ATTRIBUTE_TYPE))
                .unwrap_or(vec![]),
            data: vec![],
        }
    }
}

impl From<Vec<CK_ATTRIBUTE>> for DataObject {
    fn from(value: Vec<CK_ATTRIBUTE>) -> Self {
        let template = Template::from_vec(value.into_iter().map(|t| t.into()).collect());
        Self::from_template(template)
    }
}

impl From<Template> for DataObject {
    fn from(value: Template) -> Self {
        Self::from_template(value)
    }
}
