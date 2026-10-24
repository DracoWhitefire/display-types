/// Maximum Fixed Rate Link (FRL) bandwidth supported by a HDMI 2.1 sink.
///
/// Variants are assigned the `Max_FRL_Rate` nibble values from the HDMI 2.1a spec directly.
/// The discriminants establish the ordering: higher value = greater bandwidth.
#[non_exhaustive]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HdmiForumFrl {
    /// FRL not supported; TMDS only.
    NotSupported = 0,
    /// Up to 3 Gbps/lane on 3 lanes (≈ 9 Gbps total).
    Rate3Gbps3Lanes = 1,
    /// Up to 6 Gbps/lane on 3 lanes (≈ 18 Gbps).
    Rate6Gbps3Lanes = 2,
    /// Up to 6 Gbps/lane on 4 lanes (≈ 24 Gbps).
    Rate6Gbps4Lanes = 3,
    /// Up to 8 Gbps/lane on 4 lanes (≈ 32 Gbps).
    Rate8Gbps4Lanes = 4,
    /// Up to 10 Gbps/lane on 4 lanes (≈ 40 Gbps).
    Rate10Gbps4Lanes = 5,
    /// Up to 12 Gbps/lane on 4 lanes (≈ 48 Gbps).
    Rate12Gbps4Lanes = 6,
}

impl HdmiForumFrl {
    /// Decodes the `Max_FRL_Rate` nibble from the HDMI 2.1 SCDS.
    pub fn from_raw(raw: u8) -> Self {
        match raw {
            0 => Self::NotSupported,
            1 => Self::Rate3Gbps3Lanes,
            2 => Self::Rate6Gbps3Lanes,
            3 => Self::Rate6Gbps4Lanes,
            4 => Self::Rate8Gbps4Lanes,
            5 => Self::Rate10Gbps4Lanes,
            6 => Self::Rate12Gbps4Lanes,
            _ => Self::NotSupported, // reserved values
        }
    }

    /// The data lanes a link at this rate carries: three for TMDS
    /// ([`NotSupported`](Self::NotSupported): the three TMDS data channels) and the
    /// 3-lane FRL rates, four for the 4-lane FRL rates.
    ///
    /// Defined for every rate, so a consumer needs no rule of its own, TMDS included: a
    /// future rate gets its lane count here, where it is added. Whether the link is FRL at
    /// all is `self != HdmiForumFrl::NotSupported`.
    pub const fn lanes(self) -> HdmiLanes {
        match self {
            Self::NotSupported | Self::Rate3Gbps3Lanes | Self::Rate6Gbps3Lanes => HdmiLanes::Three,
            Self::Rate6Gbps4Lanes
            | Self::Rate8Gbps4Lanes
            | Self::Rate10Gbps4Lanes
            | Self::Rate12Gbps4Lanes => HdmiLanes::Four,
        }
    }
}

/// The number of data lanes an HDMI link carries: three for TMDS (its three data
/// channels) and 3-lane FRL, four for 4-lane FRL, where the TMDS clock channel becomes
/// lane 3. See [`HdmiForumFrl::lanes`].
///
/// Exhaustive on purpose: every HDMI version that defines FRL uses three or four lanes, so
/// a match on this type needs no wildcard; another count would be a deliberate breaking
/// change. Ordered by lane count (`Three < Four`), like [`HdmiForumFrl`] by bandwidth.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HdmiLanes {
    /// Lanes 0–2 (TMDS data channels 0–2).
    Three,
    /// Lanes 0–3.
    Four,
}

impl HdmiLanes {
    /// The number of lanes, 3 or 4.
    pub const fn count(self) -> usize {
        match self {
            Self::Three => 3,
            Self::Four => 4,
        }
    }
}

impl PartialOrd for HdmiForumFrl {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HdmiForumFrl {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}

/// Maximum number of horizontal DSC slices supported by a HDMI 2.1 sink.
///
/// Source: HDMI 2.1a `DSC_MaxSlices` field.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HdmiDscMaxSlices {
    /// DSC not supported.
    NotSupported,
    /// Up to 1 slice; ≤ 340 MHz pixel clock per slice.
    Slices1,
    /// Up to 2 slices; ≤ 340 MHz per slice.
    Slices2,
    /// Up to 4 slices; ≤ 340 MHz per slice.
    Slices4,
    /// Up to 8 slices; ≤ 340 MHz per slice.
    Slices8At340Mhz,
    /// Up to 8 slices; ≤ 400 MHz per slice.
    Slices8At400Mhz,
    /// Up to 12 slices; ≤ 400 MHz per slice.
    Slices12,
    /// Up to 16 slices; ≤ 400 MHz per slice.
    Slices16,
}

impl HdmiDscMaxSlices {
    /// Decodes the `DSC_MaxSlices` nibble from the HDMI 2.1 SCDS.
    pub fn from_raw(raw: u8) -> Self {
        match raw {
            0 => Self::NotSupported,
            1 => Self::Slices1,
            2 => Self::Slices2,
            3 => Self::Slices4,
            4 => Self::Slices8At340Mhz,
            5 => Self::Slices8At400Mhz,
            6 => Self::Slices12,
            7 => Self::Slices16,
            _ => Self::NotSupported, // reserved values
        }
    }
}

/// Display Stream Compression (DSC) capabilities from the HF-SCDB.
///
/// Present only when the block carries the optional DSC section (≥ 10 bytes
/// of SCDS payload).
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HdmiForumDsc {
    /// Supports VESA DSC 1.2a compressed video transport.
    pub dsc_1p2: bool,
    /// Supports DSC for YCbCr 4:2:0 pixel encoding (`DSC_Native_420`).
    pub native_420: bool,
    /// QMS TFR_max: highest QMS frame rate equals `vrr_max_hz` (otherwise 60 Hz).
    pub qms_tfr_max: bool,
    /// QMS TFR_min: lowest QMS frame rate equals `vrr_min_hz` (otherwise 24/1.001 Hz).
    pub qms_tfr_min: bool,
    /// Supports DSC at any valid 1/16th-bit bits-per-pixel value (`DSC_All_BPC`).
    pub all_bpc: bool,
    /// Supports 12 bpc DSC compressed video transport.
    pub bpc12: bool,
    /// Supports 10 bpc DSC compressed video transport.
    pub bpc10: bool,
    /// Maximum FRL rate for DSC transport.
    pub max_frl_rate: HdmiForumFrl,
    /// Maximum number of horizontal DSC slices.
    pub max_slices: HdmiDscMaxSlices,
    /// Maximum total bytes per line of chunks: `1024 × (1 + raw_field)`. `0` = not reported.
    pub max_chunk_bytes: u32,
}

impl HdmiForumDsc {
    /// Constructs an `HdmiForumDsc`.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        dsc_1p2: bool,
        native_420: bool,
        qms_tfr_max: bool,
        qms_tfr_min: bool,
        all_bpc: bool,
        bpc12: bool,
        bpc10: bool,
        max_frl_rate: HdmiForumFrl,
        max_slices: HdmiDscMaxSlices,
        max_chunk_bytes: u32,
    ) -> Self {
        Self {
            dsc_1p2,
            native_420,
            qms_tfr_max,
            qms_tfr_min,
            all_bpc,
            bpc12,
            bpc10,
            max_frl_rate,
            max_slices,
            max_chunk_bytes,
        }
    }
}

/// Decoded HDMI Forum Sink Capability Data Block (HF-SCDB, extended tag `0x79`).
///
/// Defined in HDMI 2.1a section 10.3.6. Structure reconstructed from the Linux
/// kernel `drm_edid.c` and the edid-decode reference implementation.
///
/// The block contains two reserved bytes after the extended tag, followed by the
/// HDMI Sink Capability Data Structure (SCDS). Optional extended fields (VRR
/// range, DSC capabilities) are present only when the block is long enough.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HdmiForumSinkCap {
    /// SCDS version field; expected to be `1`.
    pub version: u8,
    /// Maximum TMDS character rate in MHz (`raw_byte × 5`). `0` means ≤ 340 MHz.
    pub max_tmds_rate_mhz: u16,

    // ---- SCDC and 3D flags (byte 2 of SCDS) ----
    /// Sink contains a Status and Control Data Channel (SCDC).
    pub scdc_present: bool,
    /// SCDC read request capable (`RR_Capable`).
    pub rr_capable: bool,
    /// Supports cable-status indication via SCDC.
    pub cable_status: bool,
    /// Supports Color Content Bits Per Component Indication (`CCBPCI`).
    pub ccbpci: bool,
    /// Supports scrambling at TMDS rates ≤ 340 Mcsc (`LTE_340Mcsc_Scramble`).
    pub lte_340mcsc_scramble: bool,
    /// Supports 3D Independent View signaling (`Independent_View`).
    pub independent_view_3d: bool,
    /// Supports 3D Dual View signaling (`Dual_View`).
    pub dual_view_3d: bool,
    /// Supports 3D OSD Disparity signaling (`OSD_Disparity`).
    pub osd_disparity_3d: bool,

    // ---- FRL and Deep Color flags (byte 3 of SCDS) ----
    /// Maximum Fixed Rate Link bandwidth.
    pub max_frl_rate: HdmiForumFrl,
    /// Supports UHD VIC indication in AVI InfoFrame (`UHD_VIC`).
    pub uhd_vic: bool,
    /// Supports 16 bpc (48-bit) Deep Color in YCbCr 4:2:0 (`DC_48bit_420`).
    pub dc_48bit_420: bool,
    /// Supports 12 bpc (36-bit) Deep Color in YCbCr 4:2:0 (`DC_36bit_420`).
    pub dc_36bit_420: bool,
    /// Supports 10 bpc (30-bit) Deep Color in YCbCr 4:2:0 (`DC_30bit_420`).
    pub dc_30bit_420: bool,

    // ---- Optional extended feature flags (byte 4 of SCDS; false if absent) ----
    /// Supports FAPA end extended to the Vactive upper bound of 360 µs.
    pub fapa_end_extended: bool,
    /// Supports QMS VRR (Quality Media Synchronization Variable Refresh Rate).
    pub qms: bool,
    /// Has a limit on rate-of-change variations in `M_VRR` values (`M_Delta`).
    pub m_delta: bool,
    /// Supports Fast VActive (`FVA`).
    pub fva: bool,
    /// Supports Auto Low-Latency Mode (`ALLM`).
    pub allm: bool,
    /// FAPA starts on first horizontal blank after first active video pixel.
    pub fapa_start_location: bool,
    /// Supports negative `M_VRR` values (`Neg_MVRR`).
    pub neg_mvrr: bool,

    // ---- Optional VRR range (bytes 5–6 of SCDS; None if absent) ----
    /// Minimum VRR frame rate in Hz. `Some(0)` means VRR is not supported.
    /// `None` means this section was not present in the block.
    pub vrr_min_hz: Option<u8>,
    /// Maximum VRR frame rate in Hz (10-bit). `Some(0)` means VRR is not supported.
    /// `None` means this section was not present in the block.
    pub vrr_max_hz: Option<u16>,

    // ---- Optional DSC capabilities (bytes 7–9 of SCDS; None if absent) ----
    /// Display Stream Compression capabilities. `None` if the DSC section is absent.
    pub dsc: Option<HdmiForumDsc>,
}

impl HdmiForumSinkCap {
    /// Constructs an `HdmiForumSinkCap`.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        version: u8,
        max_tmds_rate_mhz: u16,
        scdc_present: bool,
        rr_capable: bool,
        cable_status: bool,
        ccbpci: bool,
        lte_340mcsc_scramble: bool,
        independent_view_3d: bool,
        dual_view_3d: bool,
        osd_disparity_3d: bool,
        max_frl_rate: HdmiForumFrl,
        uhd_vic: bool,
        dc_48bit_420: bool,
        dc_36bit_420: bool,
        dc_30bit_420: bool,
        fapa_end_extended: bool,
        qms: bool,
        m_delta: bool,
        fva: bool,
        allm: bool,
        fapa_start_location: bool,
        neg_mvrr: bool,
        vrr_min_hz: Option<u8>,
        vrr_max_hz: Option<u16>,
        dsc: Option<HdmiForumDsc>,
    ) -> Self {
        Self {
            version,
            max_tmds_rate_mhz,
            scdc_present,
            rr_capable,
            cable_status,
            ccbpci,
            lte_340mcsc_scramble,
            independent_view_3d,
            dual_view_3d,
            osd_disparity_3d,
            max_frl_rate,
            uhd_vic,
            dc_48bit_420,
            dc_36bit_420,
            dc_30bit_420,
            fapa_end_extended,
            qms,
            m_delta,
            fva,
            allm,
            fapa_start_location,
            neg_mvrr,
            vrr_min_hz,
            vrr_max_hz,
            dsc,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lanes_of_every_rate() {
        let expected = [
            (HdmiForumFrl::NotSupported, HdmiLanes::Three),
            (HdmiForumFrl::Rate3Gbps3Lanes, HdmiLanes::Three),
            (HdmiForumFrl::Rate6Gbps3Lanes, HdmiLanes::Three),
            (HdmiForumFrl::Rate6Gbps4Lanes, HdmiLanes::Four),
            (HdmiForumFrl::Rate8Gbps4Lanes, HdmiLanes::Four),
            (HdmiForumFrl::Rate10Gbps4Lanes, HdmiLanes::Four),
            (HdmiForumFrl::Rate12Gbps4Lanes, HdmiLanes::Four),
        ];
        for (rate, lanes) in expected {
            assert_eq!(rate.lanes(), lanes, "{rate:?}");
        }
    }

    #[test]
    fn lanes_follow_the_raw_rate() {
        // Every raw `Max_FRL_Rate` nibble: four lanes from 3 to 6, three otherwise — the
        // 3-lane rates, 0 (TMDS) and the reserved values (decoded as `NotSupported`).
        for raw in 0..=15u8 {
            let lanes = HdmiForumFrl::from_raw(raw).lanes();
            let expected = match raw {
                3..=6 => HdmiLanes::Four,
                _ => HdmiLanes::Three,
            };
            assert_eq!(lanes, expected, "raw {raw}");
        }
    }

    #[test]
    fn lane_count_and_order() {
        assert_eq!(HdmiLanes::Three.count(), 3);
        assert_eq!(HdmiLanes::Four.count(), 4);
        assert!(HdmiLanes::Three < HdmiLanes::Four);
    }
}
