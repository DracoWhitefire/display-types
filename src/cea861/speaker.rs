bitflags::bitflags! {
    /// Speaker channel presence flags, byte 1 of the Speaker Allocation Data Block.
    ///
    /// | Bit | Mask   | Channels                        |
    /// |-----|--------|---------------------------------|
    /// | 7   | `0x80` | FLW/FRW (Front Left/Right Wide) |
    /// | 6   | `0x40` | RLC/RRC (Rear Left/Right Center)|
    /// | 5   | `0x20` | FLC/FRC (Front Left/Right Ctr)  |
    /// | 4   | `0x10` | BC (Back Center)                |
    /// | 3   | `0x08` | BL/BR (Back Left/Right)         |
    /// | 2   | `0x04` | FC (Front Center)               |
    /// | 1   | `0x02` | LFE1 (Low-Frequency Effects 1)  |
    /// | 0   | `0x01` | FL/FR (Front Left/Right)        |
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SpeakerAllocationFlags: u8 {
        /// Front Left / Front Right channels.
        const FL_FR   = 0x01;
        /// Low-Frequency Effects channel 1.
        const LFE1    = 0x02;
        /// Front Center channel.
        const FC      = 0x04;
        /// Back Left / Back Right channels.
        const BL_BR   = 0x08;
        /// Back Center channel.
        const BC      = 0x10;
        /// Front Left Center / Front Right Center channels.
        const FLC_FRC = 0x20;
        /// Rear Left Center / Rear Right Center channels.
        const RLC_RRC = 0x40;
        /// Front Left Wide / Front Right Wide channels.
        const FLW_FRW = 0x80;
    }
}

bitflags::bitflags! {
    /// Speaker channel presence flags, byte 2 of the Speaker Allocation Data Block.
    ///
    /// | Bit | Mask   | Channels                           |
    /// |-----|--------|------------------------------------|
    /// | 7   | `0x80` | TpSiL/TpSiR (Top Side Left/Right)  |
    /// | 6   | `0x40` | SiL/SiR (Side Left/Right)          |
    /// | 5   | `0x20` | TpBC (Top Back Center)             |
    /// | 4   | `0x10` | LFE2 (Low-Frequency Effects 2)     |
    /// | 3   | `0x08` | LS/RS (Left/Right Surround)        |
    /// | 2   | `0x04` | TpFC (Top Front Center)            |
    /// | 1   | `0x02` | TpC (Top Center)                   |
    /// | 0   | `0x01` | TpFL/TpFR (Top Front Left/Right)   |
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SpeakerAllocationFlags2: u8 {
        /// Top Front Left / Top Front Right channels.
        const TP_FL_FR        = 0x01;
        /// Top Center channel.
        const TP_C            = 0x02;
        /// Top Front Center channel.
        const TP_FC           = 0x04;
        /// Left Surround / Right Surround channels.
        const LS_RS           = 0x08;
        /// Low-Frequency Effects channel 2.
        const LFE2            = 0x10;
        /// Top Back Center channel.
        const TP_BC           = 0x20;
        /// Side Left / Side Right channels.
        const SI_L_SI_R       = 0x40;
        /// Top Side Left / Top Side Right channels.
        const TP_SI_L_TP_SI_R = 0x80;
    }
}

bitflags::bitflags! {
    /// Speaker channel presence flags, byte 3 of the Speaker Allocation Data Block.
    ///
    /// | Bit | Mask   | Channels                              |
    /// |-----|--------|---------------------------------------|
    /// | 3   | `0x08` | TpLS/TpRS (Top Left/Right Surround)   |
    /// | 2   | `0x04` | BtFL/BtFR (Bottom Front Left/Right)   |
    /// | 1   | `0x02` | BtFC (Bottom Front Center)            |
    /// | 0   | `0x01` | TpBL/TpBR (Top Back Left/Right)       |
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SpeakerAllocationFlags3: u8 {
        /// Top Back Left / Top Back Right channels.
        const TP_BL_TP_BR = 0x01;
        /// Bottom Front Center channel.
        const BT_FC       = 0x02;
        /// Bottom Front Left / Bottom Front Right channels.
        const BT_FL_BT_FR = 0x04;
        /// Top Left Surround / Top Right Surround channels.
        const TP_LS_TP_RS = 0x08;
    }
}

/// Decoded Speaker Allocation Data Block (standard tag `0x04`).
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpeakerAllocation {
    /// Channels from byte 1 (core speaker channels).
    pub channels: SpeakerAllocationFlags,
    /// Channels from byte 2 (extended — top/surround/LFE2).
    pub channels_2: SpeakerAllocationFlags2,
    /// Channels from byte 3 (extended — top-back/bottom-front).
    pub channels_3: SpeakerAllocationFlags3,
}

impl SpeakerAllocation {
    /// Constructs a `SpeakerAllocation`.
    pub fn new(
        channels: SpeakerAllocationFlags,
        channels_2: SpeakerAllocationFlags2,
        channels_3: SpeakerAllocationFlags3,
    ) -> Self {
        Self {
            channels,
            channels_2,
            channels_3,
        }
    }
}

/// Decoded Room Configuration Data Block (extended tag `0x13`).
///
/// Describes the number of loudspeakers in the listening room and whether
/// individual speaker locations are provided in an accompanying
/// Speaker Location Data Block (extended tag `0x14`).
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoomConfigurationBlock {
    /// Number of loudspeakers in the room (bits 4:0).  `0` means not specified.
    pub speaker_count: u8,
    /// If `true`, individual speaker location entries are present in an
    /// accompanying Speaker Location Data Block (extended tag `0x14`).
    pub has_speaker_locations: bool,
}

impl RoomConfigurationBlock {
    /// Constructs a `RoomConfigurationBlock`.
    pub fn new(speaker_count: u8, has_speaker_locations: bool) -> Self {
        Self {
            speaker_count,
            has_speaker_locations,
        }
    }
}

/// A single speaker location entry from the Speaker Location Data Block
/// (extended tag `0x14`).
///
/// Each entry is two bytes: a channel assignment and a normalized distance
/// from the listener (0 = closest, 255 = furthest).
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpeakerLocationEntry {
    /// Channel assignment code (speaker role).
    ///
    /// Values follow the CTA-861-I Table 49 channel assignment codes
    /// (e.g. `0x00` = FL/FR, `0x01` = LFE1, `0x02` = FC, etc.).
    pub channel_assignment: u8,
    /// Normalized distance from the listener position.
    ///
    /// `0` = at or very close to the listener; `255` = furthest.
    /// The absolute distance is not encoded.
    pub distance: u8,
}

impl SpeakerLocationEntry {
    /// Constructs a `SpeakerLocationEntry`.
    pub fn new(channel_assignment: u8, distance: u8) -> Self {
        Self {
            channel_assignment,
            distance,
        }
    }
}
