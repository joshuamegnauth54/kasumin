//! Yohane implements a simple protocol to control playback for a Kasumin
//! instance.

pub mod connect;
pub mod deviceconfig;
pub mod query;

use connect::{ConnectRequest, ConnectResponse};
use query::{QueryRequest, QueryResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KasuminRequest {
    pub uuid: String,
    pub version: u16,
    pub message: RequestKind,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KasuminResponse {
    pub message: ResponseKind,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RequestKind {
    Query(QueryRequest),
    Connect(ConnectRequest),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ResponseKind {
    Ready,
    Query(QueryResponse),
    Connect(ConnectResponse),
}
