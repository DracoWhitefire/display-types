//! Types decoded from DisplayID 1.x and 2.x extension blocks (EDID extension tag `0x70`).

/// DisplayID 1.x and 2.x data block tag constants.
pub mod tag;

/// DisplayID 1.x display product primary use case constants.
pub mod product_type;

/// A single CIE chromaticity coordinate pair encoded as 12-bit fixed-point integers.
///
/// Used by DisplayID 2.x block 0x21. Each raw value is in the range `0..4096`, representing
/// a coordinate in `[0.0, 1.0)` with scale factor `2⁻¹²` (divide by 4096 to normalise).
/// The encoding may use CIE 1931 (x, y) or CIE 1976 (u', v') coordinates depending on
/// the `color_space_cie1976` flag in [`DisplayParamsV2`].
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ChromaticityPoint12 {
    /// Raw 12-bit x (or u') value.
    pub x_raw: u16,
    /// Raw 12-bit y (or v') value.
    pub y_raw: u16,
}

impl ChromaticityPoint12 {
    /// First coordinate (x or u'), normalised to `[0.0, 1.0)`.
    pub fn x(&self) -> f32 {
        self.x_raw as f32 / 4096.0
    }

    /// Second coordinate (y or v'), normalised to `[0.0, 1.0)`.
    pub fn y(&self) -> f32 {
        self.y_raw as f32 / 4096.0
    }
}

/// Factory-calibrated color primaries and white point from DisplayID 2.x block 0x21.
///
/// Chromaticity values use 12-bit fixed-point encoding; see [`ChromaticityPoint12`].
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Chromaticity12 {
    /// Primary color 1 (red) chromaticity.
    pub primary1: ChromaticityPoint12,
    /// Primary color 2 (green) chromaticity.
    pub primary2: ChromaticityPoint12,
    /// Primary color 3 (blue) chromaticity.
    pub primary3: ChromaticityPoint12,
    /// White point chromaticity.
    pub white: ChromaticityPoint12,
}

/// Display parameters decoded from DisplayID 2.x block 0x21.
///
/// Contains factory-calibrated colorimetry (12-bit chromaticity), HDR luminance
/// levels (IEEE 754 half-precision float), color depth, display technology, and
/// gamma. Image size and pixel count are exposed separately on
/// [`DisplayCapabilities`][crate::DisplayCapabilities] via `preferred_image_size_mm`
/// and `native_pixels`.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct DisplayParamsV2 {
    /// Factory-calibrated chromaticity for the three primaries and white point.
    pub chromaticity: Chromaticity12,
    /// `true` if chromaticity values use CIE 1976 (u', v') coordinates;
    /// `false` (default) for CIE 1931 (x, y).
    pub color_space_cie1976: bool,
    /// Maximum luminance at full-screen coverage in cd/m². `None` if not specified.
    pub max_luminance_full: Option<f32>,
    /// Maximum luminance at 10% screen coverage in cd/m². `None` if not specified.
    pub max_luminance_10pct: Option<f32>,
    /// Minimum luminance in cd/m². `None` if not specified.
    pub min_luminance: Option<f32>,
    /// `true` if non-zero luminance values are source guidance rather than guaranteed minima.
    pub luminance_guidance: bool,
    /// Color bit depth per channel (6, 8, 10, 12, 14, or 16). `None` if not defined.
    pub color_bit_depth: Option<u8>,
    /// Display technology decoded from byte 10 of block 0x21.
    pub display_technology: DisplayTechnology,
    /// Gamma EOTF in range 1.00–3.54. `None` if unspecified (stored byte `0xFF`).
    pub gamma: Option<f32>,
    /// Scan orientation decoded from bits 2:0 of byte 11 of block 0x21.
    pub scan_orientation: ScanOrientation,
    /// `true` if audio output uses an external jack rather than integrated speakers.
    pub audio_external: bool,
}

/// Display technology family, decoded from byte 10 of DisplayID 2.x block 0x21.
///
/// Unknown byte values are preserved via [`DisplayTechnology::Other`] so a spec-defined
/// future value (e.g. LCoS, microLED) does not make the containing block un-decodable.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DisplayTechnology {
    /// Not specified (byte value `0x00`).
    #[default]
    Unspecified,
    /// Active-matrix LCD (byte value `0x01`).
    Amlcd,
    /// Active-matrix OLED (byte value `0x02`).
    Amoled,
    /// Reserved or vendor-specific value the decoder did not recognise.
    Other(u8),
}

impl DisplayTechnology {
    /// Decodes the raw byte 10 value.
    pub fn from_byte(b: u8) -> Self {
        match b {
            0x00 => Self::Unspecified,
            0x01 => Self::Amlcd,
            0x02 => Self::Amoled,
            other => Self::Other(other),
        }
    }

    /// Returns the raw byte 10 representation.
    pub fn as_byte(self) -> u8 {
        match self {
            Self::Unspecified => 0x00,
            Self::Amlcd => 0x01,
            Self::Amoled => 0x02,
            Self::Other(b) => b,
        }
    }
}

/// Pixel scan orientation, decoded from bits 2:0 of byte 11 of DisplayID 2.x block 0x21.
///
/// Each variant names the fast (pixel) axis followed by the slow (line) axis. For example,
/// [`LeftRightTopBottom`][Self::LeftRightTopBottom] means pixels are painted left-to-right
/// within a line and lines advance top-to-bottom — the conventional raster order.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScanOrientation {
    /// Left-to-right, top-to-bottom (`0b000`). Default raster order.
    #[default]
    LeftRightTopBottom,
    /// Right-to-left, top-to-bottom (`0b001`).
    RightLeftTopBottom,
    /// Top-to-bottom, right-to-left (`0b010`).
    TopBottomRightLeft,
    /// Bottom-to-top, right-to-left (`0b011`).
    BottomTopRightLeft,
    /// Right-to-left, bottom-to-top (`0b100`).
    RightLeftBottomTop,
    /// Left-to-right, bottom-to-top (`0b101`).
    LeftRightBottomTop,
    /// Bottom-to-top, left-to-right (`0b110`).
    BottomTopLeftRight,
    /// Top-to-bottom, left-to-right (`0b111`).
    TopBottomLeftRight,
}

impl ScanOrientation {
    /// Decodes bits 2:0 of byte 11. Upper bits are ignored.
    pub fn from_bits(b: u8) -> Self {
        match b & 0b111 {
            0b000 => Self::LeftRightTopBottom,
            0b001 => Self::RightLeftTopBottom,
            0b010 => Self::TopBottomRightLeft,
            0b011 => Self::BottomTopRightLeft,
            0b100 => Self::RightLeftBottomTop,
            0b101 => Self::LeftRightBottomTop,
            0b110 => Self::BottomTopLeftRight,
            _ => Self::TopBottomLeftRight,
        }
    }

    /// Returns the 3-bit encoding (bits 2:0).
    pub fn as_bits(self) -> u8 {
        match self {
            Self::LeftRightTopBottom => 0b000,
            Self::RightLeftTopBottom => 0b001,
            Self::TopBottomRightLeft => 0b010,
            Self::BottomTopRightLeft => 0b011,
            Self::RightLeftBottomTop => 0b100,
            Self::LeftRightBottomTop => 0b101,
            Self::BottomTopLeftRight => 0b110,
            Self::TopBottomLeftRight => 0b111,
        }
    }
}

/// Dynamic video timing range decoded from DisplayID 2.x block 0x25.
///
/// Pixel clocks are in 1 kHz steps; vertical refresh rates cover the full 9-bit range
/// introduced in block revision 1.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DynamicTimingRange {
    /// Minimum pixel clock in kHz (3-byte LE field).
    pub min_pixel_clock_khz: u32,
    /// Maximum pixel clock in kHz (3-byte LE field).
    pub max_pixel_clock_khz: u32,
    /// Minimum vertical refresh rate in Hz.
    pub min_v_rate_hz: u8,
    /// Maximum vertical refresh rate in Hz (9-bit value; upper 2 bits from revision-1 flag byte).
    pub max_v_rate_hz: u16,
    /// Seamless variable refresh rate supported (fixed H pixel rate, dynamic V blanking).
    pub vrr_supported: bool,
}

/// Display interface features decoded from DisplayID 2.x block 0x26.
///
/// Each `color_depth_*` field is a bitmask where set bits indicate supported bit depths.
/// For RGB and YCbCr 4:4:4 the bit positions are: bit 0 = 6 bpc, 1 = 8, 2 = 10, 3 = 12,
/// 4 = 14, 5 = 16. For YCbCr 4:2:2 and 4:2:0: bit 0 = 8, 1 = 10, 2 = 12, 3 = 14, 4 = 16.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayInterfaceFeatures {
    /// Color depth support bitmask for RGB encoding.
    pub color_depth_rgb: u8,
    /// Color depth support bitmask for YCbCr 4:4:4 encoding.
    pub color_depth_ycbcr444: u8,
    /// Color depth support bitmask for YCbCr 4:2:2 encoding.
    pub color_depth_ycbcr422: u8,
    /// Color depth support bitmask for YCbCr 4:2:0 encoding.
    pub color_depth_ycbcr420: u8,
    /// Minimum pixel rate for YCbCr 4:2:0 in units of 74.25 MP/s. `0` = all modes supported.
    pub min_ycbcr420_pixel_rate: u8,
    /// Audio capability flags (bits 5–7: 48/44.1/32 kHz sample rate support).
    pub audio_flags: u8,
    /// Color space and EOTF combination 1 bitmask (byte 9 of payload).
    pub color_space_eotf_1: u8,
}

/// Rich capabilities extracted from a DisplayID 1.x or 2.x extension section.
///
/// Stored in `DisplayCapabilities` via `set_extension_data(0x70, ...)` by the dynamic
/// pipeline; retrieve with `caps.get_extension_data::<DisplayIdCapabilities>(0x70)`.
#[non_exhaustive]
#[cfg(any(feature = "alloc", feature = "std"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct DisplayIdCapabilities {
    /// DisplayID version byte (0x10–0x1F for v1.x, 0x20 for v2.x).
    pub version: u8,
    /// Display product primary use case (bits 2:0 of header byte 3).
    pub product_type: u8,
    /// IEEE OUI from the 2.x Product Identification block (0x20). Not a PNP ID.
    pub manufacturer_oui: Option<[u8; 3]>,
    /// Display parameters from 2.x block 0x21 (chromaticity, luminance, gamma).
    pub display_params_v2: Option<DisplayParamsV2>,
    /// Dynamic video timing range from 2.x block 0x25.
    pub dynamic_timing_range: Option<DynamicTimingRange>,
    /// Display interface features from 2.x block 0x26.
    pub interface_features: Option<DisplayInterfaceFeatures>,
    /// ContainerID UUID from 2.x block 0x29 (16 raw bytes).
    pub container_id: Option<[u8; 16]>,
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl DisplayIdCapabilities {
    /// Constructs a `DisplayIdCapabilities`.
    pub fn new(version: u8, product_type: u8) -> Self {
        Self {
            version,
            product_type,
            manufacturer_oui: None,
            display_params_v2: None,
            dynamic_timing_range: None,
            interface_features: None,
            container_id: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chromaticity_point_normalises_raw_to_unit_interval() {
        let white_d65 = ChromaticityPoint12 {
            x_raw: 1294, // ≈ 0.3158
            y_raw: 1347, // ≈ 0.3289
        };
        assert!((white_d65.x() - 0.31591797).abs() < 1e-6);
        assert!((white_d65.y() - 0.32885742).abs() < 1e-6);
    }

    #[test]
    fn chromaticity_point_endpoints() {
        let zero = ChromaticityPoint12::default();
        assert_eq!(zero.x(), 0.0);
        assert_eq!(zero.y(), 0.0);

        let max = ChromaticityPoint12 {
            x_raw: 4095,
            y_raw: 4095,
        };
        assert!(max.x() < 1.0);
        assert!(max.y() < 1.0);
    }

    #[test]
    fn display_technology_decodes_known_bytes() {
        assert_eq!(
            DisplayTechnology::from_byte(0x00),
            DisplayTechnology::Unspecified
        );
        assert_eq!(DisplayTechnology::from_byte(0x01), DisplayTechnology::Amlcd);
        assert_eq!(
            DisplayTechnology::from_byte(0x02),
            DisplayTechnology::Amoled
        );
    }

    #[test]
    fn display_technology_preserves_unknown_bytes() {
        assert_eq!(
            DisplayTechnology::from_byte(0x42),
            DisplayTechnology::Other(0x42)
        );
        assert_eq!(DisplayTechnology::Other(0x42).as_byte(), 0x42);
    }

    #[test]
    fn display_technology_round_trips() {
        for b in [0x00u8, 0x01, 0x02, 0x55, 0xFF] {
            assert_eq!(DisplayTechnology::from_byte(b).as_byte(), b);
        }
    }

    #[test]
    fn scan_orientation_round_trips_all_eight_codes() {
        for bits in 0u8..8 {
            let orient = ScanOrientation::from_bits(bits);
            assert_eq!(orient.as_bits(), bits);
        }
    }

    #[test]
    fn scan_orientation_ignores_upper_bits() {
        assert_eq!(
            ScanOrientation::from_bits(0b1111_1000),
            ScanOrientation::LeftRightTopBottom
        );
        assert_eq!(
            ScanOrientation::from_bits(0b1111_1111),
            ScanOrientation::TopBottomLeftRight
        );
    }

    #[test]
    fn defaults_match_raster_convention() {
        assert_eq!(DisplayTechnology::default(), DisplayTechnology::Unspecified);
        assert_eq!(
            ScanOrientation::default(),
            ScanOrientation::LeftRightTopBottom
        );
    }
}
