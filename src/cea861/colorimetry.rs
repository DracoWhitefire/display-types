bitflags::bitflags! {
    /// Colorimetry standards supported by the display, from the Colorimetry
    /// Data Block (extended tag `0x05`).
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ColorimetryFlags: u8 {
        /// xvYCC 601.
        const XVYCC601   = 0x01;
        /// xvYCC 709.
        const XVYCC709   = 0x02;
        /// sYCC 601.
        const SYCC601    = 0x04;
        /// opYCC 601.
        const OPYCC601   = 0x08;
        /// opRGB.
        const OPRGB      = 0x10;
        /// BT.2020 cYCC.
        const BT2020CYCC = 0x20;
        /// BT.2020 YCC.
        const BT2020YCC  = 0x40;
        /// BT.2020 RGB.
        const BT2020RGB  = 0x80;
    }
}

/// Decoded Colorimetry Data Block (extended tag `0x05`).
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorimetryBlock {
    /// Supported colorimetry standards.
    pub colorimetry: ColorimetryFlags,
    /// Gamut metadata profile support bitmap (bits 3–0 of byte 2).
    pub metadata_profiles: u8,
}

impl ColorimetryBlock {
    /// Constructs a `ColorimetryBlock`.
    pub fn new(colorimetry: ColorimetryFlags, metadata_profiles: u8) -> Self {
        Self {
            colorimetry,
            metadata_profiles,
        }
    }
}
