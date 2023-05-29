//! Errors may occur from the server or clients.

use rmp_serde::decode::Error as DecodeError;
use std::{
    io::Error as IoError,
    num::{NonZeroU32, ParseIntError},
    str::Utf8Error,
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
    InvalidSize(i32),
    // #[error("Envelope contains invalid characters: {0}")]
    // InvalidString(#[from] Utf8Error),
    #[error("Envelope is incorrectly formatted: {0}")]
    WrongFormat(String),
    #[error("Data length should be > 0 instead of: {0}")]
    ZeroOrNegative(i32),
}

#[derive(Debug, Error)]
pub enum ServerErrorKind {
    #[error("Deserializing MessagePack: `{0}`")]
    Decode(#[from] DecodeError),
    #[error("{0}")]
    Envelope(EnvelopeError),
    #[error("Network i/o error: `{0}")]
    Network(#[from] IoError),
}
