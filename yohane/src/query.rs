//! Query requests and responses for the Kasumin daemon.

use super::deviceconfig::SupportedDeviceConfig;
use serde::{Deserialize, Serialize};

/// Information on a single output device supported by the server.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SupportedOutputDevice {
    pub host: String,
    pub device: String,
    pub stream_configs: Vec<SupportedDeviceConfig>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum QueryRequest {
    OutputDevices,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum QueryResponse {
    OutputDevices(Vec<SupportedOutputDevice>),
}
