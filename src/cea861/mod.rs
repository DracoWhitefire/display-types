//! Types decoded from CEA-861 / CTA-861 extension blocks.

/// Audio descriptor types decoded from CEA Audio Data Blocks (tag `0x01`).
pub mod audio;

/// HDMI 1.x Vendor-Specific Data Block types (OUI `0x000C03`).
pub mod hdmi_vsdb;

/// Video Capability Data Block types (extended tag `0x00`).
pub mod video_capability;

/// Colorimetry Data Block types (extended tag `0x05`).
pub mod colorimetry;

/// HDR Static and Dynamic Metadata Data Block types (extended tags `0x06`, `0x07`).
pub mod hdr;

/// Speaker Allocation, Room Configuration, and Speaker Location types.
pub mod speaker;

pub use audio::{AudioFormat, AudioFormatInfo, AudioSampleRates, ShortAudioDescriptor};
pub use colorimetry::{ColorimetryBlock, ColorimetryFlags};
pub use hdmi_vsdb::{HdmiVsdb, HdmiVsdbFlags};
pub use hdr::{HdrDynamicMetadataDescriptor, HdrEotf, HdrStaticMetadata};
pub use speaker::{
    RoomConfigurationBlock, SpeakerAllocation, SpeakerAllocationFlags, SpeakerAllocationFlags2,
    SpeakerAllocationFlags3, SpeakerLocationEntry,
};
pub use video_capability::{VideoCapability, VideoCapabilityFlags};
