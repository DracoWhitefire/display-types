//! DisplayID 1.x and 2.x data block tag constants.
//!
//! Each constant corresponds to a data block type defined in the DisplayID specification.
//! Tags are the first byte of the 3-byte data block header.

/// Product Identification Block (DisplayID 1.x §4.2).
pub const PRODUCT_ID: u8 = 0x00;

/// Display Parameters Block (DisplayID 1.x §4.3).
pub const DISPLAY_PARAMS: u8 = 0x01;

/// Color Characteristics Block (DisplayID 1.x §4.4).
pub const COLOR_CHARACTERISTICS: u8 = 0x02;

/// Video Timing Modes Type I — Detailed Timings Block (DisplayID 1.x §4.4.2).
pub const TYPE_I_TIMING: u8 = 0x03;

/// Video Timing Modes Type II — Detailed Timings Block (DisplayID 1.x §4.4.3).
pub const TYPE_II_TIMING: u8 = 0x04;

/// Video Timing Modes Type III — Short Timings Block (DisplayID 1.x §4.4.4).
pub const TYPE_III_TIMING: u8 = 0x05;

/// Video Timing Modes Type IV — DMT/VIC Code Block (DisplayID 1.x §4.4.5).
pub const TYPE_IV_TIMING: u8 = 0x06;

/// VESA Video Timing Block — DMT presence bitmap (DisplayID 1.x §4.4.6).
///
/// Payload: up to 10 bytes encoding DMT IDs 0x01–0x50.
/// Bit `i` (0-indexed, LSB-first within each byte) is set if DMT ID `i + 1` is supported.
pub const VESA_VIDEO_TIMING: u8 = 0x07;

/// CTA-861 Video Timing Block — VIC presence bitmap (DisplayID 1.x §4.4.7).
///
/// Payload: up to 8 bytes encoding CTA-861 VIC codes 1–64.
/// Bit `i` (0-indexed, LSB-first within each byte) is set if VIC `i + 1` is supported.
pub const CTA_VIDEO_TIMING: u8 = 0x08;

/// Video Timing Range Limits Block (DisplayID 1.x §4.5).
pub const VIDEO_TIMING_RANGE: u8 = 0x09;

/// Product Serial Number Block (DisplayID 1.x §4.8).
pub const SERIAL_NUMBER: u8 = 0x0A;

/// General Purpose ASCII String Block (DisplayID 1.x §4.9).
pub const ASCII_STRING: u8 = 0x0B;

/// Display Device Data Block (DisplayID 1.x §4.10).
pub const DISPLAY_DEVICE_DATA: u8 = 0x0C;

/// Interface Power Sequencing Block (DisplayID 1.x §4.11).
pub const POWER_SEQUENCING: u8 = 0x0D;

/// Transfer Characteristics Block (DisplayID 1.x §4.12).
pub const TRANSFER_CHARACTERISTICS: u8 = 0x0E;

/// Display Interface Data Block (DisplayID 1.x §4.13).
pub const DISPLAY_INTERFACE: u8 = 0x0F;

/// Stereo Display Interface Data Block (DisplayID 1.x §4.14).
pub const STEREO_DISPLAY_INTERFACE: u8 = 0x10;

/// Video Timing Modes Type V — Short Timings Block (DisplayID 1.x §4.6).
pub const TYPE_V_TIMING: u8 = 0x11;

/// Tiled Display Topology Data Block (DisplayID 1.x §4.15).
pub const TILED_TOPOLOGY: u8 = 0x12;

/// Video Timing Modes Type VI — Detailed Timings Block (DisplayID 1.x §4.7).
pub const TYPE_VI_TIMING: u8 = 0x13;

// ── DisplayID 2.x tag constants ──────────────────────────────────────────────

/// Product Identification Block (DisplayID 2.x §4.2).
pub const V2_PRODUCT_ID: u8 = 0x20;

/// Display Parameters Block (DisplayID 2.x §4.3).
pub const V2_DISPLAY_PARAMS: u8 = 0x21;

/// Video Timing Modes Type VII — Detailed Timings Block (DisplayID 2.x §4.4.2).
pub const V2_TYPE_VII_TIMING: u8 = 0x22;

/// Video Timing Modes Type VIII — Enumerated Timing Code Block (DisplayID 2.x §4.4.3).
pub const V2_TYPE_VIII_TIMING: u8 = 0x23;

/// Video Timing Modes Type IX — Formula-Based Timings Block (DisplayID 2.x §4.4.4).
pub const V2_TYPE_IX_TIMING: u8 = 0x24;

/// Dynamic Video Timing Range Limits Block (DisplayID 2.x §4.5).
pub const V2_DYNAMIC_TIMING_RANGE: u8 = 0x25;

/// Display Interface Features Block (DisplayID 2.x §4.6).
pub const V2_INTERFACE_FEATURES: u8 = 0x26;

/// Stereo Display Interface Block (DisplayID 2.x §4.7).
pub const V2_STEREO_INTERFACE: u8 = 0x27;

/// Tiled Display Topology Block (DisplayID 2.x §4.8).
pub const V2_TILED_TOPOLOGY: u8 = 0x28;

/// ContainerID Block (DisplayID 2.x §4.9).
pub const V2_CONTAINER_ID: u8 = 0x29;

/// Vendor-Specific Block (DisplayID 2.x §4.10).
pub const V2_VENDOR_SPECIFIC: u8 = 0x7E;

/// CTA DisplayID Block (DisplayID 2.x §4.11).
pub const V2_CTA_DISPLAYID: u8 = 0x81;
