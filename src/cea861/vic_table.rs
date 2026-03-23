use crate::{StereoMode, SyncDefinition, VideoMode};

/// Returns the [`VideoMode`] for the given CEA-861-E Video Identification Code (VIC),
/// or `None` if the VIC is not in the table (valid range: 1–64).
///
/// Timing parameters are sourced from CEA-861-E Table 1. Vertical timings for
/// interlaced modes are given per-field, matching the convention used in EDID
/// detailed timing descriptors.
pub fn vic_to_mode(vic: u8) -> Option<VideoMode> {
    // Columns: (width, height, refresh_hz, interlaced,
    //           h_front_porch, h_sync_width,
    //           v_front_porch, v_sync_width,
    //           v_sync_positive, h_sync_positive,
    //           pixel_clock_khz)
    let (w, h, r, interlaced, hfp, hsw, vfp, vsw, vpos, hpos, pixel_clock_khz): (
        u16,
        u16,
        u8,
        bool,
        u16,
        u16,
        u16,
        u16,
        bool,
        bool,
        u32,
    ) = match vic {
        // --- NTSC-derived 60 Hz family ---
        1 => (640, 480, 60, false, 16, 96, 10, 2, false, false, 25175),
        2 => (720, 480, 60, false, 16, 62, 9, 6, false, false, 27000),
        3 => (720, 480, 60, false, 16, 62, 9, 6, false, false, 27000),
        4 => (1280, 720, 60, false, 110, 40, 5, 5, true, true, 74250),
        5 => (1920, 1080, 60, true, 88, 44, 2, 5, true, true, 74250),
        6 => (1440, 480, 60, true, 38, 124, 4, 3, false, false, 27000),
        7 => (1440, 480, 60, true, 38, 124, 4, 3, false, false, 27000),
        8 => (1440, 240, 60, false, 38, 124, 4, 3, false, false, 27000),
        9 => (1440, 240, 60, false, 38, 124, 4, 3, false, false, 27000),
        10 => (2880, 480, 60, true, 76, 248, 4, 3, false, false, 54000),
        11 => (2880, 480, 60, true, 76, 248, 4, 3, false, false, 54000),
        12 => (2880, 240, 60, false, 76, 248, 4, 3, false, false, 54000),
        13 => (2880, 240, 60, false, 76, 248, 4, 3, false, false, 54000),
        14 => (1440, 480, 60, false, 32, 124, 9, 6, false, false, 54000),
        15 => (1440, 480, 60, false, 32, 124, 9, 6, false, false, 54000),
        16 => (1920, 1080, 60, false, 88, 44, 4, 5, true, true, 148500),
        // --- PAL-derived 50 Hz family ---
        17 => (720, 576, 50, false, 12, 64, 5, 5, false, false, 27000),
        18 => (720, 576, 50, false, 12, 64, 5, 5, false, false, 27000),
        19 => (1280, 720, 50, false, 440, 40, 5, 5, true, true, 74250),
        20 => (1920, 1080, 50, true, 528, 44, 2, 5, true, true, 74250),
        21 => (1440, 576, 50, true, 24, 126, 2, 3, false, false, 27000),
        22 => (1440, 576, 50, true, 24, 126, 2, 3, false, false, 27000),
        23 => (1440, 288, 50, false, 24, 126, 2, 3, false, false, 27000),
        24 => (1440, 288, 50, false, 24, 126, 2, 3, false, false, 27000),
        25 => (2880, 576, 50, true, 48, 252, 2, 3, false, false, 54000),
        26 => (2880, 576, 50, true, 48, 252, 2, 3, false, false, 54000),
        27 => (2880, 288, 50, false, 48, 252, 2, 3, false, false, 54000),
        28 => (2880, 288, 50, false, 48, 252, 2, 3, false, false, 54000),
        29 => (1440, 576, 50, false, 24, 128, 5, 5, false, false, 54000),
        30 => (1440, 576, 50, false, 24, 128, 5, 5, false, false, 54000),
        31 => (1920, 1080, 50, false, 528, 44, 4, 5, true, true, 148500),
        // --- 1080p low-rate variants ---
        32 => (1920, 1080, 24, false, 638, 44, 4, 5, true, true, 74250),
        33 => (1920, 1080, 25, false, 528, 44, 4, 5, true, true, 74250),
        34 => (1920, 1080, 30, false, 88, 44, 4, 5, true, true, 74250),
        // --- 2880-wide pixel-quadrupled formats ---
        35 => (2880, 480, 60, false, 64, 248, 9, 6, false, false, 108000),
        36 => (2880, 480, 60, false, 64, 248, 9, 6, false, false, 108000),
        37 => (2880, 576, 50, false, 48, 256, 5, 5, false, false, 108000),
        38 => (2880, 576, 50, false, 48, 256, 5, 5, false, false, 108000),
        // --- SMPTE 295M 1080i50 (1250-line system) ---
        39 => (1920, 1080, 50, true, 32, 168, 23, 5, false, true, 72000),
        // --- 100 Hz family ---
        40 => (1920, 1080, 100, true, 528, 44, 2, 5, true, true, 148500),
        41 => (1280, 720, 100, false, 440, 40, 5, 5, true, true, 148500),
        42 => (720, 576, 100, false, 12, 64, 5, 5, false, false, 54000),
        43 => (720, 576, 100, false, 12, 64, 5, 5, false, false, 54000),
        44 => (1440, 576, 100, true, 24, 126, 2, 3, false, false, 54000),
        45 => (1440, 576, 100, true, 24, 126, 2, 3, false, false, 54000),
        // --- 120 Hz family ---
        46 => (1920, 1080, 120, true, 88, 44, 2, 5, true, true, 148500),
        47 => (1280, 720, 120, false, 110, 40, 5, 5, true, true, 148500),
        48 => (720, 480, 120, false, 16, 62, 9, 6, false, false, 54000),
        49 => (720, 480, 120, false, 16, 62, 9, 6, false, false, 54000),
        50 => (1440, 480, 120, true, 38, 124, 4, 3, false, false, 54000),
        51 => (1440, 480, 120, true, 38, 124, 4, 3, false, false, 54000),
        // --- 200 Hz family ---
        52 => (720, 576, 200, false, 12, 64, 5, 5, false, false, 108000),
        53 => (720, 576, 200, false, 12, 64, 5, 5, false, false, 108000),
        54 => (1440, 576, 200, true, 24, 126, 2, 3, false, false, 108000),
        55 => (1440, 576, 200, true, 24, 126, 2, 3, false, false, 108000),
        // --- 240 Hz family ---
        56 => (720, 480, 240, false, 16, 62, 9, 6, false, false, 108000),
        57 => (720, 480, 240, false, 16, 62, 9, 6, false, false, 108000),
        58 => (1440, 480, 240, true, 38, 124, 4, 3, false, false, 108000),
        59 => (1440, 480, 240, true, 38, 124, 4, 3, false, false, 108000),
        // --- 720p low-rate variants ---
        // Htotal=3300 for 24/30 Hz; Htotal=3960 for 25 Hz (same blanking, different pixel clock)
        60 => (1280, 720, 24, false, 1760, 40, 5, 5, true, true, 59400),
        61 => (1280, 720, 25, false, 2420, 40, 5, 5, true, true, 74250),
        62 => (1280, 720, 30, false, 1760, 40, 5, 5, true, true, 74250),
        // --- 1080p high-rate ---
        63 => (1920, 1080, 120, false, 88, 44, 4, 5, true, true, 297000),
        64 => (1920, 1080, 100, false, 528, 44, 4, 5, true, true, 297000),

        // --- CTA-861-I: VICs 65–127, 193–219 ---

        // 720p aliases (same timings as earlier VICs)
        65 => (1280, 720, 24, false, 1760, 40, 5, 5, true, true, 59400), // = VIC 60
        66 => (1280, 720, 25, false, 2420, 40, 5, 5, true, true, 74250), // = VIC 61
        67 => (1280, 720, 30, false, 1760, 40, 5, 5, true, true, 74250), // = VIC 62
        68 => (1280, 720, 50, false, 440, 40, 5, 5, true, true, 74250),  // = VIC 19
        69 => (1280, 720, 60, false, 110, 40, 5, 5, true, true, 74250),  // = VIC 4
        70 => (1280, 720, 100, false, 440, 40, 5, 5, true, true, 148500), // = VIC 41
        71 => (1280, 720, 120, false, 110, 40, 5, 5, true, true, 148500), // = VIC 47

        // 1080p aliases
        72 => (1920, 1080, 24, false, 638, 44, 4, 5, true, true, 74250), // = VIC 32
        73 => (1920, 1080, 25, false, 528, 44, 4, 5, true, true, 74250), // = VIC 33
        74 => (1920, 1080, 30, false, 88, 44, 4, 5, true, true, 74250),  // = VIC 34
        75 => (1920, 1080, 50, false, 528, 44, 4, 5, true, true, 148500), // = VIC 31
        76 => (1920, 1080, 60, false, 88, 44, 4, 5, true, true, 148500), // = VIC 16
        77 => (1920, 1080, 100, false, 528, 44, 4, 5, true, true, 297000), // = VIC 64
        78 => (1920, 1080, 120, false, 88, 44, 4, 5, true, true, 297000), // = VIC 63

        // 1680×720p
        79 => (1680, 720, 24, false, 1360, 40, 5, 5, true, true, 59400),
        80 => (1680, 720, 25, false, 1228, 40, 5, 5, true, true, 59400),
        81 => (1680, 720, 30, false, 700, 40, 5, 5, true, true, 59400),
        82 => (1680, 720, 50, false, 260, 40, 5, 5, true, true, 82500),
        83 => (1680, 720, 60, false, 260, 40, 5, 5, true, true, 99000),
        84 => (1680, 720, 100, false, 60, 40, 5, 5, true, true, 165000),
        85 => (1680, 720, 120, false, 60, 40, 5, 5, true, true, 198000),

        // 2560×1080p
        86 => (2560, 1080, 24, false, 998, 44, 4, 5, true, true, 99000),
        87 => (2560, 1080, 25, false, 448, 44, 4, 5, true, true, 90000),
        88 => (2560, 1080, 30, false, 768, 44, 4, 5, true, true, 118800),
        89 => (2560, 1080, 50, false, 548, 44, 4, 5, true, true, 185625),
        90 => (2560, 1080, 60, false, 248, 44, 4, 5, true, true, 198000),
        91 => (2560, 1080, 100, false, 218, 44, 4, 5, true, true, 371250),
        92 => (2560, 1080, 120, false, 548, 44, 4, 5, true, true, 495000),

        // 3840×2160p (4K UHD)
        93 | 103 => (3840, 2160, 24, false, 1276, 88, 8, 10, true, true, 297000),
        94 | 104 => (3840, 2160, 25, false, 1056, 88, 8, 10, true, true, 297000),
        95 | 105 => (3840, 2160, 30, false, 176, 88, 8, 10, true, true, 297000),
        96 | 106 => (3840, 2160, 50, false, 1056, 88, 8, 10, true, true, 594000),
        97 | 107 => (3840, 2160, 60, false, 176, 88, 8, 10, true, true, 594000),

        // 4096×2160p (DCI 4K)
        98 => (4096, 2160, 24, false, 1020, 88, 8, 10, true, true, 297000),
        99 => (4096, 2160, 25, false, 968, 88, 8, 10, true, true, 297000),
        100 => (4096, 2160, 30, false, 88, 88, 8, 10, true, true, 297000),
        101 => (4096, 2160, 50, false, 968, 88, 8, 10, true, true, 594000),
        102 => (4096, 2160, 60, false, 88, 88, 8, 10, true, true, 594000),

        // 48 Hz additions
        108 | 109 => (1280, 720, 48, false, 960, 40, 5, 5, true, true, 90000),
        110 => (1680, 720, 48, false, 810, 40, 5, 5, true, true, 99000),
        111 | 112 => (1920, 1080, 48, false, 638, 44, 4, 5, true, true, 148500),
        113 => (2560, 1080, 48, false, 998, 44, 4, 5, true, true, 198000),
        114 | 116 => (3840, 2160, 48, false, 1276, 88, 8, 10, true, true, 594000),
        115 => (4096, 2160, 48, false, 1020, 88, 8, 10, true, true, 594000),

        // 3840×2160p 100/120 Hz
        117 | 119 => (3840, 2160, 100, false, 1056, 88, 8, 10, true, true, 1188000),
        118 | 120 => (3840, 2160, 120, false, 176, 88, 8, 10, true, true, 1188000),

        // 5120×2160p
        121 => (5120, 2160, 24, false, 1996, 88, 8, 10, true, true, 396000),
        122 => (5120, 2160, 25, false, 1696, 88, 8, 10, true, true, 396000),
        123 => (5120, 2160, 30, false, 664, 88, 8, 10, true, true, 396000),
        124 => (5120, 2160, 48, false, 746, 88, 8, 10, true, true, 742500),
        125 => (5120, 2160, 50, false, 1096, 88, 8, 10, true, true, 742500),
        126 => (5120, 2160, 60, false, 164, 88, 8, 10, true, true, 742500),
        127 => (5120, 2160, 100, false, 1096, 88, 8, 10, true, true, 1485000),
        193 => (5120, 2160, 120, false, 164, 88, 8, 10, true, true, 1485000),

        // 7680×4320p (8K UHD)
        194 | 202 => (
            7680, 4320, 24, false, 2552, 176, 16, 20, true, true, 1188000,
        ),
        195 | 203 => (
            7680, 4320, 25, false, 2352, 176, 16, 20, true, true, 1188000,
        ),
        196 | 204 => (7680, 4320, 30, false, 552, 176, 16, 20, true, true, 1188000),
        197 | 205 => (
            7680, 4320, 48, false, 2552, 176, 16, 20, true, true, 2376000,
        ),
        198 | 206 => (
            7680, 4320, 50, false, 2352, 176, 16, 20, true, true, 2376000,
        ),
        199 | 207 => (7680, 4320, 60, false, 552, 176, 16, 20, true, true, 2376000),
        200 | 208 => (
            7680, 4320, 100, false, 2112, 176, 16, 20, true, true, 4752000,
        ),
        201 | 209 => (
            7680, 4320, 120, false, 352, 176, 16, 20, true, true, 4752000,
        ),

        // 10240×4320p
        210 => (
            10240, 4320, 24, false, 1492, 176, 16, 20, true, true, 1485000,
        ),
        211 => (
            10240, 4320, 25, false, 2492, 176, 16, 20, true, true, 1485000,
        ),
        212 => (
            10240, 4320, 30, false, 288, 176, 16, 20, true, true, 1485000,
        ),
        213 => (
            10240, 4320, 48, false, 1492, 176, 16, 20, true, true, 2970000,
        ),
        214 => (
            10240, 4320, 50, false, 2492, 176, 16, 20, true, true, 2970000,
        ),
        215 => (
            10240, 4320, 60, false, 288, 176, 16, 20, true, true, 2970000,
        ),
        216 => (
            10240, 4320, 100, false, 2192, 176, 16, 20, true, true, 5940000,
        ),
        217 => (
            10240, 4320, 120, false, 288, 176, 16, 20, true, true, 5940000,
        ),

        // 4096×2160p 100/120 Hz
        218 => (4096, 2160, 100, false, 800, 88, 8, 10, true, true, 1188000),
        219 => (4096, 2160, 120, false, 88, 88, 8, 10, true, true, 1188000),

        _ => return None,
    };

    Some(VideoMode::new(w, h, r, interlaced).with_detailed_timing(
        pixel_clock_khz,
        hfp,
        hsw,
        vfp,
        vsw,
        0,
        0,
        StereoMode::None,
        Some(SyncDefinition::DigitalSeparate {
            v_sync_positive: vpos,
            h_sync_positive: hpos,
        }),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vic1_640x480p60() {
        let mode = vic_to_mode(1).unwrap();
        assert_eq!(mode.width, 640);
        assert_eq!(mode.height, 480);
        assert_eq!(mode.refresh_rate, 60);
        assert!(!mode.interlaced);
        assert_eq!(mode.h_front_porch, 16);
        assert_eq!(mode.h_sync_width, 96);
        assert_eq!(mode.v_front_porch, 10);
        assert_eq!(mode.v_sync_width, 2);
        assert_eq!(mode.pixel_clock_khz, Some(25175));
    }

    #[test]
    fn test_vic16_1080p60() {
        let mode = vic_to_mode(16).unwrap();
        assert_eq!(mode.width, 1920);
        assert_eq!(mode.height, 1080);
        assert_eq!(mode.refresh_rate, 60);
        assert!(!mode.interlaced);
        assert!(matches!(
            mode.sync,
            Some(SyncDefinition::DigitalSeparate {
                v_sync_positive: true,
                h_sync_positive: true
            })
        ));
        assert_eq!(mode.pixel_clock_khz, Some(148500));
    }

    #[test]
    fn test_vic5_1080i60_interlaced() {
        let mode = vic_to_mode(5).unwrap();
        assert_eq!(mode.width, 1920);
        assert_eq!(mode.height, 1080);
        assert_eq!(mode.refresh_rate, 60);
        assert!(mode.interlaced);
        assert_eq!(mode.pixel_clock_khz, Some(74250));
    }

    #[test]
    fn test_vic63_1080p120() {
        let mode = vic_to_mode(63).unwrap();
        assert_eq!(mode.width, 1920);
        assert_eq!(mode.height, 1080);
        assert_eq!(mode.refresh_rate, 120);
        assert!(!mode.interlaced);
        assert_eq!(mode.pixel_clock_khz, Some(297000));
    }

    #[test]
    fn test_vic0_is_none() {
        assert!(vic_to_mode(0).is_none());
    }

    #[test]
    fn test_vic220_is_none() {
        assert!(vic_to_mode(220).is_none());
    }

    #[test]
    fn test_all_vics_1_to_64_are_some() {
        for vic in 1..=64 {
            assert!(vic_to_mode(vic).is_some(), "VIC {vic} returned None");
        }
    }

    #[test]
    fn test_extended_vics_are_some() {
        // VICs 65–127 (continuous range in CTA-861-I)
        for vic in 65..=127 {
            assert!(vic_to_mode(vic).is_some(), "VIC {vic} returned None");
        }
        // VIC 193 (5120×2160p120)
        assert!(vic_to_mode(193).is_some());
        // VICs 194–219
        for vic in 194..=219 {
            assert!(vic_to_mode(vic).is_some(), "VIC {vic} returned None");
        }
    }

    #[test]
    fn test_vic93_4k_uhd_24() {
        let mode = vic_to_mode(93).unwrap();
        assert_eq!(mode.width, 3840);
        assert_eq!(mode.height, 2160);
        assert_eq!(mode.refresh_rate, 24);
        assert!(!mode.interlaced);
        assert_eq!(mode.pixel_clock_khz, Some(297000));
    }

    #[test]
    fn test_vic97_4k_uhd_60() {
        let mode = vic_to_mode(97).unwrap();
        assert_eq!(mode.width, 3840);
        assert_eq!(mode.height, 2160);
        assert_eq!(mode.refresh_rate, 60);
        assert!(!mode.interlaced);
        assert_eq!(mode.pixel_clock_khz, Some(594000));
    }

    #[test]
    fn test_vic103_aliases_93() {
        assert_eq!(vic_to_mode(93), vic_to_mode(103));
    }

    #[test]
    fn test_vic217_10k_120() {
        let mode = vic_to_mode(217).unwrap();
        assert_eq!(mode.width, 10240);
        assert_eq!(mode.height, 4320);
        assert_eq!(mode.refresh_rate, 120);
        assert_eq!(mode.pixel_clock_khz, Some(5940000));
    }
}
