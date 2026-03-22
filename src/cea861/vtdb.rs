use crate::VideoMode;
#[cfg(any(feature = "alloc", feature = "std"))]
use crate::prelude::Vec;

/// A decoded DisplayID Type VII Video Timing Data Block (T7VTDB, extended tag `0x22`).
///
/// Each CTA T7VTDB carries exactly one 20-byte DisplayID-style timing descriptor.
/// Unlike an 18-byte EDID DTD, the pixel clock is expressed in kHz (not 10 kHz units)
/// and all horizontal/vertical fields are 16-bit rather than packed.
///
/// Multiple T7VTDBs are permitted per CTA extension block (one timing per block).
/// Per CTA-861, `interlaced` shall always be `false`; `y420` reflects the T7Y420 flag.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct T7VtdbBlock {
    /// Block revision (`Block_Rev` field, bits 2:0 of the descriptor header byte).
    /// CTA-861 expects revision `0x02`.
    pub version: u8,
    /// Decoded video timing for this descriptor.
    pub mode: VideoMode,
    /// When `true`, this timing also supports YCbCr 4:2:0 sampling (T7Y420 flag).
    pub y420: bool,
}

impl T7VtdbBlock {
    /// Constructs a `T7VtdbBlock`.
    pub fn new(version: u8, mode: VideoMode, y420: bool) -> Self {
        Self {
            version,
            mode,
            y420,
        }
    }
}

/// A decoded DisplayID Type VIII Video Timing Data Block (T8VTDB, extended tag `0x23`).
///
/// Contains a list of VESA DMT (Display Monitor Timings) ID codes referencing
/// standardised monitor timings. Only `Code_Type = 0x00` (DMT) is defined by
/// CTA-861; other code types are returned as `None` by the parser.
///
/// Codes whose DMT IDs are not in the standard table are stored in `codes` but
/// omitted from `timings`.
#[non_exhaustive]
#[cfg(any(feature = "alloc", feature = "std"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct T8VtdbBlock {
    /// Block revision (`Block_Rev` field, bits 2:0).
    pub version: u8,
    /// When `true`, all timings also support YCbCr 4:2:0 sampling (T8Y420 flag).
    pub y420: bool,
    /// Raw DMT timing codes as they appear in the block (1-byte or 2-byte each).
    pub codes: Vec<u16>,
    /// `VideoMode` values resolved from the DMT codes. Entries for unrecognised
    /// or reserved DMT IDs are omitted.
    pub timings: Vec<VideoMode>,
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl T8VtdbBlock {
    /// Constructs a `T8VtdbBlock`.
    pub fn new(version: u8, y420: bool, codes: Vec<u16>, timings: Vec<VideoMode>) -> Self {
        Self {
            version,
            y420,
            codes,
            timings,
        }
    }
}

/// A single timing entry from a T10VTDB block.
///
/// Type X timings use a CVT formula to derive the full signal, but only the
/// display-facing parameters (resolution and refresh rate) are exposed here.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct T10VtdbEntry {
    /// Horizontal active pixels.
    pub width: u16,
    /// Vertical active lines.
    pub height: u16,
    /// Vertical refresh rate in Hz (1–1024).
    ///
    /// Values above 255 are only possible when the block uses 7- or 8-byte
    /// descriptors (M ≥ 1 in the `rev` byte).
    pub refresh_hz: u16,
    /// When `true`, this timing also supports YCbCr 4:2:0 sampling (YCC420 flag).
    pub y420: bool,
}

impl T10VtdbEntry {
    /// Constructs a `T10VtdbEntry`.
    pub fn new(width: u16, height: u16, refresh_hz: u16, y420: bool) -> Self {
        Self {
            width,
            height,
            refresh_hz,
            y420,
        }
    }
}

/// A decoded DisplayID Type X Video Timing Data Block (T10VTDB, extended tag `0x2A`).
///
/// Type X timings are CVT formula-based: each descriptor encodes the active
/// resolution and refresh rate directly, with blanking derived by the display.
/// A block may contain 1–4 descriptors (limited by the 30-byte CTA extended
/// block payload cap).
#[non_exhaustive]
#[cfg(any(feature = "alloc", feature = "std"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct T10VtdbBlock {
    /// Decoded timing entries. Each entry corresponds to one descriptor in the block.
    pub entries: Vec<T10VtdbEntry>,
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl T10VtdbBlock {
    /// Constructs a `T10VtdbBlock`.
    pub fn new(entries: Vec<T10VtdbEntry>) -> Self {
        Self { entries }
    }
}

/// Decoded VESA Video Timing Block Extension (extended tag `0x03`).
///
/// Carries additional video timing modes beyond what fits in the base EDID block.
/// Each block may contain Detailed Timing Descriptors (DTBs), Coordinated Video
/// Timings (CVTs), and Standard Timing (ST) entries, per the VESA VTB-EXT Standard,
/// Release A.
#[non_exhaustive]
#[cfg(any(feature = "alloc", feature = "std"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct VtbExtBlock {
    /// VTB-EXT version byte (expected `0x01`).
    pub version: u8,
    /// All video modes decoded from this block (DTBs, CVTs, and STs combined).
    pub timings: Vec<VideoMode>,
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl VtbExtBlock {
    /// Constructs a `VtbExtBlock`.
    pub fn new(version: u8, timings: Vec<VideoMode>) -> Self {
        Self { version, timings }
    }
}
