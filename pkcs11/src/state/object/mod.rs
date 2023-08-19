use std::sync::{Arc, RwLock};

use self::{
    data_object::DataObject, private_key_object::PrivateKeyObject,
    public_key_object::PublicKeyObject, secret_key_object::SecretKeyObject, template::Template,
};
use crate::{cryptoki::bindings::CK_ATTRIBUTE_TYPE, state::object::object_class::ObjectClass};

pub(crate) mod attribute;
pub(crate) mod cryptoki_object;
pub(crate) mod data_object;
pub(crate) mod object_class;
pub(crate) mod object_search;
pub(crate) mod private_key_object;
pub(crate) mod public_key_object;
pub(crate) mod secret_key_object;
pub(crate) mod template;
