#[cfg(any(feature = "alloc", feature = "std"))]
use crate::prelude::Vec;

/// Well-known InfoFrame type codes from the InfoFrame Data Block (extended tag `0x20`).
///
/// These correspond to the InfoFrame types defined in HDMI and CTA-861.
pub mod infoframe_type {
    /// Vendor-Specific InfoFrame (VSI). The associated OUI identifies the vendor.
    pub const VENDOR_SPECIFIC: u8 = 0x01;
    /// AVI InfoFrame — active video format, colorimetry, aspect ratio.
    pub const AVI: u8 = 0x02;
    /// Source Product Descriptor InfoFrame.
    pub const SOURCE_PRODUCT_DESCRIPTOR: u8 = 0x03;
    /// Audio InfoFrame.
    pub const AUDIO: u8 = 0x04;
    /// MPEG Source InfoFrame.
    pub const MPEG_SOURCE: u8 = 0x05;
    /// NTSC VBI InfoFrame.
    pub const NTSC_VBI: u8 = 0x06;
    /// Dynamic Range and Mastering InfoFrame (HDR10 static metadata).
    pub const DYNAMIC_RANGE_MASTERING: u8 = 0x07;
}

/// One entry from an InfoFrame Data Block (extended tag `0x20`).
///
/// Each descriptor identifies an InfoFrame type that the sink is capable of
/// receiving. For Vendor-Specific InfoFrames (`type_code == 0x01`) the IEEE OUI
/// of the vendor is also present.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InfoFrameDescriptor {
    /// InfoFrame type code (bits 4:0 of the SID byte).
    ///
    /// See the constants in [`infoframe_type`] for the well-known values.
    pub type_code: u8,
    /// IEEE OUI for Vendor-Specific InfoFrames (`type_code == 0x01`).
    ///
    /// Stored as `(byte0 << 16) | (byte1 << 8) | byte2` following the byte
    /// order used in CTA-861.  `None` for all other types.
    pub vendor_oui: Option<u32>,
}

impl InfoFrameDescriptor {
    /// Constructs an `InfoFrameDescriptor`.
    pub fn new(type_code: u8, vendor_oui: Option<u32>) -> Self {
        Self {
            type_code,
            vendor_oui,
        }
    }
}

/// A decoded Vendor-Specific Video Data Block (VSVDB, extended tag `0x01`) or
/// Vendor-Specific Audio Data Block (VSADB, extended tag `0x11`).
///
/// Both block types share the same structure: a 3-byte IEEE OUI followed by an
/// opaque vendor-defined payload (CTA-861 Tables 56–57). The payload is stored
/// verbatim for consumers that recognise the OUI.
///
/// Well-known video OUIs include Dolby Vision (`0x00D046`) and HDR10+ (`0x90848B`).
#[non_exhaustive]
#[cfg(any(feature = "alloc", feature = "std"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VendorSpecificBlock {
    /// 24-bit IEEE OUI in canonical (MSB-first) form.
    ///
    /// Assembled from the three OUI bytes as `(byte2 << 16) | (byte1 << 8) | byte0`,
    /// where byte0 is the least-significant byte as stored on the wire.
    pub oui: u32,
    /// Vendor-defined payload bytes following the OUI.
    pub payload: Vec<u8>,
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl VendorSpecificBlock {
    /// Constructs a `VendorSpecificBlock`.
    pub fn new(oui: u32, payload: Vec<u8>) -> Self {
        Self { oui, payload }
    }
}
