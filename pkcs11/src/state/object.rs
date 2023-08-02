use std::ptr;

use libc::c_void;

use crate::cryptoki::bindings::{
    CKO_CERTIFICATE, CKO_DATA, CKO_DOMAIN_PARAMETERS, CKO_HW_FEATURE, CKO_MECHANISM, CKO_OTP_KEY,
    CKO_PRIVATE_KEY, CKO_PROFILE, CKO_PUBLIC_KEY, CKO_SECRET_KEY, CKO_VENDOR_DEFINED, CK_ATTRIBUTE,
    CK_ATTRIBUTE_TYPE, CK_ULONG,
};

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
pub(crate) struct Object {
    value: Vec<u8>,
}

impl Object {
    pub(crate) fn from_template(template: Attribute) -> Self {
        match template.attribute_type as u32 {
            // holds information defined by an application
            CKO_DATA => Self {
                value: template.value.unwrap(),
            },
            CKO_CERTIFICATE => unimplemented!(),
            CKO_PUBLIC_KEY => unimplemented!(),
            CKO_PRIVATE_KEY => unimplemented!(),
            CKO_SECRET_KEY => unimplemented!(),
            CKO_HW_FEATURE => unimplemented!(),
            CKO_DOMAIN_PARAMETERS => unimplemented!(),
            CKO_MECHANISM => unimplemented!(),
            CKO_OTP_KEY => unimplemented!(),
            CKO_PROFILE => unimplemented!(),
            CKO_VENDOR_DEFINED => unimplemented!(),
            _ => panic!(), //TODO
        }
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
