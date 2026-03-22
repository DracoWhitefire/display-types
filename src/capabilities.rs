/// A reference-counted, type-erased warning value.
///
/// Any type that implements [`core::error::Error`] + [`Send`] + [`Sync`] + `'static` can be
/// wrapped in a `ParseWarning`. The built-in library variants use `EdidWarning`, but
/// custom handlers may push their own error types without wrapping them in `EdidWarning`.
///
/// Using [`Arc`][crate::prelude::Arc] (rather than `Box`) means `ParseWarning` is
/// [`Clone`], which lets warnings be copied from a parsed representation into
/// [`DisplayCapabilities`] without consuming the parsed result.
///
/// To inspect a specific variant, use the inherent `downcast_ref` method available on
/// `dyn core::error::Error + Send + Sync + 'static` in `std` builds:
///
/// ```text
/// for w in caps.iter_warnings() {
///     if let Some(ew) = (**w).downcast_ref::<EdidWarning>() { ... }
/// }
/// ```
#[cfg(any(feature = "alloc", feature = "std"))]
pub type ParseWarning = crate::prelude::Arc<dyn core::error::Error + Send + Sync + 'static>;

/// Stereo viewing support decoded from DTD byte 17 bits 6, 5, and 0.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StereoMode {
    /// Normal display; no stereo (bits 6–5 = `0b00`; bit 0 is don't-care).
    #[default]
    None,
    /// Field-sequential stereo, right image when stereo sync = 1 (bits 6–5 = `0b01`, bit 0 = 0).
    FieldSequentialRightFirst,
    /// Field-sequential stereo, left image when stereo sync = 1 (bits 6–5 = `0b10`, bit 0 = 0).
    FieldSequentialLeftFirst,
    /// 2-way interleaved stereo, right image on even lines (bits 6–5 = `0b01`, bit 0 = 1).
    TwoWayInterleavedRightEven,
    /// 2-way interleaved stereo, left image on even lines (bits 6–5 = `0b10`, bit 0 = 1).
    TwoWayInterleavedLeftEven,
    /// 4-way interleaved stereo (bits 6–5 = `0b11`, bit 0 = 0).
    FourWayInterleaved,
    /// Side-by-side interleaved stereo (bits 6–5 = `0b11`, bit 0 = 1).
    SideBySideInterleaved,
}

/// Sync signal definition decoded from DTD byte 17 bits 4–1.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncDefinition {
    /// Analog composite sync (bit 4 = 0, bit 3 = 0).
    AnalogComposite {
        /// H-sync pulse present during V-sync (serrations).
        serrations: bool,
        /// Sync on all three RGB signals (`true`) or green only (`false`).
        sync_on_all_rgb: bool,
    },
    /// Bipolar analog composite sync (bit 4 = 0, bit 3 = 1).
    BipolarAnalogComposite {
        /// H-sync pulse present during V-sync (serrations).
        serrations: bool,
        /// Sync on all three RGB signals (`true`) or green only (`false`).
        sync_on_all_rgb: bool,
    },
    /// Digital composite sync on H-sync pin (bit 4 = 1, bit 3 = 0).
    DigitalComposite {
        /// H-sync pulse present during V-sync (serrations).
        serrations: bool,
        /// H-sync polarity outside V-sync: `true` = positive.
        h_sync_positive: bool,
    },
    /// Digital separate sync (bit 4 = 1, bit 3 = 1).
    DigitalSeparate {
        /// V-sync polarity: `true` = positive.
        v_sync_positive: bool,
        /// H-sync polarity: `true` = positive.
        h_sync_positive: bool,
    },
}

/// A display video mode expressed as resolution, refresh rate, and scan type.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VideoMode {
    /// Horizontal resolution in pixels.
    pub width: u16,
    /// Vertical resolution in pixels.
    pub height: u16,
    /// Refresh rate in Hz.
    pub refresh_rate: u8,
    /// `true` for interlaced modes; `false` for progressive (the common case).
    pub interlaced: bool,
    /// Horizontal front porch in pixels (0 when not decoded from a DTD).
    pub h_front_porch: u16,
    /// Horizontal sync pulse width in pixels (0 when not decoded from a DTD).
    pub h_sync_width: u16,
    /// Vertical front porch in lines (0 when not decoded from a DTD).
    pub v_front_porch: u16,
    /// Vertical sync pulse width in lines (0 when not decoded from a DTD).
    pub v_sync_width: u16,
    /// Horizontal border width in pixels on each side of the active area (0 when not from a DTD).
    pub h_border: u8,
    /// Vertical border height in lines on each side of the active area (0 when not from a DTD).
    pub v_border: u8,
    /// Stereo viewing support (default `None` for non-DTD modes).
    pub stereo: StereoMode,
    /// Sync signal definition (`None` for non-DTD modes).
    pub sync: Option<SyncDefinition>,
}

/// EDID specification version and revision, decoded from base block bytes 18–19.
///
/// Most displays in use report version 1 with revision 3 or 4.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EdidVersion {
    /// EDID version number (byte 18). Always `1` for all current displays.
    pub version: u8,
    /// EDID revision number (byte 19).
    pub revision: u8,
}

impl core::fmt::Display for EdidVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}", self.version, self.revision)
    }
}
