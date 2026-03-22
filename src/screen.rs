/// Physical screen dimensions or aspect ratio, decoded from EDID base block bytes `0x15`–`0x16`.
///
/// The two bytes encode one of three things depending on which are zero:
///
/// | `0x15` | `0x16` | Meaning                           |
/// |--------|--------|-----------------------------------|
/// | non-0  | non-0  | Physical width × height in cm     |
/// | non-0  | 0      | Landscape aspect ratio            |
/// | 0      | non-0  | Portrait aspect ratio             |
/// | 0      | 0      | Undefined — `None` on the field   |
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenSize {
    /// Physical screen dimensions. Values are in centimetres (1–255 cm per axis).
    Physical {
        /// Horizontal screen size in centimetres.
        width_cm: u8,
        /// Vertical screen size in centimetres.
        height_cm: u8,
    },
    /// Landscape aspect ratio (width ÷ height > 1), encoded as a raw EDID byte.
    ///
    /// Call [`landscape_ratio`][Self::landscape_ratio] for the computed `f32` value.
    Landscape(u8),
    /// Portrait aspect ratio (width ÷ height < 1), encoded as a raw EDID byte.
    ///
    /// Call [`portrait_ratio`][Self::portrait_ratio] for the computed `f32` value.
    Portrait(u8),
}

impl ScreenSize {
    /// Returns the landscape aspect ratio (width ÷ height) for a `Landscape` variant.
    ///
    /// Formula: `(raw + 99) / 100`. Range: 1.00 → 3.54.
    /// Returns `None` for other variants.
    pub fn landscape_ratio(&self) -> Option<f32> {
        if let Self::Landscape(v) = self {
            Some((*v as f32 + 99.0) / 100.0)
        } else {
            None
        }
    }

    /// Returns the portrait aspect ratio (width ÷ height) for a `Portrait` variant.
    ///
    /// Formula: `100 / (raw + 99)`. Range: 0.28 → 0.99.
    /// Returns `None` for other variants.
    pub fn portrait_ratio(&self) -> Option<f32> {
        if let Self::Portrait(v) = self {
            Some(100.0 / (*v as f32 + 99.0))
        } else {
            None
        }
    }
}
