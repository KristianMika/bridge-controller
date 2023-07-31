use std::{
    collections::HashMap,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use rand::{rngs::OsRng, Rng};

use crate::cryptoki::bindings::CK_SESSION_HANDLE;

use super::session::Session;

#[derive(Default)]
pub(crate) struct Sessions {
    sessions: HashMap<CK_SESSION_HANDLE, RwLock<Session>>,
}

impl Sessions {
    fn generate_session_handle(&self) -> CK_SESSION_HANDLE {
        OsRng.gen_range(0..CK_SESSION_HANDLE::MAX)
    }

    pub(crate) fn create_session(&mut self) -> CK_SESSION_HANDLE {
        let new_session_state = RwLock::new(Session::default());

        let mut session_handle = self.generate_session_handle();
        while self.sessions.contains_key(&session_handle) {
            session_handle = self.generate_session_handle();
        }
        self.sessions.insert(session_handle, new_session_state);

        session_handle
    }

    pub(crate) fn close_session(&mut self, session_handle: &CK_SESSION_HANDLE) {
        self.sessions.remove(session_handle);
        self.sessions.shrink_to_fit();
    }

    pub(crate) fn get_session(
        &self,
        session_handle: &CK_SESSION_HANDLE,
    ) -> Option<RwLockReadGuard<Session>> {
        match self.sessions.get(session_handle) {
            None => None,
            // TODO: unrap
            Some(session) => Some(session.read().unwrap()),
        }
    }

    pub(crate) fn get_session_mut(
        &mut self,
        session_handle: &CK_SESSION_HANDLE,
    ) -> Option<RwLockWriteGuard<Session>> {
        match self.sessions.get_mut(session_handle) {
            None => None,
            // TODO: unrap
            Some(session) => Some(session.write().unwrap()),
        }
    }

    pub(crate) fn close_sessions(&mut self) {
        self.sessions.clear();
        self.sessions.shrink_to_fit();
    }
}
