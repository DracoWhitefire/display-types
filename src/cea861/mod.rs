//! Types decoded from CEA-861 / CTA-861 extension blocks.

/// Audio descriptor types decoded from CEA Audio Data Blocks (tag `0x01`).
pub mod audio;

/// HDMI 1.x Vendor-Specific Data Block types (OUI `0x000C03`).
pub mod hdmi_vsdb;

pub use audio::{AudioFormat, AudioFormatInfo, AudioSampleRates, ShortAudioDescriptor};
pub use hdmi_vsdb::{HdmiVsdb, HdmiVsdbFlags};
