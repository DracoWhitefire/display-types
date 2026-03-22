//! Types decoded from DisplayID 1.x extension blocks (EDID extension tag `0x70`).

/// Rich capabilities extracted from a DisplayID 1.x extension section.
///
/// Stored in `DisplayCapabilities` via `set_extension_data(0x70, ...)` by the dynamic
/// pipeline; retrieve with `caps.get_extension_data::<DisplayIdCapabilities>(0x70)`.
#[non_exhaustive]
#[cfg(any(feature = "alloc", feature = "std"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayIdCapabilities {
    /// DisplayID version byte (0x10–0x1F for v1.x, 0x20 for v2.x).
    pub version: u8,
    /// Display product primary use case (bits 2:0 of header byte 3).
    pub product_type: u8,
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl DisplayIdCapabilities {
    /// Constructs a `DisplayIdCapabilities`.
    pub fn new(version: u8, product_type: u8) -> Self {
        Self {
            version,
            product_type,
        }
    }
}
