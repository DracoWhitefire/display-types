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
