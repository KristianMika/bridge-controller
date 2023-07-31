extern crate libc;

pub(crate) mod communicator;
pub mod cryptoki;
pub(crate) mod session;
mod state;

use crate::{communicator::meesign::Meesign, state::CryptokiState};
use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    pub(crate) static ref STATE: RwLock<CryptokiState<Meesign>> =
        RwLock::new(CryptokiState::default());
}
