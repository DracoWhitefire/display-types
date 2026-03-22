/// Decoded VESA Display Device Data Block (extended tag `0x02`).
///
/// A fixed 30-byte payload describing the physical and electrical characteristics
/// of the display, per the VESA Display Device Data Block (DDDB) Standard, Version 1.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VesaDisplayDeviceBlock {
    /// Interface type code (bits 7:4). 0=Analog, 1=LVDS, 3=DVI-D, 6=HDMI-A, 9=DP, etc.
    pub interface_type: u8,
    /// Number of lanes/channels (bits 3:0). For analog interfaces this is a subtype code.
    pub num_links: u8,
    /// Interface standard version (bits 7:4 of byte 0x03).
    pub interface_version: u8,
    /// Interface standard release (bits 3:0 of byte 0x03).
    pub interface_release: u8,
    /// Content protection code (byte 0x04). 0=none, 1=HDCP, 2=DTCP, 3=DPCP.
    pub content_protection: u8,
    /// Minimum supported clock frequency per link in MHz (6-bit, range 0–63).
    pub min_clock_mhz: u8,
    /// Maximum supported clock frequency per link in MHz (10-bit, range 0–1023).
    pub max_clock_mhz: u16,
    /// Native horizontal pixel count, or `None` if the display has no fixed format.
    pub native_width: Option<u16>,
    /// Native vertical pixel count, or `None` if the display has no fixed format.
    pub native_height: Option<u16>,
    /// Aspect ratio raw byte. Physical AR = (raw / 100.0) + 1.0 (long-axis / short-axis).
    pub aspect_ratio_raw: u8,
    /// Default orientation: 0=landscape, 1=portrait, 2=not_fixed, 3=undefined.
    pub default_orientation: u8,
    /// Rotation capability: 0=none, 1=90°CW, 2=90°CCW, 3=both.
    pub rotation_capability: u8,
    /// Zero pixel (scan origin) location: 0=upper-left, 1=upper-right, 2=lower-left, 3=lower-right.
    pub zero_pixel_location: u8,
    /// Scan direction: 0=undefined, 1=long-axis-fast, 2=short-axis-fast, 3=reserved.
    pub scan_direction: u8,
    /// Subpixel layout code (byte 0x0D). 0=undefined, 1=RGB-V, 2=RGB-H, etc.
    pub subpixel_layout: u8,
    /// Horizontal pixel pitch in 0.01 mm increments (range 0.00–2.55 mm).
    pub h_pitch_hundredths_mm: u8,
    /// Vertical pixel pitch in 0.01 mm increments (range 0.00–2.55 mm).
    pub v_pitch_hundredths_mm: u8,
    /// Dithering type: 0=none, 1=spatial, 2=temporal, 3=spatial+temporal.
    pub dithering: u8,
    /// Display is direct-drive — no internal scaling/de-interlacing/FRC.
    pub direct_drive: bool,
    /// Video source should not apply overdrive for this display.
    pub overdrive_not_recommended: bool,
    /// Display can de-interlace interlaced input to progressive scan.
    pub deinterlacing: bool,
    /// Audio is supported on the video interface.
    pub audio_on_video_interface: bool,
    /// Separate audio inputs are provided independently of the video interface.
    pub separate_audio_inputs: bool,
    /// Audio received on the video interface automatically overrides other audio inputs.
    pub audio_input_override: bool,
    /// Signed audio delay in milliseconds (positive = audio after video, negative = before).
    /// `None` means no delay information is provided (raw delay byte was 0x00).
    pub audio_delay_ms: Option<i16>,
    /// Frame-rate conversion capability: 0=none, 1=single-buffer, 2=double-buffer, 3=interpolation.
    pub frame_rate_conversion: u8,
    /// Maximum excursion (±FPS) from the nominal frame rate (6 bits, 0–63).
    pub frame_rate_range: u8,
    /// Native or nominal display frame rate in frames/second.
    pub native_frame_rate: u8,
    /// Color bit depth per primary on the video interface (1–16).
    pub interface_color_depth: u8,
    /// Color bit depth per primary at the display panel without temporal dithering (1–16).
    pub display_color_depth: u8,
    /// Raw bytes 0x16–0x1D: chromaticity data for up to three additional primaries.
    pub additional_chromaticities: [u8; 8],
    /// Response time in milliseconds (0 = < 1 ms, 127 = > 126 ms).
    pub response_time_ms: u8,
    /// `true` if the response time is white-to-black; `false` if black-to-white.
    pub response_time_white_to_black: bool,
    /// Percentage of active image outside the visible screen area, horizontally (0–15%).
    pub h_overscan_pct: u8,
    /// Percentage of active image outside the visible screen area, vertically (0–15%).
    pub v_overscan_pct: u8,
}

impl VesaDisplayDeviceBlock {
    /// Constructs a `VesaDisplayDeviceBlock`.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        interface_type: u8,
        num_links: u8,
        interface_version: u8,
        interface_release: u8,
        content_protection: u8,
        min_clock_mhz: u8,
        max_clock_mhz: u16,
        native_width: Option<u16>,
        native_height: Option<u16>,
        aspect_ratio_raw: u8,
        default_orientation: u8,
        rotation_capability: u8,
        zero_pixel_location: u8,
        scan_direction: u8,
        subpixel_layout: u8,
        h_pitch_hundredths_mm: u8,
        v_pitch_hundredths_mm: u8,
        dithering: u8,
        direct_drive: bool,
        overdrive_not_recommended: bool,
        deinterlacing: bool,
        audio_on_video_interface: bool,
        separate_audio_inputs: bool,
        audio_input_override: bool,
        audio_delay_ms: Option<i16>,
        frame_rate_conversion: u8,
        frame_rate_range: u8,
        native_frame_rate: u8,
        interface_color_depth: u8,
        display_color_depth: u8,
        additional_chromaticities: [u8; 8],
        response_time_ms: u8,
        response_time_white_to_black: bool,
        h_overscan_pct: u8,
        v_overscan_pct: u8,
    ) -> Self {
        Self {
            interface_type,
            num_links,
            interface_version,
            interface_release,
            content_protection,
            min_clock_mhz,
            max_clock_mhz,
            native_width,
            native_height,
            aspect_ratio_raw,
            default_orientation,
            rotation_capability,
            zero_pixel_location,
            scan_direction,
            subpixel_layout,
            h_pitch_hundredths_mm,
            v_pitch_hundredths_mm,
            dithering,
            direct_drive,
            overdrive_not_recommended,
            deinterlacing,
            audio_on_video_interface,
            separate_audio_inputs,
            audio_input_override,
            audio_delay_ms,
            frame_rate_conversion,
            frame_rate_range,
            native_frame_rate,
            interface_color_depth,
            display_color_depth,
            additional_chromaticities,
            response_time_ms,
            response_time_white_to_black,
            h_overscan_pct,
            v_overscan_pct,
        }
    }
}
