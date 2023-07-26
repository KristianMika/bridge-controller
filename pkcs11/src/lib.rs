extern crate libc;

pub mod cryptoki;
mod meesign;
mod state;

use crate::state::CryptokiState;
use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    pub(crate) static ref STATE: RwLock<CryptokiState> = RwLock::new(CryptokiState::default());
}
