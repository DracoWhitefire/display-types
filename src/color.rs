/// A single CIE xy chromaticity coordinate pair, decoded from the EDID base block.
///
/// Values are stored as raw 10-bit integers. Use [`x`][Self::x] and [`y`][Self::y]
/// to obtain the normalised `f32` coordinates in the range `[0.0, 1.0)`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ChromaticityPoint {
    /// Raw 10-bit x value.
    pub x_raw: u16,
    /// Raw 10-bit y value.
    pub y_raw: u16,
}

impl ChromaticityPoint {
    /// CIE x coordinate, normalised to `[0.0, 1.0)`.
    pub fn x(&self) -> f32 {
        self.x_raw as f32 / 1024.0
    }

    /// CIE y coordinate, normalised to `[0.0, 1.0)`.
    pub fn y(&self) -> f32 {
        self.y_raw as f32 / 1024.0
    }
}

/// CIE xy chromaticity coordinates for a display's color primaries and white point,
/// decoded from EDID base block bytes `0x19`–`0x22`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Chromaticity {
    /// Red primary chromaticity.
    pub red: ChromaticityPoint,
    /// Green primary chromaticity.
    pub green: ChromaticityPoint,
    /// Blue primary chromaticity.
    pub blue: ChromaticityPoint,
    /// White point chromaticity.
    pub white: ChromaticityPoint,
}

/// An additional white point entry from a `0xFB` descriptor.
///
/// Displays that support multiple white points (e.g. for HDR or wide-gamut modes)
/// encode supplementary white points here, beyond the primary white point in the
/// chromaticity block.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhitePoint {
    /// White point index number (1–255), assigned by the manufacturer.
    pub index: u8,
    /// CIE xy chromaticity of this white point.
    pub chromaticity: ChromaticityPoint,
    /// Gamma for this white point. `None` if unspecified (`0xFF`).
    pub gamma: Option<DisplayGamma>,
}

/// Display gamma, decoded from EDID base block byte `0x17`.
///
/// Gamma is encoded as `(value * 100) - 100`, so a stored byte of `120` represents
/// gamma 2.20. A byte value of `0xFF` means gamma is undefined; use `None` on
/// [`DisplayCapabilities`][crate::DisplayCapabilities] in that case.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayGamma(u8);

impl DisplayGamma {
    /// Decodes EDID byte `0x17` into a `DisplayGamma`.
    ///
    /// Returns `None` if the byte is `0xFF` (gamma not specified).
    pub fn from_edid_byte(byte: u8) -> Option<Self> {
        if byte == 0xFF { None } else { Some(Self(byte)) }
    }

    /// Returns the raw encoded byte.
    pub fn raw(&self) -> u8 {
        self.0
    }

    /// Returns the gamma value as a floating-point number (e.g. `2.20`).
    pub fn value(&self) -> f32 {
        (self.0 as f32 + 100.0) / 100.0
    }
}

/// Supported color encoding formats for a digital display, decoded from EDID base block
/// byte `0x18` bits 4–3.
///
/// Defined for EDID 1.4+ digital inputs. On EDID 1.3 displays this field has a different
/// meaning and is not decoded here.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigitalColorEncoding {
    /// Only RGB 4:4:4 is supported.
    Rgb444,
    /// RGB 4:4:4 and YCbCr 4:4:4 are supported.
    Rgb444YCbCr444,
    /// RGB 4:4:4 and YCbCr 4:2:2 are supported.
    Rgb444YCbCr422,
    /// RGB 4:4:4, YCbCr 4:4:4, and YCbCr 4:2:2 are supported.
    Rgb444YCbCr444YCbCr422,
}

/// A single color encoding format, used to describe a specific negotiated or candidate
/// configuration.
///
/// Unlike [`DigitalColorEncoding`], which models the 2-bit EDID base block field and therefore
/// only expresses combinations present in that field, `ColorFormat` names each encoding
/// individually. It covers YCbCr 4:2:0, which is signaled through CEA/CTA extension blocks
/// rather than the base block.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorFormat {
    /// RGB 4:4:4.
    Rgb444,
    /// YCbCr 4:4:4.
    YCbCr444,
    /// YCbCr 4:2:2.
    YCbCr422,
    /// YCbCr 4:2:0.
    YCbCr420,
}

/// Display color type for an analog display, decoded from EDID base block byte `0x18` bits 4–3.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalogColorType {
    /// Monochrome or grayscale display.
    Monochrome,
    /// RGB color display.
    Rgb,
    /// Non-RGB multicolor display.
    NonRgb,
}

/// Color bit depth per primary color channel, decoded from EDID base block byte `0x14` bits 6–4.
///
/// Only valid for digital input displays. `None` is used for the undefined (0b000) and
/// reserved (0b111) values.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorBitDepth {
    /// 6 bits per primary color channel.
    Depth6,
    /// 8 bits per primary color channel.
    Depth8,
    /// 10 bits per primary color channel.
    Depth10,
    /// 12 bits per primary color channel.
    Depth12,
    /// 14 bits per primary color channel.
    Depth14,
    /// 16 bits per primary color channel.
    Depth16,
}

impl ColorBitDepth {
    /// Returns the number of bits per primary color channel.
    pub fn bits_per_primary(&self) -> u8 {
        match self {
            Self::Depth6 => 6,
            Self::Depth8 => 8,
            Self::Depth10 => 10,
            Self::Depth12 => 12,
            Self::Depth14 => 14,
            Self::Depth16 => 16,
        }
    }
}

/// Bit depths supported for a given color encoding, as a compact bitset.
///
/// Used as the per-format field in [`ColorCapabilities`].
/// An empty set means the color format is not supported at all.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ColorBitDepths(u8);

impl ColorBitDepths {
    /// No bit depths supported (format absent).
    pub const NONE: Self = Self(0);
    /// 6 bits per primary color channel.
    pub const BPC_6: Self = Self(1 << 0);
    /// 8 bits per primary color channel.
    pub const BPC_8: Self = Self(1 << 1);
    /// 10 bits per primary color channel.
    pub const BPC_10: Self = Self(1 << 2);
    /// 12 bits per primary color channel.
    pub const BPC_12: Self = Self(1 << 3);
    /// 14 bits per primary color channel.
    pub const BPC_14: Self = Self(1 << 4);
    /// 16 bits per primary color channel.
    pub const BPC_16: Self = Self(1 << 5);

    /// Returns `true` if this set contains no supported bit depths.
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns a new set that also includes `depth`.
    pub const fn with(self, depth: ColorBitDepth) -> Self {
        Self(self.0 | Self::flag(depth).0)
    }

    /// Returns `true` if `depth` is in this set.
    pub const fn supports(self, depth: ColorBitDepth) -> bool {
        self.0 & Self::flag(depth).0 != 0
    }

    const fn flag(depth: ColorBitDepth) -> Self {
        match depth {
            ColorBitDepth::Depth6 => Self::BPC_6,
            ColorBitDepth::Depth8 => Self::BPC_8,
            ColorBitDepth::Depth10 => Self::BPC_10,
            ColorBitDepth::Depth12 => Self::BPC_12,
            ColorBitDepth::Depth14 => Self::BPC_14,
            ColorBitDepth::Depth16 => Self::BPC_16,
        }
    }
}

/// Supported color format and bit-depth combinations for a display.
///
/// Each field holds the set of bit depths the sink accepts for that color format.
/// An empty [`ColorBitDepths`] means the format is not supported at all.
///
/// This replaces the scattered `DigitalColorEncoding` + `color_bit_depth` + Deep Color
/// booleans with a single structure that can answer "does this sink accept
/// (YCbCr 4:2:0, 10 bpc)?" directly.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ColorCapabilities {
    /// Supported bit depths for RGB 4:4:4.
    pub rgb444: ColorBitDepths,
    /// Supported bit depths for YCbCr 4:4:4.
    pub ycbcr444: ColorBitDepths,
    /// Supported bit depths for YCbCr 4:2:2.
    pub ycbcr422: ColorBitDepths,
    /// Supported bit depths for YCbCr 4:2:0.
    pub ycbcr420: ColorBitDepths,
}

impl ColorCapabilities {
    /// Returns the supported bit depths for the given color format.
    pub const fn for_format(&self, format: ColorFormat) -> ColorBitDepths {
        match format {
            ColorFormat::Rgb444 => self.rgb444,
            ColorFormat::YCbCr444 => self.ycbcr444,
            ColorFormat::YCbCr422 => self.ycbcr422,
            ColorFormat::YCbCr420 => self.ycbcr420,
        }
    }
}

/// Derives [`ColorCapabilities`] from the raw EDID fields that encode color support.
///
/// Combines four independent EDID/HDMI sources into a single coherent map:
///
/// - `encoding` — EDID base block byte `0x18` bits 4–3: which base formats are declared.
/// - `base_depth` — EDID base block byte `0x14` bits 6–4: native input bit depth.
///   Used only when `hdmi_vsdb` is absent (non-HDMI or very old HDMI display).
/// - `hdmi_vsdb` — HDMI 1.x VSDB flags: deep color depths for RGB and YCbCr 4:4:4.
/// - `hdmi_forum` — HF-SCDB: deep color depths for YCbCr 4:2:0.
///
/// # YCbCr 4:2:0 at 8 bpc
///
/// Plain 8 bpc YCbCr 4:2:0 support (without deep color) is declared in the CEA/CTA
/// YCbCr 4:2:0 Video Data Block, which is not captured by these four fields.
/// This function infers 8 bpc YCbCr 4:2:0 support only when at least one deep color
/// flag is set in `hdmi_forum`, since deep color implies baseline support.
/// If a display supports 8 bpc YCbCr 4:2:0 but no deep color, supplement the returned
/// value by adding [`ColorBitDepths::BPC_8`] to [`ColorCapabilities::ycbcr420`] after
/// calling this function.
pub fn color_capabilities_from_edid(
    encoding: Option<DigitalColorEncoding>,
    base_depth: Option<ColorBitDepth>,
    hdmi_vsdb: Option<&crate::cea861::HdmiVsdb>,
    hdmi_forum: Option<&crate::cea861::HdmiForumSinkCap>,
) -> ColorCapabilities {
    use crate::cea861::HdmiVsdbFlags;

    let (ycbcr444_base, ycbcr422_base) = match encoding {
        None | Some(DigitalColorEncoding::Rgb444) => (false, false),
        Some(DigitalColorEncoding::Rgb444YCbCr444) => (true, false),
        Some(DigitalColorEncoding::Rgb444YCbCr422) => (false, true),
        Some(DigitalColorEncoding::Rgb444YCbCr444YCbCr422) => (true, true),
    };

    // RGB: always 8 bpc; deep color depths from VSDB flags.
    // Without VSDB, fall back to the base block bit depth declaration.
    let rgb444 = if let Some(vsdb) = hdmi_vsdb {
        let mut d = ColorBitDepths::BPC_8;
        if vsdb.flags.contains(HdmiVsdbFlags::DC_30BIT) {
            d = d.with(ColorBitDepth::Depth10);
        }
        if vsdb.flags.contains(HdmiVsdbFlags::DC_36BIT) {
            d = d.with(ColorBitDepth::Depth12);
        }
        if vsdb.flags.contains(HdmiVsdbFlags::DC_48BIT) {
            d = d.with(ColorBitDepth::Depth16);
        }
        d
    } else {
        base_depth.map_or(ColorBitDepths::BPC_8, |d| ColorBitDepths::BPC_8.with(d))
    };

    // YCbCr 4:4:4: deep color only when DC_Y444 is set alongside the RGB deep color flags.
    let ycbcr444 = if ycbcr444_base {
        let dc_y444 = hdmi_vsdb.is_some_and(|v| v.flags.contains(HdmiVsdbFlags::DC_Y444));
        if dc_y444 {
            rgb444
        } else {
            ColorBitDepths::BPC_8
        }
    } else {
        ColorBitDepths::NONE
    };

    // YCbCr 4:2:2: deep color is not tracked separately in these fields.
    let ycbcr422 = if ycbcr422_base {
        ColorBitDepths::BPC_8
    } else {
        ColorBitDepths::NONE
    };

    // YCbCr 4:2:0: deep color depths from HF-SCDB; 8 bpc implied by any deep color flag.
    let ycbcr420 = if let Some(forum) = hdmi_forum {
        let any_dc = forum.dc_30bit_420 || forum.dc_36bit_420 || forum.dc_48bit_420;
        let mut d = if any_dc {
            ColorBitDepths::BPC_8
        } else {
            ColorBitDepths::NONE
        };
        if forum.dc_30bit_420 {
            d = d.with(ColorBitDepth::Depth10);
        }
        if forum.dc_36bit_420 {
            d = d.with(ColorBitDepth::Depth12);
        }
        if forum.dc_48bit_420 {
            d = d.with(ColorBitDepth::Depth16);
        }
        d
    } else {
        ColorBitDepths::NONE
    };

    ColorCapabilities {
        rgb444,
        ycbcr444,
        ycbcr422,
        ycbcr420,
    }
}

/// DCM polynomial coefficients for a single primary colour, decoded from a `0xF9` descriptor.
///
/// The `a3` and `a2` values are the second- and third-order coefficients of the colour
/// management polynomial defined in the VESA DCM Standard v1 (January 2003).
/// Both values are unsigned 16-bit little-endian quantities.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DcmChannel {
    /// Third-order polynomial coefficient (`a3`).
    pub a3: u16,
    /// Second-order polynomial coefficient (`a2`).
    pub a2: u16,
}

/// Color Management Data decoded from a `0xF9` descriptor.
///
/// Contains DCM polynomial coefficients for the red, green, and blue primaries.
/// Only present when the EDID includes a Color Management Data descriptor with version
/// byte `0x03`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorManagementData {
    /// DCM coefficients for the red primary.
    pub red: DcmChannel,
    /// DCM coefficients for the green primary.
    pub green: DcmChannel,
    /// DCM coefficients for the blue primary.
    pub blue: DcmChannel,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cea861::{HdmiForumFrl, HdmiForumSinkCap, HdmiVsdb, HdmiVsdbFlags};

    /// Minimal VSDB with only the deep color flags set.
    fn vsdb(flags: HdmiVsdbFlags) -> HdmiVsdb {
        HdmiVsdb::new(0, flags, None, None, None, None, None)
    }

    /// Minimal HF-SCDB with only the YCbCr 4:2:0 deep color flags set.
    fn hf_forum(dc_30: bool, dc_36: bool, dc_48: bool) -> HdmiForumSinkCap {
        HdmiForumSinkCap::new(
            1,
            0,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            HdmiForumFrl::NotSupported,
            false,
            dc_48,
            dc_36,
            dc_30,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            None,
            None,
            None,
        )
    }

    fn caps(
        encoding: Option<DigitalColorEncoding>,
        base_depth: Option<ColorBitDepth>,
        vsdb: Option<&HdmiVsdb>,
        forum: Option<&HdmiForumSinkCap>,
    ) -> ColorCapabilities {
        color_capabilities_from_edid(encoding, base_depth, vsdb, forum)
    }

    // --- RGB 4:4:4 ---

    #[test]
    fn rgb_no_vsdb_no_base_depth_gives_8bpc() {
        let c = caps(None, None, None, None);
        assert_eq!(c.rgb444, ColorBitDepths::BPC_8);
    }

    #[test]
    fn rgb_no_vsdb_base_depth_added() {
        let c = caps(None, Some(ColorBitDepth::Depth12), None, None);
        assert_eq!(c.rgb444, ColorBitDepths::BPC_8.with(ColorBitDepth::Depth12));
    }

    #[test]
    fn rgb_vsdb_no_dc_flags_gives_8bpc() {
        let v = vsdb(HdmiVsdbFlags::empty());
        let c = caps(None, Some(ColorBitDepth::Depth12), Some(&v), None);
        // VSDB present → base_depth ignored; no DC flags → 8 bpc only.
        assert_eq!(c.rgb444, ColorBitDepths::BPC_8);
    }

    #[test]
    fn rgb_vsdb_dc_30bit() {
        let v = vsdb(HdmiVsdbFlags::DC_30BIT);
        let c = caps(None, None, Some(&v), None);
        assert_eq!(c.rgb444, ColorBitDepths::BPC_8.with(ColorBitDepth::Depth10));
    }

    #[test]
    fn rgb_vsdb_dc_36bit() {
        let v = vsdb(HdmiVsdbFlags::DC_36BIT);
        let c = caps(None, None, Some(&v), None);
        assert_eq!(c.rgb444, ColorBitDepths::BPC_8.with(ColorBitDepth::Depth12));
    }

    #[test]
    fn rgb_vsdb_dc_48bit() {
        let v = vsdb(HdmiVsdbFlags::DC_48BIT);
        let c = caps(None, None, Some(&v), None);
        assert_eq!(c.rgb444, ColorBitDepths::BPC_8.with(ColorBitDepth::Depth16));
    }

    #[test]
    fn rgb_vsdb_multiple_dc_flags() {
        let v = vsdb(HdmiVsdbFlags::DC_30BIT | HdmiVsdbFlags::DC_36BIT);
        let c = caps(None, None, Some(&v), None);
        assert_eq!(
            c.rgb444,
            ColorBitDepths::BPC_8
                .with(ColorBitDepth::Depth10)
                .with(ColorBitDepth::Depth12)
        );
    }

    // --- YCbCr 4:4:4 ---

    #[test]
    fn ycbcr444_not_declared_gives_none() {
        let v = vsdb(HdmiVsdbFlags::DC_30BIT | HdmiVsdbFlags::DC_Y444);
        let c = caps(Some(DigitalColorEncoding::Rgb444), None, Some(&v), None);
        assert_eq!(c.ycbcr444, ColorBitDepths::NONE);
    }

    #[test]
    fn ycbcr444_declared_without_dc_y444_gives_8bpc() {
        let v = vsdb(HdmiVsdbFlags::DC_30BIT); // DC_Y444 not set
        let c = caps(
            Some(DigitalColorEncoding::Rgb444YCbCr444),
            None,
            Some(&v),
            None,
        );
        assert_eq!(c.ycbcr444, ColorBitDepths::BPC_8);
    }

    #[test]
    fn ycbcr444_declared_with_dc_y444_mirrors_rgb() {
        let v = vsdb(HdmiVsdbFlags::DC_30BIT | HdmiVsdbFlags::DC_36BIT | HdmiVsdbFlags::DC_Y444);
        let c = caps(
            Some(DigitalColorEncoding::Rgb444YCbCr444),
            None,
            Some(&v),
            None,
        );
        // DC_Y444 set → YCbCr 4:4:4 gets the same depths as RGB.
        assert_eq!(c.ycbcr444, c.rgb444);
        assert!(c.ycbcr444.supports(ColorBitDepth::Depth10));
        assert!(c.ycbcr444.supports(ColorBitDepth::Depth12));
    }

    #[test]
    fn ycbcr444_declared_no_vsdb_gives_8bpc() {
        // No VSDB at all → DC_Y444 cannot be set → 8 bpc only.
        let c = caps(Some(DigitalColorEncoding::Rgb444YCbCr444), None, None, None);
        assert_eq!(c.ycbcr444, ColorBitDepths::BPC_8);
    }

    // --- YCbCr 4:2:2 ---

    #[test]
    fn ycbcr422_not_declared_gives_none() {
        let c = caps(Some(DigitalColorEncoding::Rgb444YCbCr444), None, None, None);
        assert_eq!(c.ycbcr422, ColorBitDepths::NONE);
    }

    #[test]
    fn ycbcr422_declared_gives_8bpc_only() {
        let c = caps(Some(DigitalColorEncoding::Rgb444YCbCr422), None, None, None);
        assert_eq!(c.ycbcr422, ColorBitDepths::BPC_8);
    }

    #[test]
    fn ycbcr422_deep_color_not_tracked() {
        // Even with all VSDB DC flags, 4:2:2 stays at 8 bpc.
        let v = vsdb(HdmiVsdbFlags::DC_30BIT | HdmiVsdbFlags::DC_36BIT | HdmiVsdbFlags::DC_48BIT);
        let c = caps(
            Some(DigitalColorEncoding::Rgb444YCbCr444YCbCr422),
            None,
            Some(&v),
            None,
        );
        assert_eq!(c.ycbcr422, ColorBitDepths::BPC_8);
    }

    // --- YCbCr 4:2:0 ---

    #[test]
    fn ycbcr420_no_hf_scdb_gives_none() {
        let c = caps(None, None, None, None);
        assert_eq!(c.ycbcr420, ColorBitDepths::NONE);
    }

    #[test]
    fn ycbcr420_hf_scdb_no_dc_flags_gives_none() {
        let f = hf_forum(false, false, false);
        let c = caps(None, None, None, Some(&f));
        assert_eq!(c.ycbcr420, ColorBitDepths::NONE);
    }

    #[test]
    fn ycbcr420_dc_30bit_gives_8_and_10bpc() {
        let f = hf_forum(true, false, false);
        let c = caps(None, None, None, Some(&f));
        assert_eq!(
            c.ycbcr420,
            ColorBitDepths::BPC_8.with(ColorBitDepth::Depth10)
        );
    }

    #[test]
    fn ycbcr420_dc_36bit_gives_8_and_12bpc() {
        let f = hf_forum(false, true, false);
        let c = caps(None, None, None, Some(&f));
        assert_eq!(
            c.ycbcr420,
            ColorBitDepths::BPC_8.with(ColorBitDepth::Depth12)
        );
    }

    #[test]
    fn ycbcr420_dc_48bit_gives_8_and_16bpc() {
        let f = hf_forum(false, false, true);
        let c = caps(None, None, None, Some(&f));
        assert_eq!(
            c.ycbcr420,
            ColorBitDepths::BPC_8.with(ColorBitDepth::Depth16)
        );
    }

    #[test]
    fn ycbcr420_all_dc_flags() {
        let f = hf_forum(true, true, true);
        let c = caps(None, None, None, Some(&f));
        assert_eq!(
            c.ycbcr420,
            ColorBitDepths::BPC_8
                .with(ColorBitDepth::Depth10)
                .with(ColorBitDepth::Depth12)
                .with(ColorBitDepth::Depth16)
        );
    }
}
