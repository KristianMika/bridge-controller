use crate::cryptoki::bindings::{CK_ATTRIBUTE_TYPE, CK_ULONG};

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

impl Attribute {
    pub fn new(attribute_type: CK_ATTRIBUTE_TYPE, value: Option<Vec<u8>>) -> Self {
        Self {
            attribute_type,
            value,
        }
    }
}
