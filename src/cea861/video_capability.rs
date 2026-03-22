bitflags::bitflags! {
    /// Flags from the Video Capability Data Block (extended tag `0x00`).
    ///
    /// Describes over-/underscan behaviour and quantization range support.
    ///
    /// | Bit | Mask   | Meaning                                          |
    /// |-----|--------|--------------------------------------------------|
    /// | 7   | `0x80` | QY: quantization range selectable (YCC)          |
    /// | 6   | `0x40` | QS: quantization range selectable (RGB)          |
    /// | 5–4 | `0x30` | PT: preferred PT behavior (2-bit field)          |
    /// | 3–2 | `0x0C` | IT: IT content behavior (2-bit field)            |
    /// | 1–0 | `0x03` | CE: CE content behavior (2-bit field)            |
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VideoCapabilityFlags: u8 {
        /// YCC quantization range is selectable (QY).
        const QY = 0x80;
        /// RGB quantization range is selectable (QS).
        const QS = 0x40;
    }
}

/// Decoded Video Capability Data Block (extended tag `0x00`).
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VideoCapability {
    /// Quantization range and overscan flags.
    pub flags: VideoCapabilityFlags,
    /// Preferred timing overscan/underscan behaviour (bits 5–4, 2-bit field).
    pub pt_behaviour: u8,
    /// IT content overscan/underscan behaviour (bits 3–2, 2-bit field).
    pub it_behaviour: u8,
    /// CE content overscan/underscan behaviour (bits 1–0, 2-bit field).
    pub ce_behaviour: u8,
}

impl VideoCapability {
    /// Constructs a `VideoCapability`.
    pub fn new(
        flags: VideoCapabilityFlags,
        pt_behaviour: u8,
        it_behaviour: u8,
        ce_behaviour: u8,
    ) -> Self {
        Self {
            flags,
            pt_behaviour,
            it_behaviour,
            ce_behaviour,
        }
    }
}
