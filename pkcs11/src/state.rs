use std::{
    error::Error,
    sync::{RwLockReadGuard, RwLockWriteGuard},
};
use tokio::runtime::Runtime;
use tonic::transport::Certificate;

use crate::{
    communicator::{meesign::Meesign, Communicator},
    cryptoki::bindings::CK_SESSION_HANDLE,
    session::{session::Session, sessions::Sessions},
};

pub(crate) struct CryptokiState<T>
where
    T: Communicator,
{
    sessions: Sessions,
    communicator: T,
    runtime: Runtime,
}

impl<T> CryptokiState<T>
where
    T: Communicator,
{
    pub(crate) fn create_session(&mut self) -> CK_SESSION_HANDLE {
        self.sessions.create_session()
    }

    pub(crate) fn close_session(&mut self, session_handle: &CK_SESSION_HANDLE) {
        self.sessions.close_session(session_handle)
    }

    pub(crate) fn get_session(
        &self,
        session_handle: &CK_SESSION_HANDLE,
    ) -> Option<RwLockReadGuard<Session>> {
        self.sessions.get_session(session_handle)
    }

    pub(crate) fn get_session_mut(
        &mut self,
        session_handle: &CK_SESSION_HANDLE,
    ) -> Option<RwLockWriteGuard<Session>> {
        self.sessions.get_session_mut(session_handle)
    }

    pub(crate) fn finalize(&mut self) {
        self.sessions.close_sessions()
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
