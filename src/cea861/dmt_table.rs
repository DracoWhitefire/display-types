use crate::{ModeSource, RefreshRate, StereoMode, SyncDefinition, VideoMode};

/// Look up a VESA DMT ID and return the corresponding `VideoMode`, or `None`
/// if the ID is not in the standard table.
///
/// Source: VESA DMT v1.13, Table 2-1 (cross-referenced against Linux kernel
/// `drivers/gpu/drm/drm_edid.c`).
///
/// When two DMT IDs share the same resolution and refresh rate (one Reduced
/// Blanking variant, one standard), both map to distinct `VideoMode` values with
/// different timing parameters. The interlaced 1024×768@43 Hz entry (0x0F) is
/// included; 0x58 (4096×2160 @ 59.94 Hz) is stored as 60000/1001 Hz.
pub fn dmt_to_mode(id: u16) -> Option<VideoMode> {
    // Columns: width, height, refresh_rate, interlaced,
    //          h_front_porch, h_sync_width, v_front_porch, v_sync_width,
    //          h_sync_positive, v_sync_positive, pixel_clock_khz
    macro_rules! e {
        ($w:expr, $h:expr, $rr:expr, $i:expr,
         $hfp:expr, $hsw:expr, $vfp:expr, $vsw:expr,
         $hp:expr, $vp:expr, $pc:expr) => {
            VideoMode::new($w, $h, $rr as u32, $i).with_detailed_timing(
                $pc,
                $hfp,
                $hsw,
                $vfp,
                $vsw,
                0,
                0,
                StereoMode::None,
                Some(SyncDefinition::DigitalSeparate {
                    h_sync_positive: $hp,
                    v_sync_positive: $vp,
                }),
            )
        };
    }

    let mode = match id {
        // 640×350
        0x01 => e!(640, 350, 85, false, 32, 64, 32, 3, true, false, 31500),
        // 640×400
        0x02 => e!(640, 400, 85, false, 32, 64, 1, 3, false, true, 31500),
        // 720×400
        0x03 => e!(720, 400, 85, false, 36, 72, 1, 3, false, true, 35500),
        // 640×480
        0x04 => e!(640, 480, 60, false, 16, 96, 10, 2, false, false, 25175),
        0x05 => e!(640, 480, 72, false, 24, 40, 9, 3, false, false, 31500),
        0x06 => e!(640, 480, 75, false, 16, 64, 1, 3, false, false, 31500),
        0x07 => e!(640, 480, 85, false, 56, 56, 1, 3, false, false, 36000),
        // 800×600
        0x08 => e!(800, 600, 56, false, 24, 72, 1, 2, true, true, 36000),
        0x09 => e!(800, 600, 60, false, 40, 128, 1, 4, true, true, 40000),
        0x0A => e!(800, 600, 72, false, 56, 120, 37, 6, true, true, 50000),
        0x0B => e!(800, 600, 75, false, 16, 80, 1, 3, true, true, 49500),
        0x0C => e!(800, 600, 85, false, 32, 64, 1, 3, true, true, 56250),
        0x0D => e!(800, 600, 120, false, 48, 32, 3, 4, true, false, 73250), // RB
        // 848×480
        0x0E => e!(848, 480, 60, false, 16, 112, 6, 8, true, true, 33750),
        // 1024×768i (interlaced)
        0x0F => e!(1024, 768, 43, true, 8, 176, 0, 4, false, false, 44900),
        // 1024×768
        0x10 => e!(1024, 768, 60, false, 24, 136, 3, 6, false, false, 65000),
        0x11 => e!(1024, 768, 70, false, 24, 136, 3, 6, false, false, 75000),
        0x12 => e!(1024, 768, 75, false, 16, 96, 1, 3, true, true, 78750),
        0x13 => e!(1024, 768, 85, false, 48, 96, 1, 3, true, true, 94500),
        0x14 => e!(1024, 768, 120, false, 48, 32, 3, 4, true, false, 115500), // RB
        // 1152×864
        0x15 => e!(1152, 864, 75, false, 64, 128, 1, 3, true, true, 108000),
        // 1280×768
        0x16 => e!(1280, 768, 60, false, 48, 32, 3, 7, true, false, 68250), // RB
        0x17 => e!(1280, 768, 60, false, 64, 128, 3, 7, false, true, 79500),
        0x18 => e!(1280, 768, 75, false, 80, 128, 3, 7, false, true, 102250),
        0x19 => e!(1280, 768, 85, false, 80, 136, 3, 7, false, true, 117500),
        0x1A => e!(1280, 768, 120, false, 48, 32, 3, 7, true, false, 140250), // RB
        // 1280×800
        0x1B => e!(1280, 800, 60, false, 48, 32, 3, 6, true, false, 71000), // RB
        0x1C => e!(1280, 800, 60, false, 72, 128, 3, 6, false, true, 83500),
        0x1D => e!(1280, 800, 75, false, 80, 128, 3, 6, false, true, 106500),
        0x1E => e!(1280, 800, 85, false, 80, 136, 3, 6, false, true, 122500),
        0x1F => e!(1280, 800, 120, false, 48, 32, 3, 6, true, false, 146250), // RB
        // 1280×960
        0x20 => e!(1280, 960, 60, false, 96, 112, 1, 3, true, true, 108000),
        0x21 => e!(1280, 960, 85, false, 64, 160, 1, 3, true, true, 148500),
        0x22 => e!(1280, 960, 120, false, 48, 32, 3, 4, true, false, 175500), // RB
        // 1280×1024
        0x23 => e!(1280, 1024, 60, false, 48, 112, 1, 3, true, true, 108000),
        0x24 => e!(1280, 1024, 75, false, 16, 144, 1, 3, true, true, 135000),
        0x25 => e!(1280, 1024, 85, false, 64, 160, 1, 3, true, true, 157500),
        0x26 => e!(1280, 1024, 120, false, 48, 32, 3, 7, true, false, 187250), // RB
        // 1360×768
        0x27 => e!(1360, 768, 60, false, 64, 112, 3, 6, true, true, 85500),
        0x28 => e!(1360, 768, 120, false, 48, 32, 3, 5, true, false, 148250), // RB
        // 1400×1050
        0x29 => e!(1400, 1050, 60, false, 48, 32, 3, 4, true, false, 101000), // RB
        0x2A => e!(1400, 1050, 60, false, 88, 144, 3, 4, false, true, 121750),
        0x2B => e!(1400, 1050, 75, false, 104, 144, 3, 4, false, true, 156000),
        0x2C => e!(1400, 1050, 85, false, 104, 152, 3, 4, false, true, 179500),
        0x2D => e!(1400, 1050, 120, false, 48, 32, 3, 4, true, false, 208000), // RB
        // 1440×900
        0x2E => e!(1440, 900, 60, false, 48, 32, 3, 6, true, false, 88750), // RB
        0x2F => e!(1440, 900, 60, false, 80, 152, 3, 6, false, true, 106500),
        0x30 => e!(1440, 900, 75, false, 96, 152, 3, 6, false, true, 136750),
        0x31 => e!(1440, 900, 85, false, 104, 152, 3, 6, false, true, 157000),
        0x32 => e!(1440, 900, 120, false, 48, 32, 3, 6, true, false, 182750), // RB
        // 1600×1200
        0x33 => e!(1600, 1200, 60, false, 64, 192, 1, 3, true, true, 162000),
        0x34 => e!(1600, 1200, 65, false, 64, 192, 1, 3, true, true, 175500),
        0x35 => e!(1600, 1200, 70, false, 64, 192, 1, 3, true, true, 189000),
        0x36 => e!(1600, 1200, 75, false, 64, 192, 1, 3, true, true, 202500),
        0x37 => e!(1600, 1200, 85, false, 64, 192, 1, 3, true, true, 229500),
        0x38 => e!(1600, 1200, 120, false, 48, 32, 3, 4, true, false, 268250), // RB
        // 1680×1050
        0x39 => e!(1680, 1050, 60, false, 48, 32, 3, 6, true, false, 119000), // RB
        0x3A => e!(1680, 1050, 60, false, 104, 176, 3, 6, false, true, 146250),
        0x3B => e!(1680, 1050, 75, false, 120, 176, 3, 6, false, true, 187000),
        0x3C => e!(1680, 1050, 85, false, 128, 176, 3, 6, false, true, 214750),
        0x3D => e!(1680, 1050, 120, false, 48, 32, 3, 6, true, false, 245500), // RB
        // 1792×1344
        0x3E => e!(1792, 1344, 60, false, 128, 200, 1, 3, false, true, 204750),
        0x3F => e!(1792, 1344, 75, false, 96, 216, 1, 3, false, true, 261000),
        0x40 => e!(1792, 1344, 120, false, 48, 32, 3, 4, true, false, 333250), // RB
        // 1856×1392
        0x41 => e!(1856, 1392, 60, false, 96, 224, 1, 3, false, true, 218250),
        0x42 => e!(1856, 1392, 75, false, 128, 224, 1, 3, false, true, 288000),
        0x43 => e!(1856, 1392, 120, false, 48, 32, 3, 4, true, false, 356500), // RB
        // 1920×1200
        0x44 => e!(1920, 1200, 60, false, 48, 32, 3, 6, true, false, 154000), // RB
        0x45 => e!(1920, 1200, 60, false, 136, 200, 3, 6, false, true, 193250),
        0x46 => e!(1920, 1200, 75, false, 136, 208, 3, 6, false, true, 245250),
        0x47 => e!(1920, 1200, 85, false, 144, 208, 3, 6, false, true, 281250),
        0x48 => e!(1920, 1200, 120, false, 48, 32, 3, 6, true, false, 317000), // RB
        // 1920×1440
        0x49 => e!(1920, 1440, 60, false, 128, 208, 1, 3, false, true, 234000),
        0x4A => e!(1920, 1440, 75, false, 144, 224, 1, 3, false, true, 297000),
        0x4B => e!(1920, 1440, 120, false, 48, 32, 3, 4, true, false, 380500), // RB
        // 2560×1600
        0x4C => e!(2560, 1600, 60, false, 48, 32, 3, 6, true, false, 268500), // RB
        0x4D => e!(2560, 1600, 60, false, 192, 280, 3, 6, false, true, 348500),
        0x4E => e!(2560, 1600, 75, false, 208, 280, 3, 6, false, true, 443250),
        0x4F => e!(2560, 1600, 85, false, 208, 280, 3, 6, false, true, 505250),
        0x50 => e!(2560, 1600, 120, false, 48, 32, 3, 6, true, false, 552750), // RB
        // Additional VESA DMT 1.13 entries (codes 0x51–0x58)
        0x51 => e!(1366, 768, 60, false, 70, 143, 3, 3, true, true, 85500),
        0x52 => e!(1920, 1080, 60, false, 88, 44, 4, 5, true, true, 148500),
        0x53 => e!(1600, 900, 60, false, 48, 32, 3, 5, true, false, 108000), // RB
        0x54 => e!(2048, 1152, 60, false, 48, 32, 3, 4, true, false, 162000), // RB
        0x55 => e!(1280, 720, 60, false, 110, 40, 5, 5, true, true, 74250),
        0x56 => e!(1366, 768, 60, false, 14, 56, 1, 3, true, true, 72000),
        0x57 => e!(4096, 2160, 60, false, 8, 32, 48, 8, true, true, 556744),
        0x58 => VideoMode::new(4096, 2160, RefreshRate::fractional(60000, 1001), false)
            .with_detailed_timing(
                556188,
                8,
                32,
                48,
                8,
                0,
                0,
                StereoMode::None,
                Some(SyncDefinition::DigitalSeparate {
                    h_sync_positive: true,
                    v_sync_positive: true,
                }),
            ),
        _ => return None,
    };
    Some(mode.with_source(ModeSource::DmtId(id)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dmt_id_is_preserved_in_source() {
        let mode = dmt_to_mode(0x52).unwrap(); // 1920×1080 @ 60 Hz
        assert_eq!(mode.width, 1920);
        assert_eq!(mode.height, 1080);
        assert_eq!(mode.source, Some(ModeSource::DmtId(0x52)));
    }

    #[test]
    fn unknown_dmt_id_returns_none() {
        assert!(dmt_to_mode(0x00).is_none());
        assert!(dmt_to_mode(0x59).is_none());
    }
}
