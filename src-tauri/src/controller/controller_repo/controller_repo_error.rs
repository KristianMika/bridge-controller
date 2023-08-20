use std::{
    error::Error,
    fmt::Display,
    io,
    sync::{MutexGuard, PoisonError},
};

use sled::Db;

#[derive(Debug)]
pub(crate) enum ControllerRepoError {
    IoError(io::Error),
    SerializationError(bincode::ErrorKind),
    LockPoisonedError,
}

impl Error for ControllerRepoError {}

impl Display for ControllerRepoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(io_error) => write!(f, "IoError: {}", io_error),
            Self::SerializationError(serialization_error) => {
                write!(f, "Serialization error: {}", serialization_error)
            }
            Self::LockPoisonedError => write!(f, "Locked poisoned when trying to unlock"),
        }
    }
}

impl From<io::Error> for ControllerRepoError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
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
