use std::collections::HashMap;

use crate::cryptoki::bindings::{CKA_CLASS, CK_ATTRIBUTE, CK_ATTRIBUTE_TYPE, CK_BBOOL};

use super::{attribute::Attribute, object_class::ObjectClass};

pub(crate) struct Template {
    attributes: HashMap<CK_ATTRIBUTE_TYPE, Option<Vec<u8>>>,
}

impl Template {
    pub(crate) fn from_vec(attributes: Vec<Attribute>) -> Self {
        let mut attributes_map = HashMap::new();
        attributes.into_iter().for_each(|attribute| {
            attributes_map.insert(
                attribute.get_attribute_type(),
                attribute.get_attribute_value().cloned(),
            );
        });
        Self {
            attributes: attributes_map,
        }
    }

    pub(crate) fn get_bool(&self, key: &CK_ATTRIBUTE_TYPE) -> Option<CK_BBOOL> {
        let Some(Some(value)) = self.attributes.get(key) else {return None;};
        let Some(&value) = value.get(0) else{
            return None;
        };
        Some(value as CK_BBOOL)
    }

    pub(crate) fn get_value(&self, key: &CK_ATTRIBUTE_TYPE) -> Option<Vec<u8>> {
        self.attributes.get(key).cloned().unwrap_or(None)
    }

    pub(crate) fn get_class(&self) -> Option<ObjectClass> {
        let Some(value) = self.get_value(&(CKA_CLASS as u64)) else {return None;};
        ObjectClass::from_vec(&value)
    }
}

impl From<Vec<CK_ATTRIBUTE>> for Template {
    fn from(value: Vec<CK_ATTRIBUTE>) -> Self {
        Self::from_vec(value.into_iter().map(|t| t.into()).collect())
    }
}
