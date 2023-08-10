use super::{object_class::ObjectClass, template::Template, CryptokiObject};

pub(crate) struct PublicKeyObject {
    data: Vec<u8>,
}

impl PublicKeyObject {
    pub(crate) fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}
impl CryptokiObject for PublicKeyObject {
    fn does_template_match(&self, template: &Template) -> bool {
        if let Some(class) = template.get_class() {
            class == ObjectClass::PublicKey
        } else {
            false
        }
    }

    fn store_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }
    fn from_template(template: Template) -> Self
    where
        Self: Sized,
    {
        // TODO

        Self { data: vec![] }
    }
}
