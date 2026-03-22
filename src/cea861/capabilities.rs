#[cfg(any(feature = "alloc", feature = "std"))]
use crate::prelude::Vec;

#[cfg(any(feature = "alloc", feature = "std"))]
use super::{
    ColorimetryBlock, HdmiForumSinkCap, HdmiVsdb, HdrDynamicMetadataDescriptor, HdrStaticMetadata,
    InfoFrameDescriptor, RoomConfigurationBlock, ShortAudioDescriptor, SpeakerAllocation,
    SpeakerLocationEntry, T7VtdbBlock, T8VtdbBlock, T10VtdbBlock, VendorSpecificBlock,
    VesaDisplayDeviceBlock, VesaTransferCharacteristic, VideoCapability, VtbExtBlock,
};

bitflags::bitflags! {
    /// Capability flags from byte 3 of a CEA-861 extension block.
    ///
    /// | Bit | Mask   | Meaning                  |
    /// |-----|--------|--------------------------|
    /// | 7   | `0x80` | Underscan support        |
    /// | 6   | `0x40` | Basic audio support      |
    /// | 5   | `0x20` | YCbCr 4:4:4 support      |
    /// | 4   | `0x10` | YCbCr 4:2:2 support      |
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Cea861Flags: u8 {
        /// The display supports underscan.
        const UNDERSCAN   = 0x80;
        /// The display supports basic audio.
        const BASIC_AUDIO = 0x40;
        /// The display supports YCbCr 4:4:4 color encoding.
        const YCBCR_444   = 0x20;
        /// The display supports YCbCr 4:2:2 color encoding.
        const YCBCR_422   = 0x10;
    }
}

/// Decoded HDMI Audio Data Block (extended tag `0x12`).
///
/// Advertises HDMI-specific audio capabilities, including Multi-Stream Audio (MSA)
/// support and a set of Short Audio Descriptors for the HDMI audio path.
#[non_exhaustive]
#[cfg(any(feature = "alloc", feature = "std"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HdmiAudioBlock {
    /// If `true`, the sink supports HDMI Multi-Stream Audio (MSA).
    pub multi_stream_audio: bool,
    /// Short Audio Descriptors for the HDMI audio path (eARC-capable formats, etc.).
    pub audio_descriptors: Vec<ShortAudioDescriptor>,
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl HdmiAudioBlock {
    /// Constructs an `HdmiAudioBlock`.
    pub fn new(multi_stream_audio: bool, audio_descriptors: Vec<ShortAudioDescriptor>) -> Self {
        Self {
            multi_stream_audio,
            audio_descriptors,
        }
    }
}

/// Decoded capabilities from a CEA-861 extension block.
///
/// Stored in `DisplayCapabilities::extension_data` under tag `0x02` by the CEA-861
/// extension handler. Retrieve it with `caps.get_extension_data::<Cea861Capabilities>(0x02)`.
#[non_exhaustive]
#[cfg(any(feature = "alloc", feature = "std"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)] // no Eq: HdrStaticMetadata contains f32
pub struct Cea861Capabilities {
    /// Capability flags from byte 3 of the CEA-861 header.
    pub flags: Cea861Flags,
    /// Short Video Descriptors from the CEA Video Data Block (tag `0x02`).
    ///
    /// Each entry is `(vic_number, is_native)`. VICs beyond the range of the
    /// built-in lookup table are included here but do not produce an entry in
    /// `DisplayCapabilities::supported_modes`.
    pub vics: Vec<(u8, bool)>,
    /// Short Audio Descriptors from the CEA Audio Data Block (tag `0x01`).
    pub audio_descriptors: Vec<ShortAudioDescriptor>,
    /// Decoded HDMI 1.x Vendor-Specific Data Block (OUI `0x000C03`), if present.
    pub hdmi_vsdb: Option<HdmiVsdb>,
    /// Decoded HDMI Forum Vendor-Specific Data Block (OUI `0xC45DD8`), if present.
    ///
    /// Carries the same HDMI Forum Sink Capability Data Structure (SCDS) as
    /// `hf_scdb`. Typically found on HDMI 2.0 sinks; HDMI 2.1 sinks
    /// more commonly use the HF-SCDB (extended tag `0x79`) instead.
    pub hf_vsdb: Option<HdmiForumSinkCap>,
    /// Decoded Video Capability Data Block (extended tag `0x00`), if present.
    pub video_capability: Option<VideoCapability>,
    /// Decoded Colorimetry Data Block (extended tag `0x05`), if present.
    pub colorimetry: Option<ColorimetryBlock>,
    /// Decoded HDR Static Metadata Data Block (extended tag `0x06`), if present.
    pub hdr_static_metadata: Option<HdrStaticMetadata>,
    /// Decoded HDMI Audio Data Block (extended tag `0x12`), if present.
    pub hdmi_audio: Option<HdmiAudioBlock>,
    /// InfoFrame descriptors from the InfoFrame Data Block (extended tag `0x20`).
    pub infoframe_descriptors: Vec<InfoFrameDescriptor>,
    /// Decoded Room Configuration Data Block (extended tag `0x13`), if present.
    pub room_configuration: Option<RoomConfigurationBlock>,
    /// Speaker location entries from the Speaker Location Data Block (extended tag `0x14`).
    pub speaker_locations: Vec<SpeakerLocationEntry>,
    /// Decoded VESA Display Transfer Characteristic Data Block (standard tag `0x05`), if present.
    pub vesa_transfer_characteristic: Option<VesaTransferCharacteristic>,
    /// Decoded Speaker Allocation Data Block (standard tag `0x04`), if present.
    pub speaker_allocation: Option<SpeakerAllocation>,
    /// HDR Dynamic Metadata application descriptors (extended tag `0x07`).
    pub hdr_dynamic_metadata: Vec<HdrDynamicMetadataDescriptor>,
    /// Raw Short Video References from the Video Format Preference Data Block
    /// (extended tag `0x0D`), if present.
    pub video_format_preferences: Vec<u8>,
    /// VICs from the YCbCr 4:2:0 Video Data Block (extended tag `0x0E`).
    pub y420_vics: Vec<u8>,
    /// Raw capability bitmap from the YCbCr 4:2:0 Capability Map Data Block
    /// (extended tag `0x0F`).
    pub y420_capability_map: Vec<u8>,
    /// Decoded VESA Display Device Data Block (extended tag `0x02`), if present.
    pub vesa_display_device: Option<VesaDisplayDeviceBlock>,
    /// Additional timing modes from VESA Video Timing Block Extension blocks
    /// (extended tag `0x03`).
    pub vtb_ext: Vec<VtbExtBlock>,
    /// Vendor-Specific Video Data Blocks (extended tag `0x01`).
    pub vendor_specific_video: Vec<VendorSpecificBlock>,
    /// Vendor-Specific Audio Data Blocks (extended tag `0x11`).
    pub vendor_specific_audio: Vec<VendorSpecificBlock>,
    /// DisplayID Type VII Video Timing Data Blocks (extended tag `0x22`).
    pub t7_vtdb: Vec<T7VtdbBlock>,
    /// DisplayID Type VIII Video Timing Data Blocks (extended tag `0x23`).
    pub t8_vtdb: Vec<T8VtdbBlock>,
    /// DisplayID Type X Video Timing Data Blocks (extended tag `0x2A`).
    pub t10_vtdb: Vec<T10VtdbBlock>,
    /// Extension block count from the HDMI Forum EDID Extension Override Data Block
    /// (extended tag `0x78`), if present.
    pub hf_eeodb_extension_count: Option<u8>,
    /// HDMI Forum Sink Capability Data Block (extended tag `0x79`), if present.
    pub hf_scdb: Option<HdmiForumSinkCap>,
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl Cea861Capabilities {
    /// Constructs a zeroed `Cea861Capabilities` with the given header flags.
    ///
    /// All `Vec` fields are initialized empty and all `Option` fields are `None`.
    /// Callers populate the fields after construction.
    pub fn new(flags: Cea861Flags) -> Self {
        Self {
            flags,
            vics: Vec::new(),
            audio_descriptors: Vec::new(),
            hdmi_vsdb: None,
            hf_vsdb: None,
            video_capability: None,
            colorimetry: None,
            hdr_static_metadata: None,
            hdmi_audio: None,
            infoframe_descriptors: Vec::new(),
            room_configuration: None,
            speaker_locations: Vec::new(),
            vesa_transfer_characteristic: None,
            speaker_allocation: None,
            hdr_dynamic_metadata: Vec::new(),
            video_format_preferences: Vec::new(),
            y420_vics: Vec::new(),
            y420_capability_map: Vec::new(),
            vesa_display_device: None,
            vtb_ext: Vec::new(),
            vendor_specific_video: Vec::new(),
            vendor_specific_audio: Vec::new(),
            t7_vtdb: Vec::new(),
            t8_vtdb: Vec::new(),
            t10_vtdb: Vec::new(),
            hf_eeodb_extension_count: None,
            hf_scdb: None,
        }
    }
}
