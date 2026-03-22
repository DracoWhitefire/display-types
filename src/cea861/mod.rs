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

/// VESA Display Transfer Characteristic Data Block types (standard tag `0x05`).
pub mod vesa_transfer;

/// HDMI Forum Sink Capability Data Block types (extended tags `0x78`, `0x79`).
pub mod hdmi_forum;

/// DisplayID Type VII/VIII/X Video Timing Data Block types and VESA VTB-EXT.
pub mod vtdb;

/// Misc block types: `InfoFrameDescriptor`, `VendorSpecificBlock`, `infoframe_type`.
pub mod misc;

/// VESA Display Device Data Block types (extended tag `0x02`).
pub mod vesa_dddb;

pub use audio::{AudioFormat, AudioFormatInfo, AudioSampleRates, ShortAudioDescriptor};
pub use colorimetry::{ColorimetryBlock, ColorimetryFlags};
pub use hdmi_forum::{HdmiDscMaxSlices, HdmiForumDsc, HdmiForumFrl, HdmiForumSinkCap};
pub use hdmi_vsdb::{HdmiVsdb, HdmiVsdbFlags};
pub use hdr::{HdrDynamicMetadataDescriptor, HdrEotf, HdrStaticMetadata};
#[cfg(any(feature = "alloc", feature = "std"))]
pub use misc::VendorSpecificBlock;
pub use misc::{InfoFrameDescriptor, infoframe_type};
pub use speaker::{
    RoomConfigurationBlock, SpeakerAllocation, SpeakerAllocationFlags, SpeakerAllocationFlags2,
    SpeakerAllocationFlags3, SpeakerLocationEntry,
};
pub use vesa_dddb::VesaDisplayDeviceBlock;
pub use vesa_transfer::DtcPointEncoding;
#[cfg(any(feature = "alloc", feature = "std"))]
pub use vesa_transfer::VesaTransferCharacteristic;
pub use video_capability::{VideoCapability, VideoCapabilityFlags};
pub use vtdb::{T7VtdbBlock, T10VtdbEntry};
#[cfg(any(feature = "alloc", feature = "std"))]
pub use vtdb::{T8VtdbBlock, T10VtdbBlock, VtbExtBlock};
