//! Configuration type that is mostly analogous to [cpal::SupportedStreamConfigRange](https://docs.rs/cpal/latest/cpal/struct.SupportedStreamConfigRange.html)
//! but allows avoiding a dependency for a few structs and enums. N.b these
//! types were essentially copied straight from cpal.

use serde::{Deserialize, Serialize};

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
    pub sample_format: SampleFormat
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
