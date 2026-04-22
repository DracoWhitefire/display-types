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

/// The source from which a [`VideoMode`] was decoded.
///
/// Populated automatically by [`vic_to_mode`][crate::cea861::vic_to_mode] and
/// [`dmt_to_mode`][crate::cea861::dmt_to_mode]; parsers that decode Detailed Timing
/// Descriptors should set it via [`VideoMode::with_source`]. `None` for modes
/// constructed directly via [`VideoMode::new`].
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModeSource {
    /// A CTA-861 Video Identification Code, as used in Short Video Descriptors,
    /// the Y420 Video Data Block, and the Y420 Capability Map Data Block.
    Vic(u8),
    /// A VESA Display Monitor Timings identifier (0x01–0x58).
    DmtId(u16),
    /// Zero-based index of a Detailed Timing Descriptor within its containing EDID block.
    DtdIndex(u8),
}

/// A display refresh rate expressed as an exact rational number (numerator/denominator in Hz).
///
/// Integer rates (60 Hz, 120 Hz, etc.) use `denom = 1`. NTSC-derived fractional rates use
/// `denom = 1001` (e.g. 60000/1001 ≈ 59.94 Hz, 24000/1001 ≈ 23.976 Hz).
///
/// Stored in lowest terms: both constructors apply GCD reduction so that `==` and `Ord`
/// comparisons are correct without cross-multiplication.
///
/// Use [`RefreshRate::integral`] for integer rates and [`RefreshRate::fractional`] for all
/// others. `From<u32>` and `From<u16>` are implemented as `integral` conversions, so
/// integer literals work wherever `impl Into<RefreshRate>` is accepted.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Eq)]
pub struct RefreshRate {
    /// Numerator in Hz.
    pub numer: u32,
    /// Denominator (1 for integer rates, 1001 for NTSC-derived fractional rates, etc.).
    pub denom: u32,
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

impl RefreshRate {
    /// Constructs an integer refresh rate (e.g. `RefreshRate::integral(60)` → 60/1).
    pub fn integral(hz: u32) -> Self {
        Self {
            numer: hz,
            denom: 1,
        }
    }

    /// Constructs an exact rational refresh rate, reduced to lowest terms.
    ///
    /// # Panics
    ///
    /// Panics if `denom` is zero.
    pub fn fractional(numer: u32, denom: u32) -> Self {
        assert!(denom != 0, "RefreshRate denominator must not be zero");
        let g = gcd(numer, denom);
        Self {
            numer: numer / g,
            denom: denom / g,
        }
    }

    /// Returns the refresh rate as `f64`.
    pub fn as_f64(self) -> f64 {
        self.numer as f64 / self.denom as f64
    }
}

impl Default for RefreshRate {
    fn default() -> Self {
        Self::integral(0)
    }
}

impl PartialEq for RefreshRate {
    fn eq(&self, other: &Self) -> bool {
        self.numer == other.numer && self.denom == other.denom
    }
}

impl PartialOrd for RefreshRate {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl core::cmp::Ord for RefreshRate {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        (self.numer as u64 * other.denom as u64).cmp(&(other.numer as u64 * self.denom as u64))
    }
}

impl core::fmt::Display for RefreshRate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.denom == 1 {
            write!(f, "{} Hz", self.numer)
        } else {
            write!(f, "{}/{} Hz", self.numer, self.denom)
        }
    }
}

impl From<u32> for RefreshRate {
    fn from(hz: u32) -> Self {
        Self::integral(hz)
    }
}

impl From<u16> for RefreshRate {
    fn from(hz: u16) -> Self {
        Self::integral(hz as u32)
    }
}

/// A display video mode expressed as resolution, refresh rate, and scan type.
///
/// Use [`VideoMode::new`] to construct a mode with only identity fields (the common case
/// for modes decoded from standard timing or SVD entries). Use
/// [`VideoMode::with_detailed_timing`] to add the blanking-interval and signal fields
/// available from a Detailed Timing Descriptor or equivalent.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VideoMode {
    /// Horizontal resolution in pixels.
    pub width: u16,
    /// Vertical resolution in pixels.
    pub height: u16,
    /// Refresh rate as an exact rational number in Hz.
    pub refresh_rate: RefreshRate,
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
    /// Stereo viewing support (default [`StereoMode::None`] for non-DTD modes).
    pub stereo: StereoMode,
    /// Sync signal definition (`None` for non-DTD modes).
    pub sync: Option<SyncDefinition>,
    /// Pixel clock in kHz (`None` for modes not decoded from a Detailed Timing Descriptor).
    pub pixel_clock_khz: Option<u32>,
    /// The source from which this mode was decoded, if known.
    ///
    /// `None` for modes constructed directly via [`VideoMode::new`] without a table lookup.
    pub source: Option<ModeSource>,
}

impl VideoMode {
    /// Constructs a `VideoMode` with the given identity fields.
    ///
    /// All blanking-interval fields (`h_front_porch`, `h_sync_width`, `v_front_porch`,
    /// `v_sync_width`, `h_border`, `v_border`) default to `0`, `stereo` defaults to
    /// [`StereoMode::None`], and `sync` defaults to `None`. Use
    /// [`with_detailed_timing`][Self::with_detailed_timing] to set those fields when
    /// decoding from a Detailed Timing Descriptor.
    pub fn new(
        width: u16,
        height: u16,
        refresh_rate: impl Into<RefreshRate>,
        interlaced: bool,
    ) -> Self {
        Self {
            width,
            height,
            refresh_rate: refresh_rate.into(),
            interlaced,
            ..Self::default()
        }
    }

    /// Sets the exact pixel clock in kHz, returning the updated mode.
    ///
    /// Use this when constructing a [`VideoMode`] from hardware timing registers or a
    /// known-good mode table entry, where the exact pixel clock is available but full
    /// Detailed Timing Descriptor fields are not. The supplied clock is returned verbatim
    /// by [`pixel_clock_khz`][crate::pixel_clock_khz], bypassing the CVT-RB fallback
    /// estimate.
    ///
    /// ```
    /// use display_types::VideoMode;
    /// use display_types::pixel_clock_khz;
    ///
    /// // Custom panel: 1920×1200 @ 60 Hz, exact pixel clock from PLL register.
    /// let mode = VideoMode::new(1920, 1200, 60u32, false).with_pixel_clock(154_000);
    /// assert_eq!(pixel_clock_khz(&mode), 154_000);
    /// ```
    pub fn with_pixel_clock(mut self, pixel_clock_khz: u32) -> Self {
        self.pixel_clock_khz = Some(pixel_clock_khz);
        self
    }

    /// Sets the mode source, returning the updated mode.
    ///
    /// Called automatically by [`vic_to_mode`][crate::cea861::vic_to_mode] and
    /// [`dmt_to_mode`][crate::cea861::dmt_to_mode]. Parsers decoding Detailed Timing
    /// Descriptors should call `.with_source(ModeSource::DtdIndex(n))` so that the
    /// descriptor's position survives into negotiated output.
    pub fn with_source(mut self, source: ModeSource) -> Self {
        self.source = Some(source);
        self
    }

    /// Adds blanking-interval and signal fields decoded from a Detailed Timing Descriptor
    /// or equivalent source, returning the updated mode.
    ///
    /// The 9-parameter count mirrors the DTD fields directly (EDID §3.10.3 / DisplayID §4.4).
    #[allow(clippy::too_many_arguments)]
    pub fn with_detailed_timing(
        mut self,
        pixel_clock_khz: u32,
        h_front_porch: u16,
        h_sync_width: u16,
        v_front_porch: u16,
        v_sync_width: u16,
        h_border: u8,
        v_border: u8,
        stereo: StereoMode,
        sync: Option<SyncDefinition>,
    ) -> Self {
        self.pixel_clock_khz = Some(pixel_clock_khz);
        self.h_front_porch = h_front_porch;
        self.h_sync_width = h_sync_width;
        self.v_front_porch = v_front_porch;
        self.v_sync_width = v_sync_width;
        self.h_border = h_border;
        self.v_border = v_border;
        self.stereo = stereo;
        self.sync = sync;
        self
    }
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

/// Trait for typed data stored in [`DisplayCapabilities::extension_data`] by custom handlers.
///
/// A blanket implementation covers any type that is `Any + Debug + Send + Sync`, so consumers
/// do not need to implement this trait manually — `#[derive(Debug)]` on a `Send + Sync` type
/// is sufficient.
#[cfg(any(feature = "alloc", feature = "std"))]
pub trait ExtensionData: core::any::Any + core::fmt::Debug + Send + Sync {
    /// Returns `self` as `&dyn Any` to enable downcasting.
    fn as_any(&self) -> &dyn core::any::Any;
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T: core::any::Any + core::fmt::Debug + Send + Sync> ExtensionData for T {
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
}

/// Consumer-facing display capability model produced by a display data parser.
///
/// All fields defined by the relevant specification are decoded and exposed here.
/// No field is omitted because it appears obscure or unlikely to be needed — that
/// judgement belongs to the consumer, not the library.
///
/// Fields are `Option` where the underlying data may be absent or undecodable.
/// `None` means the value was not present or could not be reliably determined; it does
/// not imply the field is unimportant. The library never invents or defaults data.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct DisplayCapabilities {
    /// Three-character PNP manufacturer ID (e.g. `GSM` for LG, `SAM` for Samsung).
    pub manufacturer: Option<crate::manufacture::ManufacturerId>,
    /// Manufacture date or model year.
    pub manufacture_date: Option<crate::manufacture::ManufactureDate>,
    /// EDID specification version and revision.
    pub edid_version: Option<EdidVersion>,
    /// Manufacturer-assigned product code.
    pub product_code: Option<u16>,
    /// Manufacturer-assigned serial number, if encoded numerically in the base block.
    pub serial_number: Option<u32>,
    /// Serial number string from the monitor serial number descriptor (`0xFF`), if present.
    pub serial_number_string: Option<crate::manufacture::MonitorString>,
    /// Human-readable display name from the monitor name descriptor, if present.
    pub display_name: Option<crate::manufacture::MonitorString>,
    /// Unspecified ASCII text strings from `0xFE` descriptors, in descriptor slot order.
    ///
    /// Up to four entries (one per descriptor slot). Each slot is `None` if the corresponding
    /// descriptor was not a `0xFE` entry.
    pub unspecified_text: [Option<crate::manufacture::MonitorString>; 4],
    /// Additional white points from the `0xFB` descriptor.
    ///
    /// Up to two entries (the EDID `0xFB` descriptor has two fixed slots). Each slot is
    /// `None` if the corresponding entry was unused (index byte `0x00`).
    pub white_points: [Option<crate::color::WhitePoint>; 2],
    /// `true` if the display uses a digital input interface.
    pub digital: bool,
    /// Color bit depth per primary channel.
    /// `None` for analog displays or when the field is undefined or reserved.
    pub color_bit_depth: Option<crate::color::ColorBitDepth>,
    /// Physical display technology (e.g. TFT, OLED, PDP).
    /// `None` when the Display Device Data Block is absent.
    pub display_technology: Option<crate::panel::DisplayTechnology>,
    /// Technology-specific sub-type code (raw, 0–15).
    /// `None` when the Display Device Data Block is absent.
    pub display_subtype: Option<u8>,
    /// Panel operating mode (continuous or non-continuous refresh).
    /// `None` when the Display Device Data Block is absent.
    pub operating_mode: Option<crate::panel::OperatingMode>,
    /// Backlight type.
    /// `None` when the Display Device Data Block is absent.
    pub backlight_type: Option<crate::panel::BacklightType>,
    /// Whether the panel uses a Data Enable (DE) signal.
    /// `None` when the Display Device Data Block is absent.
    pub data_enable_used: Option<bool>,
    /// Data Enable signal polarity: `true` = positive, `false` = negative.
    /// Valid only when `data_enable_used` is `Some(true)`.
    /// `None` when the Display Device Data Block is absent.
    pub data_enable_positive: Option<bool>,
    /// Native pixel format `(width_px, height_px)`.
    /// `None` when the Display Device Data Block is absent or either dimension is zero.
    pub native_pixels: Option<(u16, u16)>,
    /// Panel aspect ratio encoded as `(AR − 1) × 100` (raw byte).
    /// For example `77` represents approximately 16:9 (AR ≈ 1.77). `None` when the block is absent.
    pub panel_aspect_ratio_100: Option<u8>,
    /// Physical mounting orientation of the panel.
    /// `None` when the Display Device Data Block is absent.
    pub physical_orientation: Option<crate::panel::PhysicalOrientation>,
    /// Panel rotation capability.
    /// `None` when the Display Device Data Block is absent.
    pub rotation_capability: Option<crate::panel::RotationCapability>,
    /// Location of the zero (origin) pixel in the framebuffer.
    /// `None` when the Display Device Data Block is absent.
    pub zero_pixel_location: Option<crate::panel::ZeroPixelLocation>,
    /// Fast-scan direction relative to H-sync.
    /// `None` when the Display Device Data Block is absent.
    pub scan_direction: Option<crate::panel::ScanDirection>,
    /// Sub-pixel color filter arrangement.
    /// `None` when the Display Device Data Block is absent.
    pub subpixel_layout: Option<crate::panel::SubpixelLayout>,
    /// Pixel pitch `(horizontal_hundredths_mm, vertical_hundredths_mm)` in 0.01 mm units.
    /// `None` when the Display Device Data Block is absent or either pitch is zero.
    pub pixel_pitch_hundredths_mm: Option<(u8, u8)>,
    /// Pixel response time in milliseconds.
    /// `None` when the Display Device Data Block is absent or the value is zero.
    pub pixel_response_time_ms: Option<u8>,
    /// Interface power sequencing timing parameters.
    /// `None` when the Interface Power Sequencing Block is absent.
    pub power_sequencing: Option<crate::panel::PowerSequencing>,
    /// Display luminance transfer function.
    /// `None` when the Transfer Characteristics Block is absent.
    #[cfg(any(feature = "alloc", feature = "std"))]
    pub transfer_characteristic: Option<crate::transfer::DisplayIdTransferCharacteristic>,
    /// Physical display interface capabilities.
    /// `None` when the Display Interface Data Block is absent.
    pub display_id_interface: Option<crate::panel::DisplayIdInterface>,
    /// Stereo display interface parameters.
    /// `None` when the Stereo Display Interface Data Block is absent.
    pub stereo_interface: Option<crate::panel::DisplayIdStereoInterface>,
    /// Tiled display topology.
    /// `None` when the Tiled Display Topology Data Block is absent.
    pub tiled_topology: Option<crate::panel::DisplayIdTiledTopology>,
    /// CIE xy chromaticity coordinates for the color primaries and white point.
    pub chromaticity: crate::color::Chromaticity,
    /// Display gamma. `None` if the display did not specify a gamma value.
    pub gamma: Option<crate::color::DisplayGamma>,
    /// Display feature support flags.
    pub display_features: Option<crate::features::DisplayFeatureFlags>,
    /// Supported color encoding formats. Only populated for EDID 1.4+ digital displays.
    pub digital_color_encoding: Option<crate::color::DigitalColorEncoding>,
    /// Color type for analog displays; `None` for the undefined value (`0b11`).
    pub analog_color_type: Option<crate::color::AnalogColorType>,
    /// Video interface type.
    /// `None` for analog displays or when the field is undefined or reserved.
    pub video_interface: Option<crate::input::VideoInterface>,
    /// Analog sync and video white levels. Only populated for analog displays.
    pub analog_sync_level: Option<crate::input::AnalogSyncLevel>,
    /// Physical screen dimensions or aspect ratio.
    /// `None` when both bytes are zero (undefined).
    pub screen_size: Option<crate::screen::ScreenSize>,
    /// Minimum supported vertical refresh rate in Hz.
    pub min_v_rate: Option<u16>,
    /// Maximum supported vertical refresh rate in Hz.
    pub max_v_rate: Option<u16>,
    /// Minimum supported horizontal scan rate in kHz.
    pub min_h_rate_khz: Option<u16>,
    /// Maximum supported horizontal scan rate in kHz.
    pub max_h_rate_khz: Option<u16>,
    /// Maximum pixel clock in MHz.
    pub max_pixel_clock_mhz: Option<u16>,
    /// Physical image area dimensions in millimetres `(width_mm, height_mm)`.
    ///
    /// More precise than [`screen_size`][Self::screen_size] (which is in cm).
    /// `None` when all DTD image-size fields are zero.
    pub preferred_image_size_mm: Option<(u16, u16)>,
    /// Video timing formula reported in the display range limits descriptor.
    pub timing_formula: Option<crate::timing::TimingFormula>,
    /// DCM polynomial coefficients.
    pub color_management: Option<crate::color::ColorManagementData>,
    /// Video modes decoded from the display data.
    #[cfg(any(feature = "alloc", feature = "std"))]
    pub supported_modes: crate::prelude::Vec<VideoMode>,
    /// Non-fatal conditions collected from the parser and all handlers.
    ///
    /// Not serialized — use a custom handler to map warnings to a serializable form.
    #[cfg(any(feature = "alloc", feature = "std"))]
    #[cfg_attr(feature = "serde", serde(skip))]
    pub warnings: crate::prelude::Vec<ParseWarning>,
    /// Typed data attached by extension handlers, keyed by extension tag byte.
    ///
    /// Uses a `Vec` of `(tag, data)` pairs rather than a `HashMap` so that this field is
    /// available in `alloc`-only (no_std) builds. The number of distinct extension tags in
    /// any real EDID is small enough that linear scan is negligible.
    ///
    /// Not serialized — use a custom handler to map this to a serializable form.
    #[cfg(any(feature = "alloc", feature = "std"))]
    #[cfg_attr(feature = "serde", serde(skip))]
    pub extension_data: crate::prelude::Vec<(u8, crate::prelude::Arc<dyn ExtensionData>)>,
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl DisplayCapabilities {
    /// Returns an iterator over all collected warnings.
    pub fn iter_warnings(&self) -> impl Iterator<Item = &ParseWarning> {
        self.warnings.iter()
    }

    /// Appends a warning, wrapping it in a [`ParseWarning`].
    pub fn push_warning(&mut self, w: impl core::error::Error + Send + Sync + 'static) {
        self.warnings.push(crate::prelude::Arc::new(w));
    }

    /// Store typed data from a handler, keyed by an extension tag.
    /// Replaces any previously stored entry for the same tag.
    pub fn set_extension_data<T: ExtensionData>(&mut self, tag: u8, data: T) {
        if let Some(entry) = self.extension_data.iter_mut().find(|(t, _)| *t == tag) {
            entry.1 = crate::prelude::Arc::new(data);
        } else {
            self.extension_data
                .push((tag, crate::prelude::Arc::new(data)));
        }
    }

    /// Retrieve typed data previously stored by a handler for the given tag.
    /// Returns `None` if no data is stored for the tag or the type does not match.
    pub fn get_extension_data<T: core::any::Any>(&self, tag: u8) -> Option<&T> {
        self.extension_data
            .iter()
            .find(|(t, _)| *t == tag)
            // `**data` deref-chains through `&` then through Arc's Deref to reach
            // `dyn ExtensionData`, forcing vtable dispatch for `as_any()`.
            // Calling `.as_any()` on `&Arc<dyn ExtensionData>` would hit the blanket
            // `ExtensionData` impl for Arc itself and return the wrong TypeId.
            .and_then(|(_, data)| (**data).as_any().downcast_ref::<T>())
    }
}
