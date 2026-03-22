//! Types decoded from CEA-861 / CTA-861 extension blocks.

/// Audio descriptor types decoded from CEA Audio Data Blocks (tag `0x01`).
pub mod audio;

pub use audio::{AudioFormat, AudioFormatInfo, AudioSampleRates, ShortAudioDescriptor};
