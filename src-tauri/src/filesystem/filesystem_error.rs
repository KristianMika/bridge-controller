use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum FilesystemError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("Couldn't get home directory")]
    HomeDirectoryError,
}
