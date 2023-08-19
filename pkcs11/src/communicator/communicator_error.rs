use std::{error::Error, fmt::Display};

use tonic::{codegen::http::uri::InvalidUri, Status};

type WaitingTimeSeconds = u64;

#[derive(Debug)]
pub(crate) enum CommunicatorError {
    TransportError,
    InvalidConfigurationError,
    TaskFailedError,
    TaskTimedOutError(WaitingTimeSeconds),
    #[cfg(feature = "mocked_meesign")]
    CryptographicError,
}

impl Error for CommunicatorError {}

impl Display for CommunicatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommunicatorError::TransportError => write!(f, "Transport error"),
            CommunicatorError::InvalidConfigurationError => write!(f, "Invalid configuration"),
            CommunicatorError::TaskFailedError => write!(f, "Task failed remotely"),
            CommunicatorError::TaskTimedOutError(waiting_time) => {
                write!(f, "Task hasn't finished within {} seconds", waiting_time)
            }
            #[cfg(feature = "mocked_meesign")]
            CommunicatorError::CryptographicError => write!(f, "Cryptographic operation failed"),
        }
    }
}

impl From<tonic::transport::Error> for CommunicatorError {
    fn from(_value: tonic::transport::Error) -> Self {
        Self::TransportError
    }
}

impl From<Status> for CommunicatorError {
    fn from(_value: Status) -> Self {
        Self::TransportError
    }
}

impl From<InvalidUri> for CommunicatorError {
    fn from(_value: InvalidUri) -> Self {
        Self::InvalidConfigurationError
    }
}

#[cfg(feature = "mocked_meesign")]
impl From<p256::ecdsa::Error> for CommunicatorError {
    fn from(_value: p256::ecdsa::Error) -> Self {
        Self::CryptographicError
    }
}
