use crate::hosterror::HostError;
use cpal::{traits::HostTrait, Device, Devices, HostId, OutputDevices};
use std::{
    iter::{FusedIterator, Peekable},
    slice,
};

/// Container for [HostId] and an associated output [Device].
pub struct HostDevicePair {
    /// API that exposes the current [Device], such as ALSA.
    pub hostid: HostId,
    /// An output device as exposed by the [Host].
    pub device: Device,
}

/// Iterator over all hosts and all output devices.
pub struct AllHostsDevices<'host> {
    hostids: slice::Iter<'host, HostId>,
    current_host: Option<HostId>,
    devices: Option<Peekable<OutputDevices<Devices>>>,
}

impl AllHostsDevices<'_> {
    #[inline]
    pub fn iter() -> Self {
        Self {
            hostids: cpal::ALL_HOSTS.iter(),
            current_host: None,
            devices: None,
        }
        /*Self {
            hostids: cpal::ALL_HOSTS.into_iter().map(|&hostid| {
                let host = cpal::host_from_id(hostid)
                    .map_err(|kind| HostError::new(kind.into(), Some(hostid), None))?;

                Ok(host
                    .output_devices()
                    .map_err(|e| HostError::new(e.into(), Some(hostid), None))?
                    .map(|device| HostDevicePair { hostid, device }))
            }),
        }*/
    }
}

impl Iterator for AllHostsDevices<'_> {
    type Item = Result<HostDevicePair, HostError>;

    fn next(&mut self) -> Option<Self::Item> {
        // This code looks kind of finicky but it's straightforward.
        // I'll replace it with better iterator shenanigans whenever I figure out how to
        // store the long, nested type without fighting with the borrow checker.

        // current_host and devices should either both be Some or both None
        // If that's the case, get the next device. If the iterator is exhausted then
        // try to replace it.
        if let (Some(device), Some(hostid)) = (
            self.devices.as_mut().and_then(|devices| devices.next()),
            self.current_host,
        ) {
            Some(Ok(HostDevicePair { hostid, device }))
        }
        // Do I have a Devices iterator? Is it exhausted?
        else if self.devices.is_none()
            || self
                .devices
                .as_mut()
                .map_or(true, |devices| devices.peek().is_none())
        {
            // Short circuit the iterator here if hostids is exhausted
            let hostid = *self.hostids.next()?;
            self.current_host = Some(hostid);

            match cpal::host_from_id(hostid)
                .map_err(|e| HostError::new(e.into(), Some(hostid), None))
            {
                Ok(host) => {
                    match host
                        .output_devices()
                        .map(Iterator::peekable)
                        .map_err(|e| HostError::new(e.into(), Some(hostid), None))
                    {
                        Ok(devices) => {
                            self.devices = Some(devices);
                            // Recurse with the new hostid and Devices iterator
                            self.next()
                        }
                        Err(e) => Some(Err(e)),
                    }
                }
                Err(e) => Some(Err(e)),
            }
        } else {
            None
        }
    }
}

impl FusedIterator for AllHostsDevices<'_> {}

#[cfg(test)]
mod tests {
    use super::AllHostsDevices;
    use cpal::traits::{DeviceTrait, HostTrait};

    #[test]
    // Test if the iterator can be exhausted and doesn't cause a stack overflow.
    fn allhostsdevices_exhaust() {
        for _ in AllHostsDevices::iter() {}
    }

    #[test]
    fn allhostdevices_has_default_output() {
        let default_output = cpal::default_host()
            .default_output_device()
            .expect("Expected a default output device. Can't run this test.")
            .name()
            .expect("Expected output device to have a valid name.");

        let _ = AllHostsDevices::iter()
            .find(|host_device_res| {
                host_device_res.as_ref().map_or(false, |host_device| {
                    host_device
                        .device
                        .name()
                        .map_or(false, |name| name == default_output)
                })
            })
            .expect("The default output device should be yielded by the iterator.");
    }
}
