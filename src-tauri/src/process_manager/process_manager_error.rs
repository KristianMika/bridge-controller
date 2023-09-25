use std::{error::Error, io};

#[derive(Debug)]
pub(crate) enum ProcessManagerError {
    ProcessAlreadyRunning,
    ProcessNotRunning,
    IoError(io::Error),
}

impl Error for ProcessManagerError {}
impl std::fmt::Display for ProcessManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProcessAlreadyRunning => {
                write!(f, "Process is already running")
            }
            Self::ProcessNotRunning => {
                write!(f, "Process is not running")
            }
            Self::IoError(e) => {
                write!(f, "IO error: {}", e)
            }
        }
    }
}

impl From<io::Error> for ProcessManagerError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}
