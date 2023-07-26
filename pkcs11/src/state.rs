use openssl::hash::Hasher;
use rand::{rngs::OsRng, Rng};
use std::{
    collections::HashMap,
    error::Error,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};
use tokio::runtime::Runtime;
use tonic::transport::Certificate;

use crate::{
    communicator::{meesign::Meesign, Communicator},
    cryptoki::bindings::CK_SESSION_HANDLE,
};

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

pub(crate) struct CryptokiState<T>
where
    T: Communicator,
{
    sessions: HashMap<CK_SESSION_HANDLE, RwLock<SessionState>>,
    communicator: T,
    runtime: Runtime,
}

impl<T> CryptokiState<T>
where
    T: Communicator,
{
    fn generate_session_handle(&self) -> CK_SESSION_HANDLE {
        OsRng.gen_range(0..CK_SESSION_HANDLE::MAX)
    }

    pub(crate) fn create_session(&mut self) -> CK_SESSION_HANDLE {
        let new_session_state = RwLock::new(SessionState::default());

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
    ) -> Option<RwLockReadGuard<SessionState>> {
        match self.sessions.get(session_handle) {
            None => None,
            // TODO: unrap
            Some(session) => Some(session.read().unwrap()),
        }
    }

    pub(crate) fn get_session_mut(
        &mut self,
        session_handle: &CK_SESSION_HANDLE,
    ) -> Option<RwLockWriteGuard<SessionState>> {
        match self.sessions.get_mut(session_handle) {
            None => None,
            // TODO: unrap
            Some(session) => Some(session.write().unwrap()),
        }
    }

    pub(crate) fn finalize(&mut self) {
        self.sessions.clear();
        self.sessions.shrink_to_fit();
    }

    pub(crate) async fn get_groups(&mut self) -> Result<Vec<String>, Box<dyn Error>> {
        self.communicator.get_groups().await
    }

    pub(crate) fn get_groups_blocking(&mut self) -> Result<Vec<String>, Box<dyn Error>> {
        self.runtime
            .block_on(async { self.communicator.get_groups().await })
    }

    pub(crate) fn new(communicator: T, runtime: Runtime) -> Self
    where
        T: Communicator,
    {
        Self {
            sessions: Default::default(),
            communicator,
            runtime,
        }
    }

    pub fn get_runtime(&self) -> &Runtime {
        &self.runtime
    }
}

impl Default for CryptokiState<Meesign> {
    // TODO: just tmp, remove later, pls don't look
    fn default() -> Self {
        let cert = Certificate::from_pem(
            std::fs::read("/home/kiko/Desktop/tmp/meesign-server/keys/meesign-ca-cert.pem")
                .unwrap(),
        );
        let runtime = Runtime::new().unwrap();
        let meesign = runtime.block_on(async move {
            Meesign::new("meesign.local".into(), 1337, cert)
                .await
                .unwrap()
        });
        Self::new(meesign, runtime)
    }
}
