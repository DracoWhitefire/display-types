bitflags::bitflags! {
    /// Capability flags from byte 5 of the HDMI VSDB payload (byte 8 of the CEA block,
    /// after the 3-byte header and 3-byte OUI).
    ///
    /// | Bit | Mask   | Meaning                                  |
    /// |-----|--------|------------------------------------------|
    /// | 7   | `0x80` | Supports ACP / ISRC packets (`SUPPORTS_AI`) |
    /// | 6   | `0x40` | 48-bit deep color                        |
    /// | 5   | `0x20` | 36-bit deep color                        |
    /// | 4   | `0x10` | 30-bit deep color                        |
    /// | 3   | `0x08` | YCbCr 4:4:4 in deep color modes          |
    /// | 0   | `0x01` | DVI dual-link                            |
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct HdmiVsdbFlags: u8 {
        /// Sink supports ACP, ISRC1, and ISRC2 packets.
        const SUPPORTS_AI = 0x80;
        /// Sink supports 48-bit (16 bpc) deep color.
        const DC_48BIT    = 0x40;
        /// Sink supports 36-bit (12 bpc) deep color.
        const DC_36BIT    = 0x20;
        /// Sink supports 30-bit (10 bpc) deep color.
        const DC_30BIT    = 0x10;
        /// Sink supports YCbCr 4:4:4 in deep color modes.
        const DC_Y444     = 0x08;
        /// Source supports DVI dual-link.
        const DVI_DUAL    = 0x01;
    }
}

/// Decoded HDMI 1.x Vendor-Specific Data Block (OUI `0x000C03`).
///
/// Stored in the `Cea861Capabilities::hdmi_vsdb` field when the CEA extension block
/// contains an HDMI VSDB.
///
/// Field presence depends on the block length:
/// - `source_physical_address` is always present (minimum valid VSDB is 5 bytes after OUI).
/// - `flags` and `max_tmds_clock_mhz` require at least 2 and 3 payload bytes respectively.
/// - Latency fields require the corresponding presence bits in the misc byte.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HdmiVsdb {
    /// Source physical address encoded as four 4-bit nibbles (A.B.C.D) in a `u16`.
    ///
    /// Nibble layout: `AABBCCDD` where AA = bits 15–12, BB = 11–8, CC = 7–4, DD = 3–0.
    pub source_physical_address: u16,
    /// Deep color and audio capability flags (byte 5 of the VSDB payload).
    ///
    /// `HdmiVsdbFlags::empty()` when the byte is absent (short VSDB).
    pub flags: HdmiVsdbFlags,
    /// Maximum TMDS clock supported by the sink in MHz.
    ///
    /// Decoded as raw byte × 5. `None` when the byte is absent.
    pub max_tmds_clock_mhz: Option<u16>,
    /// Progressive video latency in milliseconds, or `None` if absent or unknown.
    pub video_latency_ms: Option<u16>,
    /// Progressive audio latency in milliseconds, or `None` if absent or unknown.
    pub audio_latency_ms: Option<u16>,
    /// Interlaced video latency in milliseconds, or `None` if absent or unknown.
    pub interlaced_video_latency_ms: Option<u16>,
    /// Interlaced audio latency in milliseconds, or `None` if absent or unknown.
    pub interlaced_audio_latency_ms: Option<u16>,
}

impl HdmiVsdb {
    /// Constructs an `HdmiVsdb`.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        source_physical_address: u16,
        flags: HdmiVsdbFlags,
        max_tmds_clock_mhz: Option<u16>,
        video_latency_ms: Option<u16>,
        audio_latency_ms: Option<u16>,
        interlaced_video_latency_ms: Option<u16>,
        interlaced_audio_latency_ms: Option<u16>,
    ) -> Self {
        Self {
            source_physical_address,
            flags,
            max_tmds_clock_mhz,
            video_latency_ms,
            audio_latency_ms,
            interlaced_video_latency_ms,
            interlaced_audio_latency_ms,
        }
    }
}
