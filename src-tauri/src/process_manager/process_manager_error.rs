use std::io;

use thiserror::Error;

use super::CreatableInterface;

#[derive(Debug, Error)]
pub(crate) enum ProcessManagerError {
    #[error("Process for interface {0:?} is already running")]
    ProcessAlreadyRunning(CreatableInterface),
    #[error("Process for interface {0:?} is not running")]
    ProcessNotRunning(CreatableInterface),
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
}
