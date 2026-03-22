//! Well-known IEEE OUI values in canonical (MSB-first) `u32` form.
//!
//! These match the `oui` field of `VendorSpecificBlock`, which stores
//! OUI bytes as `(byte2 << 16) | (byte1 << 8) | byte0` (wire order is LSB-first).
//!
//! Use these constants to identify vendor-specific blocks by OUI:
//!
//! ```rust
//! # use display_types::cea861::{VendorSpecificBlock, oui};
//! # let block = VendorSpecificBlock::new(oui::DOLBY_VISION, vec![]);
//! if block.oui == oui::DOLBY_VISION {
//!     // parse Dolby Vision metadata from block.payload
//! }
//! ```

/// HDMI Licensing, LLC — used in HDMI 1.x Vendor-Specific Data Blocks.
pub const HDMI_LICENSING: u32 = 0x000C03;

/// HDMI Forum — used in HDMI Forum Vendor-Specific Data Blocks (HF-VSDB).
pub const HDMI_FORUM: u32 = 0xC45DD8;

/// Dolby Laboratories — used in Dolby Vision Vendor-Specific Video Data Blocks.
pub const DOLBY_VISION: u32 = 0x00D046;

/// Samsung Electronics / HDR10+ Technology — used in HDR10+ Vendor-Specific Video Data Blocks.
pub const HDR10_PLUS: u32 = 0x90848B;
