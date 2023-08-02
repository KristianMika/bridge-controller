use std::ptr;

use libc::c_void;

use crate::cryptoki::bindings::{CK_ATTRIBUTE, CK_ATTRIBUTE_TYPE, CK_ULONG};

pub(crate) struct ObjectSearch {
    template: Attribute,
    count: CK_ULONG,
}

impl ObjectSearch {
    pub(crate) fn new(template: Attribute, count: CK_ULONG) -> Self {
        Self { template, count }
    }
}

pub(crate) struct Attribute {
    attribute_type: CK_ATTRIBUTE_TYPE,
    value: Option<Vec<u8>>,
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

#[derive(PartialEq, Eq, Hash)]
pub(crate) struct Object {}

impl Object {
    pub(crate) fn from_template(template: Attribute) -> Self {
        unimplemented!()
    }
}

impl From<CK_ATTRIBUTE> for Object {
    fn from(value: CK_ATTRIBUTE) -> Self {
        Self::from_template(value.into())
    }
}

impl From<Attribute> for Object {
    fn from(value: Attribute) -> Self {
        Self::from_template(value)
    }
}
