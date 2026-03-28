//! Resolved display configuration type.

use crate::cea861::HdmiForumFrl;
use crate::{ColorBitDepth, ColorFormat, VideoMode};

/// A resolved display configuration ready to program into hardware.
///
/// `ResolvedDisplayConfig` contains the hardware-relevant fields produced by a
/// display negotiation engine — the video mode, color encoding, transport
/// settings, and compression flags that a DRM driver or InfoFrame encoder
/// needs to configure the link.
///
/// # Design note
///
/// This type lives in `display-types` so that drivers, InfoFrame encoders, and
/// compositors can consume negotiation output without a direct dependency on the
/// negotiation engine. This mirrors how [`DisplayCapabilities`] lives here so
/// negotiation engines can consume parser output without depending on the parser.
///
/// [`DisplayCapabilities`]: crate::DisplayCapabilities
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedDisplayConfig {
    /// The resolved video mode.
    pub mode: VideoMode,

    /// Color encoding format for this configuration.
    pub color_encoding: ColorFormat,

    /// Color bit depth per channel.
    pub bit_depth: ColorBitDepth,

    /// FRL rate tier, or [`HdmiForumFrl::NotSupported`] for TMDS transport.
    pub frl_rate: HdmiForumFrl,

    /// Whether Display Stream Compression is required for this configuration.
    pub dsc_required: bool,

    /// Whether Variable Refresh Rate is applicable for this configuration.
    pub vrr_applicable: bool,
}

impl ResolvedDisplayConfig {
    /// Constructs a `ResolvedDisplayConfig`.
    pub fn new(
        mode: VideoMode,
        color_encoding: ColorFormat,
        bit_depth: ColorBitDepth,
        frl_rate: HdmiForumFrl,
        dsc_required: bool,
        vrr_applicable: bool,
    ) -> Self {
        Self {
            mode,
            color_encoding,
            bit_depth,
            frl_rate,
            dsc_required,
            vrr_applicable,
        }
    }
}
