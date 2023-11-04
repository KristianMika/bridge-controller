use std::{
    io,
    sync::{MutexGuard, PoisonError},
};

use sled::Db;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum ControllerRepoError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Serialization error: {0}")]
    Serialization(bincode::ErrorKind),
    #[error("Locke poisoned when trying to unlock")]
    LockPoisoned,
    #[error("Sled error: {0}")]
    Sled(#[from] sled::Error),
}

impl From<Box<bincode::ErrorKind>> for ControllerRepoError {
    fn from(value: Box<bincode::ErrorKind>) -> Self {
        Self::Serialization(*value)
    }
}

impl From<PoisonError<MutexGuard<'_, Db>>> for ControllerRepoError {
    fn from(_value: PoisonError<MutexGuard<'_, Db>>) -> Self {
        Self::LockPoisoned
    }
}
