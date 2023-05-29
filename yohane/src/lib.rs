//! Yohane implements a simple protocol to control playback for a Kasumin
//! instance.

pub mod deviceconfig;
pub mod query;
pub mod connect;

use query::{QueryRequest, QueryResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct KasuminRequest {
    pub uuid: String,
    pub message: RequestKind
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KasuminResponse {
    pub message: ResponseKind
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum RequestKind {
    Query(QueryRequest),
}

#[derive(Clone, Deserialize, Serialize)]
pub enum KasuminResponse {
    Query(QueryResponse),
}
