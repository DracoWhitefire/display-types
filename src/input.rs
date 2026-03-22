bitflags::bitflags! {
    /// Boolean flags from EDID byte `0x14` (Video Input Definition).
    ///
    /// Bit 7 (`DIGITAL`) determines the input type. Bits 4–0 are only meaningful
    /// for analog displays. The multi-bit fields in this byte (color bit depth,
    /// video interface type, and analog sync level) are not represented here —
    /// those require dedicated enum types.
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VideoInputFlags: u8 {
        /// Digital input. When clear, the display uses an analog input interface.
        const DIGITAL          = 0x80;
        /// Blank-to-black setup (pedestal) expected (analog only).
        const BLANK_TO_BLACK   = 0x10;
        /// Separate sync signals are supported (analog only).
        const SEPARATE_SYNC    = 0x08;
        /// Composite sync on HSync is supported (analog only).
        const COMPOSITE_SYNC   = 0x04;
        /// Sync on green is supported (analog only).
        const SYNC_ON_GREEN    = 0x02;
        /// VSync pulse must be serrated when composite or sync-on-green is used (analog only).
        const SERRATION        = 0x01;
    }
}

/// Video white and sync levels for an analog display, decoded from EDID base block
/// byte `0x14` bits 6–5.
///
/// Specifies the signal voltage levels used for video white and sync, relative to blank.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalogSyncLevel {
    /// 0.700 V video / 0.300 V sync / 1.000 V total (most common).
    V700_300,
    /// 0.714 V video / 0.286 V sync / 1.000 V total (EGA/CGA-compatible).
    V714_286,
    /// 1.000 V video / 0.400 V sync / 1.400 V total.
    V1000_400,
    /// 0.700 V video / 0.000 V sync / 0.700 V total.
    V700_0,
}

/// Video interface type, decoded from EDID base block byte `0x14` bits 3–0.
///
/// Only valid for digital input displays. `None` is used for the undefined (0x0)
/// and reserved (0x6–0xF) values.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoInterface {
    /// DVI interface.
    Dvi,
    /// HDMI-a interface.
    HdmiA,
    /// HDMI-b interface.
    HdmiB,
    /// MDDI (Mobile Display Digital Interface).
    Mddi,
    /// DisplayPort interface.
    DisplayPort,
}
