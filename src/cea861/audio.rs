bitflags::bitflags! {
    /// Sample-rate support mask from byte 2 of a Short Audio Descriptor.
    ///
    /// Each set bit indicates the display/sink supports that sample rate.
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct AudioSampleRates: u8 {
        /// 32 kHz sample rate.
        const HZ_32000  = 0x01;
        /// 44.1 kHz sample rate.
        const HZ_44100  = 0x02;
        /// 48 kHz sample rate.
        const HZ_48000  = 0x04;
        /// 88.2 kHz sample rate.
        const HZ_88200  = 0x08;
        /// 96 kHz sample rate.
        const HZ_96000  = 0x10;
        /// 176.4 kHz sample rate.
        const HZ_176400 = 0x20;
        /// 192 kHz sample rate.
        const HZ_192000 = 0x40;
    }
}

/// Audio format code from bits 6–3 of the first SAD byte.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    /// Linear PCM (uncompressed).
    Lpcm,
    /// Dolby AC-3.
    Ac3,
    /// MPEG-1 (Layers 1 & 2).
    Mpeg1,
    /// MPEG-1 Layer 3 (MP3).
    Mp3,
    /// MPEG-2 multi-channel.
    Mpeg2Multichannel,
    /// AAC LC.
    AacLc,
    /// DTS.
    Dts,
    /// ATRAC.
    Atrac,
    /// One Bit Audio (SACD).
    OneBitAudio,
    /// Enhanced AC-3 (Dolby Digital Plus / E-AC-3).
    EnhancedAc3,
    /// DTS-HD.
    DtsHd,
    /// MLP / Dolby TrueHD.
    MlpTrueHd,
    /// DST.
    Dst,
    /// WMA Pro.
    WmaPro,
    /// Extended audio format (AFC = 15); the inner value is bits 7–3 of byte 3.
    Extended(u8),
    /// Reserved or unknown format code.
    Reserved(u8),
}

/// Format-specific information from byte 3 of a Short Audio Descriptor.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormatInfo {
    /// LPCM (AFC = 1): supported bit depths.
    Lpcm {
        /// 16-bit depth is supported.
        depth_16: bool,
        /// 20-bit depth is supported.
        depth_20: bool,
        /// 24-bit depth is supported.
        depth_24: bool,
    },
    /// Compressed formats (AFC 2–8): maximum bitrate in kbps (raw byte × 8).
    MaxBitrateKbps(u16),
    /// All other formats: raw byte 3 value.
    Raw(u8),
}

/// A decoded Short Audio Descriptor (3 bytes) from a CEA Audio Data Block (tag `0x01`).
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortAudioDescriptor {
    /// Audio coding format.
    pub format: AudioFormat,
    /// Maximum number of audio channels (1–8).
    pub max_channels: u8,
    /// Supported sample rates.
    pub sample_rates: AudioSampleRates,
    /// Format-specific byte 3 interpretation.
    pub format_info: AudioFormatInfo,
}

impl ShortAudioDescriptor {
    /// Constructs a `ShortAudioDescriptor`.
    pub fn new(
        format: AudioFormat,
        max_channels: u8,
        sample_rates: AudioSampleRates,
        format_info: AudioFormatInfo,
    ) -> Self {
        Self {
            format,
            max_channels,
            sample_rates,
            format_info,
        }
    }
}
