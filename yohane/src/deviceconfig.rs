//! Configuration type that is mostly analogous to [cpal::SupportedStreamConfigRange](https://docs.rs/cpal/latest/cpal/struct.SupportedStreamConfigRange.html)
//! but allows avoiding a dependency for a few structs and enums. N.b these
//! types were essentially copied straight from cpal.

use serde::{Deserialize, Serialize};

#[cfg(feature = "from_cpal")]
use cpal::{BackendSpecificError, SupportedStreamConfigRange, SupportedStreamConfigsError};

/// Device configuration's supported sample rate.
pub type SampleRate = u32;

/// Device configuration's supported channel count.
pub type ChannelCount = u16;

/// Number of frames buffered.
pub type FrameCount = u32;

/// Device configuration's supported sample rate.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum SampleFormat {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
}

/// Minimum and maximum buffer size.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct BufferSize {
    pub min: FrameCount,
    pub max: FrameCount,
}

/// Supported device config.
///
/// This is a potentially usable device config for a stream.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct SupportedDeviceConfig {
    pub channels: ChannelCount,
    pub min_sample_rate: SampleRate,
    pub max_sample_rate: SampleRate,
    pub buffer_size: Option<BufferSize>,
    pub sample_format: SampleFormat,
}

/// Device config.
///
/// This is a response type for setting a specific stream configuration.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct DeviceConfig {
    pub channels: ChannelCount,
    pub buffer_size: BufferSize,
    pub sample_rate: SampleRate,
    pub sample_format: SampleFormat,
}

// From implementations for cpal types.
#[cfg(feature = "from_cpal")]
impl BufferSize {
    #[inline]
    pub fn from_cpal(buffer_size: &cpal::SupportedBufferSize) -> Option<Self> {
        if let &cpal::SupportedBufferSize::Range { min, max } = buffer_size {
            Some(Self { min, max })
        } else {
            None
        }
    }
}

#[cfg(feature = "from_cpal")]
impl TryFrom<cpal::SampleFormat> for SampleFormat {
    type Error = SupportedStreamConfigsError;

    #[inline]
    fn try_from(sample_format: cpal::SampleFormat) -> Result<Self, Self::Error> {
        match sample_format {
            cpal::SampleFormat::I8 => Ok(SampleFormat::I8),
            cpal::SampleFormat::I16 => Ok(SampleFormat::I16),
            cpal::SampleFormat::I32 => Ok(SampleFormat::I32),
            cpal::SampleFormat::I64 => Ok(SampleFormat::I64),
            cpal::SampleFormat::U8 => Ok(SampleFormat::U8),
            cpal::SampleFormat::U16 => Ok(SampleFormat::U16),
            cpal::SampleFormat::U32 => Ok(SampleFormat::U32),
            cpal::SampleFormat::U64 => Ok(SampleFormat::U64),
            cpal::SampleFormat::F32 => Ok(SampleFormat::F32),
            cpal::SampleFormat::F64 => Ok(SampleFormat::F64),
            _ => Err(SupportedStreamConfigsError::BackendSpecific {
                err: BackendSpecificError { description: format!("Unhandled cpal::SampleFormat enumerator `{sample_format}`. Report this as a Kasumin bug ASAP.") }
            }),
        }
    }
}

#[cfg(feature = "from_cpal")]
impl From<&SupportedStreamConfigRange> for SupportedDeviceConfig {
    #[inline]
    fn from(config: &SupportedStreamConfigRange) -> Self {
        Self {
            channels: config.channels(),
            min_sample_rate: config.min_sample_rate().0,
            max_sample_rate: config.max_sample_rate().0,
            buffer_size: BufferSize::from_cpal(config.buffer_size()),
            sample_format: config
                .sample_format()
                .try_into()
                .expect("All cpal::SampleFormat elements should be handled"),
        }
    }
}

#[cfg(feature = "from_cpal")]
impl From<SupportedStreamConfigRange> for SupportedDeviceConfig {
    #[inline(always)]
    fn from(config: SupportedStreamConfigRange) -> Self {
        config.into()
    }
}
