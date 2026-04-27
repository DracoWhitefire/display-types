use crate::{CvtAlgorithm, RefreshRate, VideoMode};

/// Returns the pixel clock in kHz for a [`VideoMode`].
///
/// When `mode.pixel_clock_khz` is `Some` (set from a Detailed Timing Descriptor), returns
/// that exact value directly. When it is `None` (modes decoded from standard timings,
/// established timings, or SVD entries that lack a DTD), falls back to a CVT Reduced
/// Blanking estimate:
///
/// - **Horizontal blanking:** 160 pixels (CVT-RB fixed blank, VESA CVT 1.2 §2.2).
/// - **Vertical blanking:** 8 lines (minimum RB frame-height adjustment).
///
/// ```text
/// pixel_clock_khz ≈ (width + 160) × (height + 8) × refresh_rate / 1000
/// ```
///
/// Returns `0` when neither `mode.pixel_clock_khz` nor `mode.refresh_rate` is set.
///
/// # Accuracy of the fallback estimate
///
/// CVT-RB is the dominant timing standard for modern display modes. For typical consumer
/// resolutions the estimate is within ~2% of the actual clock. HDMI Forum-specified CTA
/// modes (e.g. 4K@60, VIC 97) use larger blanking than CVT-RB predicts and may be
/// under-estimated by ~10–15%, which can produce false accepts in bandwidth ceiling checks.
/// Interlaced modes diverge further.
///
/// The fallback is only used when no exact clock is available. Prefer populating
/// `pixel_clock_khz` from the EDID Detailed Timing Descriptor wherever possible.
pub fn pixel_clock_khz(mode: &VideoMode) -> u32 {
    if let Some(clk) = mode.pixel_clock_khz {
        return clk;
    }
    let Some(rr) = mode.refresh_rate else {
        return 0;
    };
    let h_total = mode.width as u64 + 160;
    let v_total = mode.height as u64 + 8;
    let numer = rr.numer() as u64;
    let denom = rr.denom() as u64;
    (h_total * v_total * numer / (denom * 1000)) as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::VideoMode;

    #[test]
    fn exact_clock_returned_unchanged() {
        let mode = VideoMode::new(1920, 1080, 60u32, false).with_detailed_timing(
            148_500,
            88,
            44,
            4,
            5,
            0,
            0,
            Default::default(),
            None,
        );
        assert_eq!(pixel_clock_khz(&mode), 148_500);
    }

    #[test]
    fn with_pixel_clock_bypasses_estimate() {
        let mode = VideoMode::new(1920, 1200, 60u32, false).with_pixel_clock(154_000);
        assert_eq!(pixel_clock_khz(&mode), 154_000);
    }

    #[test]
    fn non_dtd_mode_uses_cvt_rb_formula() {
        // 1920×1080@60: (1920+160) × (1080+8) × 60 / 1000 = 135_782
        let mode = VideoMode::new(1920, 1080, 60u32, false);
        assert_eq!(pixel_clock_khz(&mode), 135_782);
    }

    #[test]
    fn zero_refresh_rate_returns_zero() {
        let mode = VideoMode::new(1920, 1080, 0u32, false);
        assert_eq!(pixel_clock_khz(&mode), 0);
    }

    #[test]
    fn unset_refresh_rate_returns_zero() {
        let mode = VideoMode {
            width: 1920,
            height: 1080,
            refresh_rate: None,
            ..Default::default()
        };
        assert_eq!(pixel_clock_khz(&mode), 0);
    }
}

/// Pixel clock and blanking parameters computed from a CVT formula.
///
/// All fields are derived from `(width, height, refresh_rate, cvt_algorithm)`. Designed
/// to feed [`VideoMode::with_detailed_timing`][crate::VideoMode::with_detailed_timing]
/// directly: the five fields it carries map 1:1 onto that builder's
/// `pixel_clock_khz` / `h_front_porch` / `h_sync_width` / `v_front_porch` / `v_sync_width`
/// arguments. `h_total` and `v_total` are exposed for sanity checks (e.g. bandwidth
/// estimation against the CVT-rounded clock).
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComputedTiming {
    /// Pixel clock in kHz, rounded down to the algorithm's clock step (CVT-RB v1: 250 kHz).
    pub pixel_clock_khz: u32,
    /// `width + horizontal blanking`. Useful for bandwidth checks.
    pub h_total: u16,
    /// `height + vertical blanking`. Useful for bandwidth checks.
    pub v_total: u16,
    /// Horizontal front porch in pixels.
    pub h_front_porch: u16,
    /// Horizontal sync pulse width in pixels.
    pub h_sync_width: u16,
    /// Vertical front porch in lines.
    pub v_front_porch: u16,
    /// Vertical sync pulse width in lines.
    pub v_sync_width: u16,
}

// CVT-RB v1 constants (VESA CVT 1.1 §3.4 "Reduced Blanking" timing).
const RB_V1_CLOCK_STEP_KHZ: u32 = 250; // 0.25 MHz pixel-clock granularity
const RB_V1_MIN_V_BLANK_US: u32 = 460; // microseconds, fixed
const RB_V1_H_BLANK: u16 = 160; // pixels, fixed total H blanking
const RB_V1_H_SYNC: u16 = 32; // pixels, fixed
const RB_V1_H_BPORCH: u16 = 80; // pixels, fixed
const RB_V1_H_FPORCH: u16 = RB_V1_H_BLANK - RB_V1_H_SYNC - RB_V1_H_BPORCH; // 48
const RB_V1_V_FPORCH: u16 = 3; // lines, fixed
const RB_V1_V_SYNC: u16 = 4; // lines, fixed
const RB_V1_MIN_V_BPORCH: u16 = 6; // lines, lower bound

// CVT-RB v2 constants (VESA CVT 1.2 §4 "Reduced Blanking version 2" timing).
const RB_V2_CLOCK_STEP_KHZ: u32 = 1; // 0.001 MHz (1 kHz) pixel-clock granularity
const RB_V2_MIN_V_BLANK_US: u32 = 460; // microseconds, fixed
const RB_V2_H_BLANK: u16 = 80; // pixels, fixed total H blanking (half of RB v1)
const RB_V2_H_SYNC: u16 = 32; // pixels, fixed
const RB_V2_H_BPORCH: u16 = 40; // pixels, fixed
const RB_V2_H_FPORCH: u16 = RB_V2_H_BLANK - RB_V2_H_SYNC - RB_V2_H_BPORCH; // 8
const RB_V2_MIN_V_FPORCH: u16 = 1; // lines, lower bound (slack lives here, not in back porch)
const RB_V2_V_SYNC: u16 = 8; // lines, fixed (v1 used 4)
const RB_V2_V_BPORCH: u16 = 6; // lines, fixed (v1 had a 6-line minimum but variable back porch)

/// Computes pixel clock and blanking parameters for a DisplayID 2.x Type IX
/// (Formula-Based Timing) descriptor using the named CVT variant.
///
/// Returns `None` for:
/// - degenerate input (`width == 0`, `height == 0`, non-positive or non-finite refresh rate)
/// - algorithms not yet implemented (currently CVT-RB v2/v3 and the "reduced blanking with
///   CVT-RB1/RB2" encodings; see `doc/roadmap.md` in the piaf repo)
/// - the `Reserved(_)` algorithm encoding
///
/// The returned [`ComputedTiming`] feeds [`VideoMode::with_detailed_timing`] directly.
///
/// # Algorithm coverage
///
/// | `CvtAlgorithm` variant       | Status |
/// |------------------------------|--------|
/// | `CvtRb1`                     | implemented (VESA CVT 1.1 §3.4) |
/// | `CvtRb2`                     | implemented (VESA CVT 1.2 §4) |
/// | `CvtRb3`                     | not yet — returns `None` |
/// | `ReducedBlankingCvtRb1`      | not yet — returns `None` |
/// | `ReducedBlankingCvtRb2`      | not yet — returns `None` |
/// | `Reserved(_)`                | always `None` |
pub fn compute_type_ix_timing(
    width: u16,
    height: u16,
    refresh_rate: RefreshRate,
    algorithm: CvtAlgorithm,
) -> Option<ComputedTiming> {
    match algorithm {
        CvtAlgorithm::CvtRb1 => cvt_rb_v1(width, height, refresh_rate),
        CvtAlgorithm::CvtRb2 => cvt_rb_v2(width, height, refresh_rate),
        // CVT-RB v3 and the "reduced blanking with CVT-RB1/RB2" variants are not
        // yet implemented. Reserved(_) values are by definition unevaluable.
        _ => None,
    }
}

/// CVT-RB v1 evaluator. See [`compute_type_ix_timing`] for the public entry point.
fn cvt_rb_v1(width: u16, height: u16, refresh_rate: RefreshRate) -> Option<ComputedTiming> {
    if width == 0 || height == 0 {
        return None;
    }
    let v_field_rate_hz = refresh_rate.as_f64();
    if !v_field_rate_hz.is_finite() || v_field_rate_hz <= 0.0 {
        return None;
    }

    let v_active = u32::from(height);
    let frame_period_us = 1_000_000.0 / v_field_rate_hz;

    // Horizontal line period estimate, used to derive how many lines fit in the fixed
    // 460 µs minimum vertical blanking.
    let h_period_est_us = (frame_period_us - f64::from(RB_V1_MIN_V_BLANK_US))
        / f64::from(v_active + u32::from(RB_V1_V_FPORCH));
    if h_period_est_us <= 0.0 {
        return None; // refresh too high to fit RB1's minimum blanking budget
    }

    let vbi_lines = (f64::from(RB_V1_MIN_V_BLANK_US) / h_period_est_us).ceil() as u32;
    let rb_min_vbi =
        u32::from(RB_V1_V_FPORCH) + u32::from(RB_V1_V_SYNC) + u32::from(RB_V1_MIN_V_BPORCH);
    let actual_vbi_lines = vbi_lines.max(rb_min_vbi);

    let v_total = v_active + actual_vbi_lines;
    let h_total = u32::from(width) + u32::from(RB_V1_H_BLANK);

    // Pixel clock: V_FIELD_RATE × V_TOTAL × H_TOTAL, floored to the 250 kHz step.
    let pixel_clock_hz = v_field_rate_hz * f64::from(v_total) * f64::from(h_total);
    let pixel_clock_khz_steps =
        (pixel_clock_hz / 1000.0 / f64::from(RB_V1_CLOCK_STEP_KHZ)).floor() as u32;
    let pixel_clock_khz = pixel_clock_khz_steps * RB_V1_CLOCK_STEP_KHZ;

    // Sanity: H_TOTAL and V_TOTAL must fit in u16 for VideoMode.
    let h_total = u16::try_from(h_total).ok()?;
    let v_total = u16::try_from(v_total).ok()?;

    Some(ComputedTiming {
        pixel_clock_khz,
        h_total,
        v_total,
        h_front_porch: RB_V1_H_FPORCH,
        h_sync_width: RB_V1_H_SYNC,
        v_front_porch: RB_V1_V_FPORCH,
        v_sync_width: RB_V1_V_SYNC,
    })
}

/// CVT-RB v2 evaluator. See [`compute_type_ix_timing`] for the public entry point.
///
/// Differences from v1: half the horizontal blanking (80 vs 160 px), 1 kHz pixel-clock
/// step (vs 0.25 MHz), V_SYNC widened to 8 lines (was 4), V_BPORCH fixed at 6 lines
/// (was a 6-line minimum), and the variable slack lives in V_FPORCH (≥ 1 line) rather
/// than V_BPORCH.
fn cvt_rb_v2(width: u16, height: u16, refresh_rate: RefreshRate) -> Option<ComputedTiming> {
    if width == 0 || height == 0 {
        return None;
    }
    let v_field_rate_hz = refresh_rate.as_f64();
    if !v_field_rate_hz.is_finite() || v_field_rate_hz <= 0.0 {
        return None;
    }

    let v_active = u32::from(height);
    let frame_period_us = 1_000_000.0 / v_field_rate_hz;

    // RB v2 doesn't add V_FPORCH to the divisor (unlike v1's `(v_active + V_FPORCH)`).
    let h_period_est_us = (frame_period_us - f64::from(RB_V2_MIN_V_BLANK_US)) / f64::from(v_active);
    if h_period_est_us <= 0.0 {
        return None; // refresh too high to fit RB v2's minimum blanking budget
    }

    let vbi_lines = (f64::from(RB_V2_MIN_V_BLANK_US) / h_period_est_us).ceil() as u32;
    let rb_min_vbi =
        u32::from(RB_V2_MIN_V_FPORCH) + u32::from(RB_V2_V_SYNC) + u32::from(RB_V2_V_BPORCH);
    let actual_vbi_lines = vbi_lines.max(rb_min_vbi);

    let v_total = v_active + actual_vbi_lines;
    let h_total = u32::from(width) + u32::from(RB_V2_H_BLANK);

    // Pixel clock: V_FIELD_RATE × V_TOTAL × H_TOTAL, floored to the 1 kHz step.
    let pixel_clock_hz = v_field_rate_hz * f64::from(v_total) * f64::from(h_total);
    let pixel_clock_khz_steps =
        (pixel_clock_hz / 1000.0 / f64::from(RB_V2_CLOCK_STEP_KHZ)).floor() as u32;
    let pixel_clock_khz = pixel_clock_khz_steps * RB_V2_CLOCK_STEP_KHZ;

    // V_FPORCH carries the slack in v2: VBI = V_FPORCH + V_SYNC + V_BPORCH.
    let v_front_porch_u32 = actual_vbi_lines - u32::from(RB_V2_V_SYNC) - u32::from(RB_V2_V_BPORCH);
    let v_front_porch = u16::try_from(v_front_porch_u32).ok()?;

    // Sanity: H_TOTAL and V_TOTAL must fit in u16 for VideoMode.
    let h_total = u16::try_from(h_total).ok()?;
    let v_total = u16::try_from(v_total).ok()?;

    Some(ComputedTiming {
        pixel_clock_khz,
        h_total,
        v_total,
        h_front_porch: RB_V2_H_FPORCH,
        h_sync_width: RB_V2_H_SYNC,
        v_front_porch,
        v_sync_width: RB_V2_V_SYNC,
    })
}

#[cfg(test)]
mod cvt_tests {
    use super::*;

    #[test]
    fn cvt_rb_v1_1920x1080_at_60() {
        // Canonical CVT-RB v1 mode. VESA-published value: 138.500 MHz pixel clock,
        // h_total = 2080, v_total = 1111.
        let t = compute_type_ix_timing(1920, 1080, RefreshRate::integral(60), CvtAlgorithm::CvtRb1)
            .expect("CVT-RB v1 must produce a timing");
        assert_eq!(t.pixel_clock_khz, 138_500);
        assert_eq!(t.h_total, 2080);
        assert_eq!(t.v_total, 1111);
        assert_eq!(t.h_front_porch, 48);
        assert_eq!(t.h_sync_width, 32);
        assert_eq!(t.v_front_porch, 3);
        assert_eq!(t.v_sync_width, 4);
    }

    #[test]
    fn cvt_rb_v1_2560x1440_at_60() {
        // CVT-RB v1 reference: 241.500 MHz, h_total = 2720, v_total = 1481.
        let t = compute_type_ix_timing(2560, 1440, RefreshRate::integral(60), CvtAlgorithm::CvtRb1)
            .expect("CVT-RB v1 must produce a timing");
        assert_eq!(t.pixel_clock_khz, 241_500);
        assert_eq!(t.h_total, 2720);
        assert_eq!(t.v_total, 1481);
    }

    #[test]
    fn cvt_rb_v1_3840x2160_at_30() {
        // CVT-RB v1 reference: 262.750 MHz, h_total = 4000, v_total = 2191.
        let t = compute_type_ix_timing(3840, 2160, RefreshRate::integral(30), CvtAlgorithm::CvtRb1)
            .expect("CVT-RB v1 must produce a timing");
        assert_eq!(t.pixel_clock_khz, 262_750);
        assert_eq!(t.h_total, 4000);
        assert_eq!(t.v_total, 2191);
    }

    #[test]
    fn cvt_rb_v1_zero_width_returns_none() {
        assert!(
            compute_type_ix_timing(0, 1080, RefreshRate::integral(60), CvtAlgorithm::CvtRb1)
                .is_none()
        );
    }

    #[test]
    fn cvt_rb_v1_zero_height_returns_none() {
        assert!(
            compute_type_ix_timing(1920, 0, RefreshRate::integral(60), CvtAlgorithm::CvtRb1)
                .is_none()
        );
    }

    #[test]
    fn cvt_rb_v1_unreachable_refresh_returns_none() {
        // Refresh so high that frame period is below the 460 µs RB minimum.
        // 1/3000 s = 333 µs < 460 µs → no time for active video.
        assert!(
            compute_type_ix_timing(
                1920,
                1080,
                RefreshRate::integral(3000),
                CvtAlgorithm::CvtRb1
            )
            .is_none()
        );
    }

    #[test]
    fn cvt_rb_v2_1920x1080_at_60() {
        // CVT-RB v2 1920×1080@60: half the H blanking of v1, slack in V_FPORCH.
        // h_total = 2000, v_total = 1111 → 60 × 2000 × 1111 = 133_320_000 Hz → 133_320 kHz.
        let t = compute_type_ix_timing(1920, 1080, RefreshRate::integral(60), CvtAlgorithm::CvtRb2)
            .expect("CVT-RB v2 must produce a timing");
        assert_eq!(t.pixel_clock_khz, 133_320);
        assert_eq!(t.h_total, 2000);
        assert_eq!(t.v_total, 1111);
        assert_eq!(t.h_front_porch, 8);
        assert_eq!(t.h_sync_width, 32);
        // VBI = 31, V_SYNC = 8, V_BPORCH = 6 → V_FPORCH = 31 - 8 - 6 = 17.
        assert_eq!(t.v_front_porch, 17);
        assert_eq!(t.v_sync_width, 8);
    }

    #[test]
    fn cvt_rb_v2_2560x1440_at_120() {
        // 144/240 Hz panels typically use RB v2. h_total = 2640, v_total = 1525.
        // 120 × 2640 × 1525 = 483_120_000 Hz → 483_120 kHz.
        let t =
            compute_type_ix_timing(2560, 1440, RefreshRate::integral(120), CvtAlgorithm::CvtRb2)
                .expect("CVT-RB v2 must produce a timing");
        assert_eq!(t.pixel_clock_khz, 483_120);
        assert_eq!(t.h_total, 2640);
        assert_eq!(t.v_total, 1525);
    }

    #[test]
    fn cvt_rb_v2_3840x2160_at_60() {
        // 4K@60 RB v2: h_total = 3920, v_total = 2222.
        // 60 × 3920 × 2222 = 522_614_400 Hz → 522_614 kHz (floor of 522614.4).
        let t = compute_type_ix_timing(3840, 2160, RefreshRate::integral(60), CvtAlgorithm::CvtRb2)
            .expect("CVT-RB v2 must produce a timing");
        assert_eq!(t.pixel_clock_khz, 522_614);
        assert_eq!(t.h_total, 3920);
        assert_eq!(t.v_total, 2222);
    }

    #[test]
    fn cvt_rb_v2_zero_width_returns_none() {
        assert!(
            compute_type_ix_timing(0, 1080, RefreshRate::integral(60), CvtAlgorithm::CvtRb2)
                .is_none()
        );
    }

    #[test]
    fn cvt_rb_v2_unreachable_refresh_returns_none() {
        // Frame period below the 460 µs RB v2 minimum.
        assert!(
            compute_type_ix_timing(
                1920,
                1080,
                RefreshRate::integral(3000),
                CvtAlgorithm::CvtRb2
            )
            .is_none()
        );
    }

    #[test]
    fn cvt_rb_v3_not_yet_implemented_returns_none() {
        assert!(
            compute_type_ix_timing(1920, 1080, RefreshRate::integral(60), CvtAlgorithm::CvtRb3)
                .is_none()
        );
    }

    #[test]
    fn reserved_algorithm_returns_none() {
        assert!(
            compute_type_ix_timing(
                1920,
                1080,
                RefreshRate::integral(60),
                CvtAlgorithm::Reserved(7)
            )
            .is_none()
        );
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
