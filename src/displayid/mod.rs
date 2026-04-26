//! Types decoded from DisplayID 1.x and 2.x extension blocks (EDID extension tag `0x70`).

#[cfg(any(feature = "alloc", feature = "std"))]
use crate::Vec;

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
#[derive(Debug, Clone, PartialEq, Default)]
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
///
/// `Other` is `#[non_exhaustive]` and cannot be constructed outside this crate; use
/// [`from_byte`][Self::from_byte] (which canonicalises known bytes) or [`other`][Self::other]
/// (which rejects known bytes) so that every value satisfies the invariant
/// `from_byte(t.as_byte()) == t`.
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
    /// Reserved or vendor-specific value the decoder did not recognise. Construction
    /// is restricted to this crate; external callers must go through [`from_byte`][Self::from_byte]
    /// or [`other`][Self::other].
    #[non_exhaustive]
    Other(u8),
}

impl DisplayTechnology {
    /// Decodes the raw byte 10 value, mapping known bytes to their named variant.
    pub fn from_byte(b: u8) -> Self {
        match b {
            0x00 => Self::Unspecified,
            0x01 => Self::Amlcd,
            0x02 => Self::Amoled,
            other => Self::Other(other),
        }
    }

    /// Constructs an `Other(b)` for a non-canonical byte value.
    ///
    /// Returns `None` for `0x00`, `0x01`, and `0x02` — those bytes have named variants
    /// and constructing `Other` for them would break the round-trip invariant
    /// `from_byte(t.as_byte()) == t`. Callers decoding raw EDID bytes should use
    /// [`from_byte`][Self::from_byte] instead.
    pub fn other(b: u8) -> Option<Self> {
        match b {
            0x00..=0x02 => None,
            x => Some(Self::Other(x)),
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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

bitflags::bitflags! {
    /// Color depths supported for full-bandwidth encodings (RGB and YCbCr 4:4:4) per
    /// DisplayID 2.x block 0x26 bytes 0–1. Bit positions match the on-wire encoding.
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct ColorDepthsFull: u8 {
        /// 6 bits per channel.
        const BPC_6  = 0x01;
        /// 8 bits per channel.
        const BPC_8  = 0x02;
        /// 10 bits per channel.
        const BPC_10 = 0x04;
        /// 12 bits per channel.
        const BPC_12 = 0x08;
        /// 14 bits per channel.
        const BPC_14 = 0x10;
        /// 16 bits per channel.
        const BPC_16 = 0x20;
    }
}

bitflags::bitflags! {
    /// Color depths supported for chroma-subsampled YCbCr encodings (4:2:2 and 4:2:0)
    /// per DisplayID 2.x block 0x26 bytes 2–3. 6 bpc is not representable: subsampled
    /// formats start at 8 bpc. Bit positions match the on-wire encoding.
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct ColorDepthsSubsampled: u8 {
        /// 8 bits per channel.
        const BPC_8  = 0x01;
        /// 10 bits per channel.
        const BPC_10 = 0x02;
        /// 12 bits per channel.
        const BPC_12 = 0x04;
        /// 14 bits per channel.
        const BPC_14 = 0x08;
        /// 16 bits per channel.
        const BPC_16 = 0x10;
    }
}

/// Display interface features decoded from DisplayID 2.x block 0x26.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DisplayInterfaceFeatures {
    /// Color depths supported for RGB encoding (payload byte 0).
    pub color_depth_rgb: ColorDepthsFull,
    /// Color depths supported for YCbCr 4:4:4 encoding (payload byte 1).
    pub color_depth_ycbcr444: ColorDepthsFull,
    /// Color depths supported for YCbCr 4:2:2 encoding (payload byte 2).
    pub color_depth_ycbcr422: ColorDepthsSubsampled,
    /// Color depths supported for YCbCr 4:2:0 encoding (payload byte 3).
    pub color_depth_ycbcr420: ColorDepthsSubsampled,
    /// Minimum pixel rate for YCbCr 4:2:0 in units of 74.25 MP/s (payload byte 4). `0` = all modes supported.
    pub min_ycbcr420_pixel_rate: u8,
    /// Audio capability flags (payload byte 5). Stored as a raw byte: bits 5–7 advertise
    /// 32/44.1/48 kHz sample-rate support, lower bits carry audio override and additional
    /// flags whose semantics this crate does not yet typify. Treat as opaque until a
    /// dedicated bitflags type lands.
    pub audio_flags: u8,
    /// Color space and EOTF defined-combinations bitmask (payload byte 6). Each set bit
    /// indicates support for one of the spec's pre-defined `(color space, EOTF)` pairs.
    /// Stored as a raw byte until a typed bitflags wrapper for the defined combinations
    /// lands. Custom combinations (payload bytes 7–8) are a separate concept and are not
    /// represented here.
    pub color_space_eotf_combos: u8,
}

/// Identifies the eye targeted by a stereo viewing-method parameter.
///
/// Encodes the `Left`/`Right` selector that several DisplayID 2.x stereo methods (0x27)
/// use to label which half of the frame, or which physical interface, carries which eye.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StereoEye {
    /// Left eye view.
    Left,
    /// Right eye view.
    Right,
}

/// Mirroring applied to one of the dual interfaces in [`StereoViewingMethodV2::DualInterface`].
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DualInterfaceMirroring {
    /// No mirroring applied.
    None,
    /// Image is mirrored across the vertical axis (left/right swap).
    LeftRight,
    /// Image is mirrored across the horizontal axis (top/bottom swap).
    TopBottom,
    /// Reserved by the DisplayID 2.x specification.
    Reserved,
}

/// Scope of the timings to which a 2.x Stereo Display Interface block (0x27) applies.
///
/// Encoded in bits 7:6 of the block's revision/flags byte. Variants `ExplicitAndListedTimings`
/// and `ListedTimingCodesOnly` indicate that the block carries an inline list of timing codes
/// (DMT/VIC/HDMI VIC) which currently is not parsed; consumers can still detect its presence
/// via [`DisplayIdStereoInterfaceV2::has_timing_codes`].
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StereoTimingScopeV2 {
    /// Applies only to timings that explicitly report 3D capability.
    ExplicitTimingsOnly,
    /// Applies to explicit-3D timings AND the timing codes listed in this block.
    ExplicitAndListedTimings,
    /// Applies to all listed timings.
    AllListedTimings,
    /// Applies only to the timing codes listed in this block.
    ListedTimingCodesOnly,
}

impl StereoTimingScopeV2 {
    /// Decodes the scope from bits 7:6 of the block's revision byte.
    pub fn from_revision(revision: u8) -> Self {
        match revision >> 6 {
            0b00 => StereoTimingScopeV2::ExplicitTimingsOnly,
            0b01 => StereoTimingScopeV2::ExplicitAndListedTimings,
            0b10 => StereoTimingScopeV2::AllListedTimings,
            _ => StereoTimingScopeV2::ListedTimingCodesOnly,
        }
    }

    /// Returns `true` when the block payload includes an inline timing-code list.
    pub fn has_timing_codes(self) -> bool {
        matches!(
            self,
            StereoTimingScopeV2::ExplicitAndListedTimings
                | StereoTimingScopeV2::ListedTimingCodesOnly
        )
    }
}

/// Stereo viewing method advertised by a DisplayID 2.x Stereo Display Interface block (0x27).
///
/// Each variant carries the method-specific parameters. Method codes that are reserved by the
/// spec are surfaced as [`StereoViewingMethodV2::Reserved`] with the raw method byte.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StereoViewingMethodV2 {
    /// Method 0x00 — Field-sequential stereo. `eye_on_high_half` identifies which eye view
    /// is delivered during the HIGH half of the sync signal; the other eye is delivered
    /// during the LOW half. `Right` corresponds to payload bit 0 = 1 (spec wording
    /// "L/R polarity 1"), `Left` to bit 0 = 0 ("L/R polarity 0").
    FieldSequential {
        /// Eye view delivered during the HIGH half of the sync signal.
        eye_on_high_half: StereoEye,
    },
    /// Method 0x01 — Side-by-side stereo. `left_half` indicates which eye view occupies
    /// the left half of the frame.
    SideBySide {
        /// Eye view that occupies the left half of the frame.
        left_half: StereoEye,
    },
    /// Method 0x02 — Pixel-interleaved stereo. The 8-byte `pattern` describes an 8×8
    /// L/R pixel mask (bit set = Left, clear = Right). MSB of each byte is the leftmost
    /// pixel of that row.
    PixelInterleaved {
        /// 8×8 bitmap; row `i`, bit `7 − x` selects whether pixel `(x, i)` is Left (1) or Right (0).
        pattern: [u8; 8],
    },
    /// Method 0x03 — Dual-interface stereo. Each physical interface carries a single
    /// eye view; `eye` identifies which eye is delivered on this interface, and
    /// `mirroring` describes how the image is oriented.
    DualInterface {
        /// Eye view delivered over this interface.
        eye: StereoEye,
        /// Mirroring applied to this interface's image.
        mirroring: DualInterfaceMirroring,
    },
    /// Method 0x04 — Multi-view stereo. `view_count` is the number of views and
    /// `interleaving_method_code` is a vendor-defined identifier for how they interleave.
    MultiView {
        /// Number of distinct views in the multi-view configuration.
        view_count: u8,
        /// Vendor-defined identifier describing how the views interleave.
        interleaving_method_code: u8,
    },
    /// Method 0x05 — Stacked-frame stereo (top/bottom). `top_half` indicates which eye
    /// view occupies the top half of the frame.
    StackedFrame {
        /// Eye view that occupies the top half of the frame.
        top_half: StereoEye,
    },
    /// Method 0xFF — Proprietary / vendor-defined.
    Proprietary,
    /// Method codes reserved by the DisplayID 2.x specification (0x06–0xFE).
    Reserved(u8),
}

/// Stereo display interface decoded from a DisplayID 2.x block 0x27.
///
/// The block also carries an optional inline list of timing codes (DMT/VIC/HDMI VIC) when
/// [`Self::has_timing_codes`] returns `true`; that list is not currently parsed.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayIdStereoInterfaceV2 {
    /// Scope of timings to which this stereo configuration applies.
    pub timing_scope: StereoTimingScopeV2,
    /// Stereo viewing method and method-specific parameters.
    pub method: StereoViewingMethodV2,
}

impl Default for DisplayIdStereoInterfaceV2 {
    fn default() -> Self {
        Self {
            timing_scope: StereoTimingScopeV2::ExplicitTimingsOnly,
            method: StereoViewingMethodV2::Proprietary,
        }
    }
}

impl DisplayIdStereoInterfaceV2 {
    /// Convenience accessor for [`StereoTimingScopeV2::has_timing_codes`].
    pub fn has_timing_codes(&self) -> bool {
        self.timing_scope.has_timing_codes()
    }
}

/// Vendor-specific data block from DisplayID 2.x block 0x7E (§4.10).
///
/// The payload is an IEEE OUI identifying the vendor followed by `n` bytes of
/// vendor-defined data whose semantics this crate does not interpret. Consumers
/// match on `oui` to dispatch to a vendor-specific parser of their choice.
///
/// Multiple 0x7E blocks may appear in a single section — each is decoded and
/// appended to `DisplayIdCapabilities::vendor_specific` in payload order.
#[non_exhaustive]
#[cfg(any(feature = "alloc", feature = "std"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DisplayIdVendorSpecific {
    /// 3-byte IEEE OUI identifying the vendor (e.g. `[0x00, 0xD0, 0x46]` = Dolby).
    /// Bytes are stored in the order they appear in the payload (high-order byte first).
    pub oui: [u8; 3],
    /// Opaque vendor-defined payload following the OUI. Empty when the block carries
    /// only the OUI with no further data.
    pub data: Vec<u8>,
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
    /// Stereo display interface from 2.x block 0x27.
    pub stereo_interface_v2: Option<DisplayIdStereoInterfaceV2>,
    /// ContainerID UUID from 2.x block 0x29 (16 raw bytes).
    pub container_id: Option<[u8; 16]>,
    /// Vendor-specific data blocks from 2.x block 0x7E, in payload order.
    /// Empty when no 0x7E blocks were present.
    pub vendor_specific: Vec<DisplayIdVendorSpecific>,
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
            stereo_interface_v2: None,
            container_id: None,
            vendor_specific: Vec::new(),
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
            let t = DisplayTechnology::from_byte(b);
            assert_eq!(t.as_byte(), b);
            assert_eq!(DisplayTechnology::from_byte(t.as_byte()), t);
        }
    }

    #[test]
    fn display_technology_other_rejects_canonical_bytes() {
        assert_eq!(DisplayTechnology::other(0x00), None);
        assert_eq!(DisplayTechnology::other(0x01), None);
        assert_eq!(DisplayTechnology::other(0x02), None);
    }

    #[test]
    fn display_technology_other_accepts_reserved_bytes() {
        assert_eq!(
            DisplayTechnology::other(0x42),
            Some(DisplayTechnology::Other(0x42))
        );
        assert_eq!(
            DisplayTechnology::other(0xFF),
            Some(DisplayTechnology::Other(0xFF))
        );
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

    #[test]
    fn color_depths_full_bit_layout() {
        // Wire byte 0b0010_1010 = 8 + 12 + 16 bpc.
        let depths = ColorDepthsFull::from_bits_truncate(0b0010_1010);
        assert!(depths.contains(ColorDepthsFull::BPC_8));
        assert!(depths.contains(ColorDepthsFull::BPC_12));
        assert!(depths.contains(ColorDepthsFull::BPC_16));
        assert!(!depths.contains(ColorDepthsFull::BPC_6));
        assert!(!depths.contains(ColorDepthsFull::BPC_10));
        assert!(!depths.contains(ColorDepthsFull::BPC_14));
        assert_eq!(depths.bits(), 0b0010_1010);
    }

    #[test]
    fn color_depths_subsampled_bit_layout() {
        // Wire byte 0b0001_0101 = 8 + 12 + 16 bpc (no 6 bpc bit).
        let depths = ColorDepthsSubsampled::from_bits_truncate(0b0001_0101);
        assert!(depths.contains(ColorDepthsSubsampled::BPC_8));
        assert!(depths.contains(ColorDepthsSubsampled::BPC_12));
        assert!(depths.contains(ColorDepthsSubsampled::BPC_16));
        assert!(!depths.contains(ColorDepthsSubsampled::BPC_10));
        assert!(!depths.contains(ColorDepthsSubsampled::BPC_14));
        assert_eq!(depths.bits(), 0b0001_0101);
    }

    #[test]
    fn color_depths_default_is_empty() {
        assert!(ColorDepthsFull::default().is_empty());
        assert!(ColorDepthsSubsampled::default().is_empty());
        assert!(
            DisplayInterfaceFeatures::default()
                .color_depth_rgb
                .is_empty()
        );
    }

    #[test]
    fn color_depths_subsampled_truncates_reserved_bits() {
        // Bit 5 is reserved for subsampled; from_bits_truncate drops it.
        let depths = ColorDepthsSubsampled::from_bits_truncate(0b0011_1111);
        assert_eq!(depths.bits(), 0b0001_1111);
    }
}
