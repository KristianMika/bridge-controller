use std::collections::HashMap;

use aes::Aes128;
use openssl::hash::Hasher;
use rand::{rngs::OsRng, Rng};

use crate::{
    cryptoki::bindings::CK_OBJECT_HANDLE,
    state::{
        object::{object_search::ObjectSearch, CryptokiArc},
        slots::TokenStore,
    },
};

/// Holds the current state of PKCS#11 lib
pub(crate) struct Session {
    /// Holds the object managed by functions C_Digest*
    hasher: Option<Hasher>,

    object_search: Option<ObjectSearch>,

    // TODO: objects should be held by the token struct
    objects: HashMap<CK_OBJECT_HANDLE, CryptokiArc>,

    token: TokenStore,

    encryptor: Option<Aes128>,

    signer: Option<Signer>,
}

#[derive(Clone)]
pub(crate) struct Signer {
    pub key: CryptokiArc,
    pub response: Option<Vec<u8>>,
    pub task_id: Option<Vec<u8>>,
}
impl Signer {
    pub(crate) fn new(key: CryptokiArc) -> Self {
        Self {
            key,
            response: None,
            task_id: None,
        }
    }
}
impl Session {
    pub(crate) fn new(token: TokenStore) -> Self {
        Self {
            hasher: None,
            object_search: None,
            objects: Default::default(),
            token,
            encryptor: None,
            signer: None,
        }
    }
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

    pub fn reset_object_search(&mut self) {
        self.object_search = None;
    }

    pub fn create_object(&mut self, object: CryptokiArc) -> CK_OBJECT_HANDLE {
        let object_handle = self.generate_object_handle();

        let _ = self.objects.insert(object_handle, object);
        object_handle
    }

    // TODO: custom error
    pub fn destroy_object(&mut self, object_handle: &CK_OBJECT_HANDLE) -> Option<CryptokiArc> {
        // todo: are we sure this is the only arc?
        self.objects.remove(object_handle)
    }

    pub(crate) fn get_object(&self, object_handle: CK_OBJECT_HANDLE) -> Option<CryptokiArc> {
        self.objects.get(&object_handle).cloned()
    }

    fn generate_object_handle(&self) -> CK_OBJECT_HANDLE {
        let mut object_handle = OsRng.gen_range(0..CK_OBJECT_HANDLE::MAX);
        while self.objects.contains_key(&object_handle) {
            object_handle = OsRng.gen_range(0..CK_OBJECT_HANDLE::MAX);
        }

        object_handle
    }

    pub(crate) fn get_token(&self) -> TokenStore {
        self.token.clone()
    }

    // TODO: return an error if search not innited
    pub fn get_filtered_handles(&self) -> Vec<CK_OBJECT_HANDLE> {
        let Some( object_search) = self.object_search.as_ref() else {
            return vec![]; // TODO: return error
        };
        self.objects
            .iter()
            .filter(|(_handle, object)| object.does_template_match(object_search.get_template()))
            .map(|(&handle, _)| handle)
            .collect()
    }

    pub fn set_encryptor(&mut self, encryptor: Aes128) {
        self.encryptor = Some(encryptor);
    }

    pub fn get_encryptor(&self) -> Option<Aes128> {
        self.encryptor.clone()
    }

    pub fn set_signer(&mut self, signer: Signer) {
        self.signer = Some(signer)
    }

    pub fn get_signer(&self) -> Option<Signer> {
        self.signer.clone()
    }

    pub fn store_signing_response(&mut self, response: Vec<u8>) {
        let Some(ref mut signer) = self.signer else {
            return;
        };

        signer.response = Some(response);
    }

    pub fn get_signing_response(&self) -> Option<Vec<u8>> {
        let Some(ref signer) = self.signer else {
            return None;
        };
        signer.response.clone()
    }

    pub fn set_signer_task_id(&mut self, task_id: Vec<u8>) {
        let Some(ref mut signer) = self.signer else {
            return;
        };
        signer.task_id = Some(task_id)
    }
    pub fn get_signing_task_id(&self) -> Option<Vec<u8>> {
        let Some(ref signer) = self.signer else {
            return None;
        };
        signer.task_id.clone()
    }
}
