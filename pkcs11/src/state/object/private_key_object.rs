use super::{object_class::ObjectClass, template::Template, CryptokiObject};

pub(crate) struct PrivateKeyObject {}

impl PrivateKeyObject {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
impl CryptokiObject for PrivateKeyObject {
    fn does_template_match(&self, template: &Template) -> bool {
        if let Some(class) = template.get_class() {
            class == ObjectClass::PrivateKey
        } else {
            false
        }
    }

    fn store_data(&mut self, _data: Vec<u8>) {}
    fn from_template(template: Template) -> Self
    where
        Self: Sized,
    {
        // TODO

        Self {}
    }
}
