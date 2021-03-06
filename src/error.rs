//! Etcd Client Error handling.

use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

/// The error type for `etcdv3` client.
#[derive(Debug)]
pub enum Error {
    /// Invalid arguments
    InvalidArgs(String),

    /// Invalid URI
    InvalidUri(http::uri::InvalidUri),

    /// IO error
    IOError(std::io::Error),

    /// Transport error
    TransportError(tonic::transport::Error),

    /// gRPC status
    GRPCStatus(tonic::Status),
}

impl Display for Error {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidArgs(e) => write!(f, "invalid arguments: {}", e),
            Error::InvalidUri(e) => write!(f, "invalid uri: {}", e),
            Error::IOError(e) => write!(f, "io error: {}", e),
            Error::TransportError(e) => write!(f, "transport error: {}", e),
            Error::GRPCStatus(e) => write!(f, "grep request error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<http::uri::InvalidUri> for Error {
    #[inline]
    fn from(e: http::uri::InvalidUri) -> Self {
        Error::InvalidUri(e)
    }
}

impl From<std::io::Error> for Error {
    #[inline]
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e)
    }
}

impl From<tonic::transport::Error> for Error {
    #[inline]
    fn from(e: tonic::transport::Error) -> Self {
        Error::TransportError(e)
    }
}

impl From<tonic::Status> for Error {
    #[inline]
    fn from(e: tonic::Status) -> Self {
        Error::GRPCStatus(e)
    }
}
