use std::collections::HashSet;

use openssl::hash::Hasher;

use crate::state::object::{Object, ObjectSearch};

/// Holds the current state of PKCS#11 lib
#[derive(Default)]
pub(crate) struct Session {
    /// Holds the object managed by functions C_Digest*
    hasher: Option<Hasher>,

    object_search: Option<ObjectSearch>,

    // TODO: objects should be held by the token struct
    objects: HashSet<Object>,
}

impl Session {
    pub fn get_hasher_mut(&mut self) -> Option<&mut Hasher> {
        self.hasher.as_mut()
    }

    pub fn set_hasher(&mut self, hasher: Hasher) {
        self.hasher = Some(hasher)
    }

    pub fn set_object_search(&mut self, object_search: ObjectSearch) {
        self.object_search = Some(object_search);
    }

    pub fn get_object_search(&self) -> Option<&ObjectSearch> {
        self.object_search.as_ref()
    }

    pub fn init_object_search(&mut self, object_search: ObjectSearch) {
        self.object_search = Some(object_search);
    }

    pub fn create_object(&mut self, object: Object) {
        let _ = self.objects.insert(object);
    }
}
