use crate::VideoMode;

/// Estimates the pixel clock in kHz for a [`VideoMode`] using the CVT Reduced Blanking (CVT-RB)
/// overhead model.
///
/// When `mode.pixel_clock_khz` is `Some`, returns that exact value directly — no estimation
/// is performed. When it is `None` (modes decoded from standard timings, established timings,
/// or SVD entries that lack a Detailed Timing Descriptor), applies the CVT-RB fixed blanking
/// parameters:
///
/// - **Horizontal blanking:** 160 pixels (CVT-RB fixed blank, VESA CVT 1.2 §2.2).
/// - **Vertical blanking:** 8 lines (minimum RB frame-height adjustment).
///
/// The resulting estimate is:
///
/// ```text
/// pixel_clock_khz ≈ (width + 160) × (height + 8) × refresh_rate_hz / 1000
/// ```
///
/// # Accuracy
///
/// CVT-RB is the dominant timing standard for modern display modes. For typical consumer
/// resolutions the estimate is within ~2% of the actual clock. Interlaced modes and
/// non-CVT timings (e.g. legacy CTA-861 formats with larger blanking) may diverge further.
///
/// For bandwidth ceiling checks this function is conservative in the direction of
/// *under*-estimating — use it to detect obvious violations, not as a substitute for an
/// exact clock when one is available.
pub fn pixel_clock_khz_cvt_rb_estimate(mode: &VideoMode) -> u32 {
    if let Some(clk) = mode.pixel_clock_khz {
        return clk;
    }
    let h_total = mode.width as u64 + 160;
    let v_total = mode.height as u64 + 8;
    (h_total * v_total * mode.refresh_rate as u64 / 1000) as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::VideoMode;

    #[test]
    fn exact_clock_returned_unchanged() {
        let mode = VideoMode::new(1920, 1080, 60, false)
            .with_detailed_timing(148_500, 88, 44, 4, 5, 0, 0, Default::default(), None);
        assert_eq!(pixel_clock_khz_cvt_rb_estimate(&mode), 148_500);
    }

    #[test]
    fn non_dtd_mode_uses_cvt_rb_formula() {
        // 1920×1080@60: (1920+160) × (1080+8) × 60 / 1000 = 135_782
        let mode = VideoMode::new(1920, 1080, 60, false);
        assert_eq!(pixel_clock_khz_cvt_rb_estimate(&mode), 135_782);
    }

    #[test]
    fn zero_refresh_rate_returns_zero() {
        let mode = VideoMode::new(1920, 1080, 0, false);
        assert_eq!(pixel_clock_khz_cvt_rb_estimate(&mode), 0);
    }
}

/// Video timing support reported in the display range limits descriptor (`0xFD`), byte 10.
///
/// Indicates which timing generation formula (if any) the display supports beyond the
/// explicitly listed modes.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimingFormula {
    /// Default GTF supported (byte 10 = `0x00`).
    ///
    /// The display accepts any timing within its range limits that satisfies the
    /// default GTF parameters. Requires bit 0 of the Feature Support byte (`0x18`) to be set.
    DefaultGtf,
    /// Range limits only; no secondary timing formula (byte 10 = `0x01`).
    ///
    /// The display supports only the video timing modes explicitly listed in the EDID.
    RangeLimitsOnly,
    /// Secondary GTF curve supported (byte 10 = `0x02`).
    ///
    /// The display accepts timings using either the default GTF or the secondary GTF curve
    /// whose parameters are stored in bytes 12–17.
    SecondaryGtf(GtfSecondaryParams),
    /// CVT timing supported (byte 10 = `0x04`), with parameters from bytes 11–17.
    ///
    /// The display accepts Coordinated Video Timings within its range limits.
    /// Requires bit 0 of the Feature Support byte (`0x18`) to be set.
    Cvt(CvtSupportParams),
}

/// GTF secondary curve parameters decoded from a display range limits descriptor (`0xFD`).
///
/// Used when [`TimingFormula::SecondaryGtf`] is active (byte 10 = `0x02`).
/// The GTF formula selects the secondary curve for horizontal frequencies at or above
/// [`start_freq_khz`][Self::start_freq_khz] and the default curve below it.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GtfSecondaryParams {
    /// Start break frequency in kHz (byte 12 × 2).
    pub start_freq_khz: u16,
    /// GTF `C` parameter (0–127); byte 13 ÷ 2.
    pub c: u8,
    /// GTF `M` parameter (0–65535); bytes 14–15, little-endian.
    pub m: u16,
    /// GTF `K` parameter (0–255); byte 16.
    pub k: u8,
    /// GTF `J` parameter (0–127); byte 17 ÷ 2.
    pub j: u8,
}

/// CVT support parameters decoded from a display range limits descriptor (`0xFD`).
///
/// Used when [`TimingFormula::Cvt`] is active (byte 10 = `0x04`).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CvtSupportParams {
    /// CVT standard version, encoded as two BCD nibbles (e.g., `0x11` = version 1.1).
    pub version: u8,
    /// Additional pixel clock precision: 6-bit value from byte 12 bits 7–2.
    ///
    /// The maximum pixel clock is: `(descriptor byte 9 × 10 MHz) − (pixel_clock_adjust × 0.25 MHz)`.
    /// When all six bits are set (`63`), byte 9 was already rounded up to the nearest 10 MHz.
    pub pixel_clock_adjust: u8,
    /// Maximum number of horizontal active pixels, or `None` if there is no limit.
    ///
    /// Computed as `8 × (byte 13 + 256 × (byte 12 bits 1–0))`. `None` when the 10-bit
    /// combined value is zero.
    pub max_h_active_pixels: Option<u16>,
    /// Aspect ratios the display supports for CVT-generated timings.
    pub supported_aspect_ratios: CvtAspectRatios,
    /// Preferred aspect ratio for CVT-generated timings, or `None` for a reserved value.
    pub preferred_aspect_ratio: Option<CvtAspectRatio>,
    /// Standard CVT blanking (normal blanking) is supported.
    pub standard_blanking: bool,
    /// Reduced CVT blanking is supported (preferred over standard blanking).
    pub reduced_blanking: bool,
    /// Display scaling capabilities.
    pub scaling: CvtScaling,
    /// Preferred vertical refresh rate in Hz, or `None` if byte 17 = `0x00` (reserved).
    pub preferred_v_rate: Option<u8>,
}

bitflags::bitflags! {
    /// Aspect ratios supported for CVT-generated timings (byte 14 of a `0xFD` descriptor).
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct CvtAspectRatios: u8 {
        /// 4∶3 aspect ratio supported.
        const R4_3   = 0x80;
        /// 16∶9 aspect ratio supported.
        const R16_9  = 0x40;
        /// 16∶10 aspect ratio supported.
        const R16_10 = 0x20;
        /// 5∶4 aspect ratio supported.
        const R5_4   = 0x10;
        /// 15∶9 aspect ratio supported.
        const R15_9  = 0x08;
    }
}

bitflags::bitflags! {
    /// Display scaling capabilities reported in byte 16 of a `0xFD` CVT descriptor.
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct CvtScaling: u8 {
        /// Input horizontal active pixels can exceed the display's preferred horizontal count.
        const HORIZONTAL_SHRINK  = 0x80;
        /// Input horizontal active pixels can be fewer than the display's preferred horizontal count.
        const HORIZONTAL_STRETCH = 0x40;
        /// Input vertical active lines can exceed the display's preferred vertical count.
        const VERTICAL_SHRINK    = 0x20;
        /// Input vertical active lines can be fewer than the display's preferred vertical count.
        const VERTICAL_STRETCH   = 0x10;
    }
}

/// Preferred aspect ratio for CVT-generated timings, decoded from byte 15 bits 7–5.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CvtAspectRatio {
    /// 4∶3 preferred aspect ratio.
    R4_3,
    /// 16∶9 preferred aspect ratio.
    R16_9,
    /// 16∶10 preferred aspect ratio.
    R16_10,
    /// 5∶4 preferred aspect ratio.
    R5_4,
    /// 15∶9 preferred aspect ratio.
    R15_9,
}
