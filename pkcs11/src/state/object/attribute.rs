use std::ptr;

use libc::c_void;

use crate::cryptoki::bindings::{CKA_VALUE, CK_ATTRIBUTE, CK_ATTRIBUTE_TYPE};

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
            let mut value: Vec<u8> = Vec::with_capacity(template.ulValueLen as usize);
            unsafe {
                ptr::copy(
                    template.pValue as *mut u8,
                    value.as_mut_ptr(),
                    template.ulValueLen as usize,
                );
                value.set_len(template.ulValueLen as usize);
            }
            template_value = Some(value);
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
