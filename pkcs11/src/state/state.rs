use std::{
    error::Error,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};
use tokio::runtime::Runtime;
use tonic::transport::Certificate;

use crate::{
    communicator::{meesign::Meesign, Communicator, Group},
    cryptoki::bindings::{CK_SESSION_HANDLE, CK_SLOT_ID, CK_TOKEN_INFO},
};

use super::{
    session::{session::Session, sessions::Sessions},
    slots::{Slots, TokenStore},
    token::{MeesignToken, Token},
};

pub(crate) struct CryptokiState<C>
where
    C: Communicator,
{
    sessions: Sessions,
    communicator: C,
    runtime: Runtime,
    slots: Slots,
}

impl<C> CryptokiState<C>
where
    C: Communicator,
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

    pub(crate) async fn get_groups(&mut self) -> Result<Vec<Group>, Box<dyn Error>> {
        self.communicator.get_groups().await
    }

    pub(crate) fn get_groups_blocking(&mut self) -> Result<Vec<Group>, Box<dyn Error>> {
        self.runtime
            .block_on(async { self.communicator.get_groups().await })
    }

    pub(crate) fn insert_token(&mut self, token: TokenStore) -> CK_SLOT_ID {
        self.slots.insert_token(token)
    }

    pub(crate) fn get_token_info(&self, slot_id: &CK_SLOT_ID) -> Option<CK_TOKEN_INFO> {
        self.slots.get_token_info(slot_id)
    }

    pub(crate) fn new(communicator: C, runtime: Runtime) -> Self
    where
        C: Communicator,
    {
        Self {
            sessions: Default::default(),
            communicator,
            runtime,
            slots: Slots::new(),
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
