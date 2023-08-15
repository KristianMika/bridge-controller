extern crate libc;

pub(crate) mod communicator;
pub mod cryptoki;
pub(crate) mod state;

use crate::state::state::CryptokiState;
use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    pub(crate) static ref STATE: RwLock<Option<CryptokiState>> = RwLock::new(None);
}
