//! Errors may occur from the server or clients.

use rmp_serde::decode::Error as DecodeError;
use std::{
    fmt::{self, Display, Formatter},
    io::Error as IoError,
    num::{NonZeroU32, ParseIntError},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub struct ServerError {
    client_name: Option<String>,
    client_id: Option<String>,
    kind: ServerErrorKind,
}

#[derive(Debug, Error)]
pub enum EnvelopeError {
    #[error("Data length is too large: {data_length} > {limit}")]
    DataTooLarge {
        limit: NonZeroU32,
        data_length: NonZeroU32,
    },
    #[error("Retrieving envelope: {0}")]
    Network(#[from] IoError),
    #[error("Envelope contains an invalid length field: {0}")]
    InvalidNum(#[from] ParseIntError),
    #[error("Envelope length too small: {0}")]
    InvalidSize(usize),
    // #[error("Envelope contains invalid characters: {0}")]
    // InvalidString(#[from] Utf8Error),
    #[error("Envelope is incorrectly formatted: {0}")]
    WrongFormat(String),
    #[error("Data length should be > 0 instead of: {0}")]
    Zero(u32),
}

#[derive(Debug, Error)]
pub enum ServerErrorKind {
    #[error("Deserializing MessagePack: `{0}`")]
    Decode(#[from] DecodeError),
    #[error("{0}")]
    Envelope(#[from] EnvelopeError),
    #[error("Network i/o error: `{0}")]
    Network(#[from] IoError),
}

impl ServerError {
    #[inline]
    pub fn new(kind: ServerErrorKind, client_name: Option<&str>, client_id: Option<&str>) -> Self {
        Self {
            kind,
            client_name: client_name.map(ToOwned::to_owned),
            client_id: client_id.map(ToOwned::to_owned)
        }
    }
}

impl Display for ServerError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}) => {}",
            self.client_name.as_deref().unwrap_or("N/A"),
            self.client_id.as_deref().unwrap_or("N/A"),
            self.kind
        )
    }
}
