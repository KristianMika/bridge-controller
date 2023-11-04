use std::{
    io,
    sync::{MutexGuard, PoisonError},
};

use sled::Db;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum ControllerRepoError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("Serialization error: {0}")]
    SerializationError(bincode::ErrorKind),
    #[error("Locke poisoned when trying to unlock")]
    LockPoisonedError,
    #[error("Sled error: {0}")]
    SledError(#[from] sled::Error),
}

impl From<Box<bincode::ErrorKind>> for ControllerRepoError {
    fn from(value: Box<bincode::ErrorKind>) -> Self {
        Self::SerializationError(*value)
    }
}

impl From<PoisonError<MutexGuard<'_, Db>>> for ControllerRepoError {
    fn from(_value: PoisonError<MutexGuard<'_, Db>>) -> Self {
        Self::LockPoisonedError
    }
}
