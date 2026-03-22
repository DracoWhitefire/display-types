bitflags::bitflags! {
    /// Electro-Optical Transfer Functions (EOTFs) supported by the display,
    /// from the HDR Static Metadata Data Block (extended tag `0x06`).
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct HdrEotf: u8 {
        /// Traditional gamma — SDR luminance range.
        const SDR    = 0x01;
        /// Traditional gamma — HDR luminance range.
        const HDR    = 0x02;
        /// SMPTE ST 2084 (PQ / HDR10).
        const ST2084 = 0x04;
        /// Hybrid Log-Gamma (HLG).
        const HLG    = 0x08;
    }
}

/// Decoded HDR Static Metadata Data Block (extended tag `0x06`).
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)] // no Eq: contains f32
pub struct HdrStaticMetadata {
    /// Supported EOTFs (tone-mapping curves).
    pub eotf: HdrEotf,
    /// Static Metadata Descriptor type bitmap (usually bit 0 = Type 1 / MaxCLL/MaxFALL).
    pub static_metadata_descriptors: u8,
    /// Desired content maximum luminance in cd/m², decoded from byte 3.
    ///
    /// Encoded as `50 × 2^(raw / 32)`. `None` when the byte is absent.
    pub max_luminance: Option<f32>,
    /// Desired content maximum frame-average light level (MaxFALL) in cd/m².
    ///
    /// Same encoding as `max_luminance`. `None` when absent.
    pub max_fall: Option<f32>,
    /// Desired content minimum luminance in cd/m², decoded from byte 5.
    ///
    /// Encoded as `max_luminance × (raw / 255)² / 100`. `None` when absent or when
    /// `max_luminance` is not present.
    pub min_luminance: Option<f32>,
}

/// One entry from an HDR Dynamic Metadata Data Block (extended tag `0x07`).
///
/// Each descriptor identifies the HDR dynamic metadata technology supported
/// (e.g. HDR10+ / SMPTE ST 2094, or Dolby Vision).
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HdrDynamicMetadataDescriptor {
    /// Application type identifier (bits 5–0 of the descriptor byte).
    ///
    /// `1` = SMPTE ST 2094 (HDR10+); `2` = Dolby Vision.
    pub application_type: u8,
    /// Application metadata version (bits 7–6 of the descriptor byte).
    pub application_version: u8,
}

impl HdrDynamicMetadataDescriptor {
    /// Constructs an `HdrDynamicMetadataDescriptor`.
    pub fn new(application_type: u8, application_version: u8) -> Self {
        Self {
            application_type,
            application_version,
        }
    }
}

impl HdrStaticMetadata {
    /// Constructs an `HdrStaticMetadata`.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        eotf: HdrEotf,
        static_metadata_descriptors: u8,
        max_luminance: Option<f32>,
        max_fall: Option<f32>,
        min_luminance: Option<f32>,
    ) -> Self {
        Self {
            eotf,
            static_metadata_descriptors,
            max_luminance,
            max_fall,
            min_luminance,
        }
    }
}
