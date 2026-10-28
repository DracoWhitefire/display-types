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
/// Advertises HDMI Multi-Stream Audio and HDMI 3D Audio. Layout as decoded by edid-decode
/// (`cta_hdmi_audio_block`) and libdisplay-info (`parse_hdmi_audio_block`): byte 1 holds
/// the maximum stream count less one (bits 1:0) and MS NonMixed support (bit 2); byte 2
/// the number of 3D audio descriptors (bits 2:0); then that many 4-byte 3D audio
/// descriptors and, when there is at least one, a 4-byte 3D speaker allocation. The block
/// carries no ordinary Short Audio Descriptors. HDMI 2.2 deprecates 3D audio.
#[non_exhaustive]
#[cfg(any(feature = "alloc", feature = "std"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HdmiAudioBlock {
    /// The most audio streams the sink accepts at once: 1 without Multi-Stream Audio, 2–4
    /// with it.
    pub max_streams: u8,
    /// The sink accepts multi-stream audio whose streams are not mixed (MS NonMixed). Only
    /// meaningful when `max_streams > 1`.
    pub ms_non_mixed: bool,
    /// HDMI 3D Audio Descriptors: L-PCM or One Bit Audio, with up to 32 channels. They hold
    /// the same fields as a Short Audio Descriptor, packed in 4 bytes.
    pub audio_3d_descriptors: Vec<ShortAudioDescriptor>,
    /// The speaker allocation for 3D audio, present when there are 3D audio descriptors.
    pub speaker_allocation_3d: Option<Audio3dSpeakerAllocation>,
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl HdmiAudioBlock {
    /// Constructs an `HdmiAudioBlock`.
    pub fn new(
        max_streams: u8,
        ms_non_mixed: bool,
        audio_3d_descriptors: Vec<ShortAudioDescriptor>,
        speaker_allocation_3d: Option<Audio3dSpeakerAllocation>,
    ) -> Self {
        Self {
            max_streams,
            ms_non_mixed,
            audio_3d_descriptors,
            speaker_allocation_3d,
        }
    }

    /// The sink accepts more than one audio stream at once (HDMI Multi-Stream Audio).
    pub fn multi_stream(&self) -> bool {
        self.max_streams > 1
    }
}

/// The channel layout a 3D speaker allocation describes (byte 4, bits 7:4).
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Audio3dChannels {
    /// 10.2 channels.
    Ch10_2,
    /// 22.2 channels.
    Ch22_2,
    /// 30.2 channels.
    Ch30_2,
    /// A value no known layout uses; the raw 4-bit value.
    Unknown(u8),
}

/// The 3D speaker allocation descriptor that ends an HDMI Audio Data Block's 3D audio
/// descriptors: the speaker bits (as in the Speaker Allocation Data Block) and the layout.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Audio3dSpeakerAllocation {
    /// The channel layout.
    pub channels: Audio3dChannels,
    /// The speakers present.
    pub speakers: super::SpeakerAllocation,
}

impl Audio3dSpeakerAllocation {
    /// Constructs an `Audio3dSpeakerAllocation`.
    pub fn new(channels: Audio3dChannels, speakers: super::SpeakerAllocation) -> Self {
        Self { channels, speakers }
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

#[cfg(all(test, any(feature = "alloc", feature = "std")))]
mod hdmi_audio_tests {
    use super::*;
    use crate::cea861::{
        SpeakerAllocation, SpeakerAllocationFlags, SpeakerAllocationFlags2, SpeakerAllocationFlags3,
    };

    #[test]
    fn multi_stream_follows_the_stream_count() {
        assert!(!HdmiAudioBlock::new(1, false, Vec::new(), None).multi_stream());
        assert!(HdmiAudioBlock::new(2, true, Vec::new(), None).multi_stream());
    }

    #[test]
    fn speaker_allocation_3d_holds_layout_and_speakers() {
        let speakers = SpeakerAllocation::new(
            SpeakerAllocationFlags::FL_FR | SpeakerAllocationFlags::FC,
            SpeakerAllocationFlags2::empty(),
            SpeakerAllocationFlags3::empty(),
        );
        let alloc = Audio3dSpeakerAllocation::new(Audio3dChannels::Ch22_2, speakers);
        let block = HdmiAudioBlock::new(1, false, Vec::new(), Some(alloc));
        assert_eq!(
            block.speaker_allocation_3d.map(|a| a.channels),
            Some(Audio3dChannels::Ch22_2)
        );
    }
}
