use openssl::hash::Hasher;
use rand::{rngs::OsRng, Rng};
use std::{
    collections::HashMap,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use crate::bindings::CK_SESSION_HANDLE;

/// Holds the current state of PKCS#11 lib
#[derive(Default)]
pub struct SessionState {
    /// Holds the object managed by functions C_Digest*
    hasher: Option<Hasher>,
}

impl SessionState {
    pub fn get_hasher(&self) -> Option<&Hasher> {
        self.hasher.as_ref()
    }
    pub fn get_hasher_mut(&mut self) -> Option<&mut Hasher> {
        self.hasher.as_mut()
    }

    pub fn set_hasher(&mut self, hasher: Hasher) {
        self.hasher = Some(hasher)
    }
}

#[derive(Default)]
pub struct CryptokiState {
    sessions: HashMap<CK_SESSION_HANDLE, RwLock<SessionState>>,
}

impl CryptokiState {
    fn generate_session_handle(&self) -> CK_SESSION_HANDLE {
        OsRng.gen_range(0..CK_SESSION_HANDLE::MAX)
    }

    pub fn create_session(&mut self) -> CK_SESSION_HANDLE {
        let new_session_state = RwLock::new(SessionState::default());

        let mut session_handle = self.generate_session_handle();
        while self.sessions.contains_key(&session_handle) {
            session_handle = self.generate_session_handle();
        }
        self.sessions.insert(session_handle, new_session_state);

        session_handle
    }

    pub fn close_session(&mut self, session_handle: &CK_SESSION_HANDLE) {
        self.sessions.remove(session_handle);
        self.sessions.shrink_to_fit();
    }

    pub fn get_session(
        &self,
        session_handle: &CK_SESSION_HANDLE,
    ) -> Option<RwLockReadGuard<SessionState>> {
        match self.sessions.get(session_handle) {
            None => None,
            // TODO: unrap
            Some(session) => Some(session.read().unwrap()),
        }
    }

    pub fn get_session_mut(
        &mut self,
        session_handle: &CK_SESSION_HANDLE,
    ) -> Option<RwLockWriteGuard<SessionState>> {
        match self.sessions.get_mut(session_handle) {
            None => None,
            // TODO: unrap
            Some(session) => Some(session.write().unwrap()),
        }
    }
}
