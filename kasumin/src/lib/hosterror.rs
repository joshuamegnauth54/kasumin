use std::fmt::{self, Display, Formatter};

use cpal::{
    BackendSpecificError, DevicesError, HostId, HostUnavailable, SupportedStreamConfigsError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HostErrorKind {
    #[error("{0}")]
    Backend(#[from] BackendSpecificError),
    #[error("{0}")]
    Devices(#[from] DevicesError),
    #[error("{0}")]
    HostUnavailable(#[from] HostUnavailable),
    #[error("{0}")]
    StreamConfig(#[from] SupportedStreamConfigsError),
}

#[derive(Debug, Error)]
pub struct HostError {
    error: HostErrorKind,
    hostid: Option<HostId>,
    device_name: Option<String>,
}

impl HostError {
    #[inline(always)]
    pub(crate) fn new(
        error: HostErrorKind,
        hostid: Option<HostId>,
        device_name: Option<String>,
    ) -> Self {
        Self {
            error,
            hostid,
            device_name,
        }
    }
}

impl Display for HostError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match (
            self.hostid.as_ref().map(HostId::name),
            self.device_name.as_deref(),
        ) {
            (Some(hostid), Some(device_name)) => {
                write!(f, "Device `{device_name}` via `{hostid}: {}", self.error)
            }
            (Some(hostid), None) => write!(f, "Unknown device via `{hostid}`: {}", self.error),
            // This condition should be impossible. Devices are attached to hosts.
            (None, Some(device_name)) => write!(
                f,
                "Device `{device_name} on an unknown host: {}",
                self.error
            ),
            _ => write!(f, "{}", self.error),
        }
    }
}

/// [From] implementation where [cpal::HostId] and [cpal::Device] aren't
/// available.
impl From<HostErrorKind> for HostError {
    #[inline(always)]
    fn from(error: HostErrorKind) -> Self {
        HostError::new(error, None, None)
    }
}
