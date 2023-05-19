//! Yohane implements a simple protocol to control playback for a Kasumin
//! instance.

pub mod deviceconfig;
pub mod query;

use query::{QueryRequest, QueryResponse};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum KasuminRequest {
    Query(QueryRequest),
}

#[derive(Clone, Deserialize, Serialize)]
pub enum KasuminResponse {
    Query(QueryResponse),
}
