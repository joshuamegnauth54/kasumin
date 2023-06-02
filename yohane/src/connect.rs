//! Connection request types

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ConnectRequest {
    Name(String)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ConnectResponse {
    Uuid(String)
}
