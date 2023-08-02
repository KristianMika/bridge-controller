use std::{collections::HashMap, ptr};

use libc::c_void;

use crate::cryptoki::bindings::{
    CKA_COPYABLE, CKA_DESTROYABLE, CKA_LABEL, CKA_MODIFIABLE, CKA_PRIVATE, CKA_TOKEN,
    CKA_UNIQUE_ID, CK_ATTRIBUTE, CK_ATTRIBUTE_TYPE, CK_BBOOL, CK_FALSE, CK_TRUE,
};

pub(crate) struct ObjectSearch {
    template: Template,
}

impl ObjectSearch {
    pub(crate) fn new(template: Template) -> Self {
        Self { template }
    }

    pub(crate) fn get_template(&self) -> &Template {
        &self.template
    }
}

pub(crate) struct Attribute {
    attribute_type: CK_ATTRIBUTE_TYPE,
    value: Option<Vec<u8>>,
}

impl Attribute {
    pub(crate) fn get_attribute_type(&self) -> CK_ATTRIBUTE_TYPE {
        self.attribute_type
    }

    pub(crate) fn get_attribute_value(&self) -> Option<&Vec<u8>> {
        self.value.as_ref()
    }
}

impl From<CK_ATTRIBUTE> for Attribute {
    fn from(template: CK_ATTRIBUTE) -> Self {
        let mut template_value = None;
        if template.ulValueLen > 0 {
            let mut value = Vec::with_capacity(template.ulValueLen as usize);
            unsafe {
                ptr::copy(
                    template.pValue,
                    value.as_mut_ptr(),
                    template.ulValueLen as usize,
                );
                value.set_len(template.ulValueLen as usize);
            }
            template_value = Some(value.into_iter().map(|b: c_void| b as u8).collect());
        }
        Attribute::new(template.type_, template_value)
    }
}

impl Attribute {
    pub fn new(attribute_type: CK_ATTRIBUTE_TYPE, value: Option<Vec<u8>>) -> Self {
        Self {
            attribute_type,
            value,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Default)]
pub(crate) struct DataObject {
    CKA_TOKEN: CK_BBOOL,
    CKA_PRIVATE: CK_BBOOL,
    CKA_MODIFIABLE: CK_BBOOL,
    CKA_LABEL: Vec<u8>,
    CKA_COPYABLE: CK_BBOOL,
    CKA_DESTROYABLE: CK_BBOOL,
    CKA_UNIQUE_ID: Vec<u8>,
}

impl DataObject {
    pub(crate) fn from_template(template: Vec<Attribute>) -> Self {
        let mut data_object = DataObject::default();
        let template = Template::from_vec(template);
        // TODO: check class
        // if template.get(&(CKA_CLASS as u64)).unwrap() != CKO_DATA {
        //     unimplemented!()
        // }
        Self {
            CKA_TOKEN: template
                .get_bool(&(CKA_TOKEN as CK_ATTRIBUTE_TYPE))
                .unwrap_or(CK_TRUE as u8),
            CKA_PRIVATE: template
                .get_bool(&(CKA_PRIVATE as CK_ATTRIBUTE_TYPE))
                .unwrap_or(CK_TRUE as u8),
            CKA_MODIFIABLE: template
                .get_bool(&(CKA_MODIFIABLE as CK_ATTRIBUTE_TYPE))
                .unwrap_or(CK_FALSE as u8),
            CKA_COPYABLE: template
                .get_bool(&(CKA_COPYABLE as CK_ATTRIBUTE_TYPE))
                .unwrap_or(CK_FALSE as u8),
            CKA_DESTROYABLE: template
                .get_bool(&(CKA_DESTROYABLE as CK_ATTRIBUTE_TYPE))
                .unwrap_or(CK_TRUE as u8),
            CKA_LABEL: template
                .get_value(&(CKA_LABEL as CK_ATTRIBUTE_TYPE))
                .unwrap_or(vec![]),
            CKA_UNIQUE_ID: template
                .get_value(&(CKA_UNIQUE_ID as CK_ATTRIBUTE_TYPE))
                .unwrap_or(vec![]),
        }
    }

    pub(crate) fn does_template_match(&self, template: &Template) -> bool {
        // TODO: class
        if let Some(label) = template.get_value(&(CKA_LABEL as CK_ATTRIBUTE_TYPE)) {
            if label != self.CKA_LABEL {
                return false;
            }
        };

        if let Some(unique_id) = template.get_value(&(CKA_UNIQUE_ID as CK_ATTRIBUTE_TYPE)) {
            if unique_id != self.CKA_UNIQUE_ID {
                return false;
            }
        }
        // TODO: apply other filters

        true
    }
}

impl From<Vec<CK_ATTRIBUTE>> for DataObject {
    fn from(value: Vec<CK_ATTRIBUTE>) -> Self {
        Self::from_template(value.into_iter().map(|t| t.into()).collect())
    }
}

impl From<Vec<Attribute>> for DataObject {
    fn from(value: Vec<Attribute>) -> Self {
        Self::from_template(value)
    }
}

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

    fn into_attributes(self) -> HashMap<CK_ATTRIBUTE_TYPE, Option<Vec<u8>>> {
        self.attributes
    }

    fn get_bool(&self, key: &CK_ATTRIBUTE_TYPE) -> Option<CK_BBOOL> {
        let Some(Some(value)) = self.attributes.get(key) else {return None;};
        let Some(&value) = value.get(0) else{
            return None;
        };
        Some(value as CK_BBOOL)
    }

    fn get_value(&self, key: &CK_ATTRIBUTE_TYPE) -> Option<Vec<u8>> {
        self.attributes.get(key).cloned().unwrap_or(None)
    }
}

impl From<Vec<CK_ATTRIBUTE>> for Template {
    fn from(value: Vec<CK_ATTRIBUTE>) -> Self {
        Self::from_vec(value.into_iter().map(|t| t.into()).collect())
    }
}
