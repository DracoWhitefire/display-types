# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Breaking changes

- `pixel_clock_khz_cvt_rb_estimate` was renamed to `pixel_clock_khz` to avoid the suggestion that it always estimates.

## [0.2.2] - 2026-03-24

### Added

- `pixel_clock_khz_cvt_rb_estimate(mode: &VideoMode) -> u32` — free function in
  `display_types::timing` that returns the pixel clock in kHz for any `VideoMode`.
  When `mode.pixel_clock_khz` is `Some`, returns that exact value unchanged. When it is
  `None` (modes decoded from standard timings, established timings, or SVD entries), applies
  the CVT Reduced Blanking fixed-blanking model: `(width + 160) × (height + 8) × refresh_rate / 1000`.
  Accurate to ~2% for typical consumer resolutions using CVT-RB timings; biased toward
  under-estimation, so suitable as a conservative input to TMDS bandwidth ceiling checks
  but not as a substitute for an exact clock.

### Internal

- Unit tests for `color_capabilities_from_edid` covering all code paths: RGB with and
  without VSDB, base-depth fallback, DC flag combinations, `DC_Y444` interaction with
  YCbCr 4:4:4 depth mirroring, YCbCr 4:2:2 fixed at 8 bpc, and all YCbCr 4:2:0 deep
  color flag combinations.
- Coverage ratchet: CI now measures line coverage with `cargo-llvm-cov` across `std`
  and `serde` feature sets and fails if coverage drops below `.coverage-baseline`. When
  coverage improves on a push to `main` or `develop`, CI opens a PR automatically to
  ratchet the baseline forward.

## [0.2.1] - 2026-03-24

### Added

- `ColorBitDepths` — compact u8 bitset of supported bit depths for a single color
  format. Constants `BPC_6`…`BPC_16`; methods `is_empty()`, `supports(ColorBitDepth)`,
  and `with(ColorBitDepth)` for building and querying the set.
- `ColorCapabilities` — aggregate of four `ColorBitDepths` fields (one per `ColorFormat`:
  `rgb444`, `ycbcr444`, `ycbcr422`, `ycbcr420`). Method `for_format(&self, ColorFormat)`
  returns the supported depths for that format. Replaces the scattered
  `DigitalColorEncoding` + `ColorBitDepth` + Deep Color booleans as the primary
  color-capability surface.
- `color_capabilities_from_edid(encoding, base_depth, hdmi_vsdb, hdmi_forum) -> ColorCapabilities`
  — free function that derives a `ColorCapabilities` from the four raw EDID/HDMI fields
  that encode color support: the EDID base block encoding field, the base block bit depth
  field, the HDMI 1.x VSDB deep color flags, and the HF-SCDB YCbCr 4:2:0 deep color flags.
  Plain 8 bpc YCbCr 4:2:0 (signaled via the CEA/CTA Y420VDB) is not covered by these
  fields; callers should supplement `ycbcr420` with `BPC_8` after calling this function
  when that block is present.

## [0.2.0] - 2026-03-23

### Breaking changes

- `VideoMode::with_detailed_timing` has a new first parameter `pixel_clock_khz: u32`.

  Before:
  ```rust
  VideoMode::new(w, h, r, i).with_detailed_timing(
      h_front_porch, h_sync_width,
      v_front_porch, v_sync_width,
      h_border, v_border,
      stereo, sync,
  )
  ```
  After:
  ```rust
  VideoMode::new(w, h, r, i).with_detailed_timing(
      pixel_clock_khz,
      h_front_porch, h_sync_width,
      v_front_porch, v_sync_width,
      h_border, v_border,
      stereo, sync,
  )
  ```
  The pixel clock is the first two bytes of a DTD (little-endian, in 10 kHz units for
  EDID/DisplayID Type I–II; 1 kHz units for DisplayID Type VI). Multiply accordingly before
  passing.

### Added

- `VideoMode::pixel_clock_khz: Option<u32>` — pixel clock in kHz, populated from Detailed
  Timing Descriptors. `None` for modes decoded from Standard Timings or SVD entries, which
  carry no authoritative clock value.
- `ColorFormat` — a single color encoding format (`Rgb444`, `YCbCr444`, `YCbCr422`,
  `YCbCr420`) for use in negotiated or candidate configurations. Distinct from
  `DigitalColorEncoding`, which models the 2-bit EDID base block field. `YCbCr420` is
  included here because it is signaled through CEA/CTA extension blocks rather than the
  base block.
- `HdmiForumFrl` now implements `PartialOrd` and `Ord`. Ordering is by bandwidth: higher
  variant = greater link capacity. The implementation compares the spec `Max_FRL_Rate`
  discriminant values directly rather than relying on declaration order.

### Changed

- `HdmiForumFrl` is now `#[repr(u8)]` with explicit discriminants matching the HDMI 2.1a
  `Max_FRL_Rate` nibble values (0–6).
- `vic_to_mode` now populates `pixel_clock_khz` for all VICs 1–64 (CEA-861-E) and
  65–127, 193–219 (CTA-861-I). Pixel clocks are sourced from the CEA-861/CTA-861 spec.
- `dmt_to_mode` now populates `pixel_clock_khz` for all DMT IDs 0x01–0x58. Pixel clocks
  are sourced from VESA DMT v1.13.

## [0.1.3] - 2026-03-22

### Added

**DisplayID 1.x types** (`display_types::displayid`)

- `DisplayIdCapabilities` — version byte and product primary use case decoded from a
  DisplayID section header (`alloc`/`std` only); retrieve via
  `caps.get_extension_data::<DisplayIdCapabilities>(0x70)`
- `displayid::tag` — data block tag constants (`PRODUCT_ID`, `DISPLAY_PARAMS`,
  `COLOR_CHARACTERISTICS`, `TYPE_I_TIMING` … `TYPE_VI_TIMING`; all 20 implemented tags)
- `displayid::product_type` — display product primary use case constants (`EXTENSION`,
  `TEST`, `MONITOR`, `TV`, `REPEATER`, `DIRECT_DRIVE`) for comparing against
  `DisplayIdCapabilities::product_type`

## [0.1.2] - 2026-03-22

### Added

**CEA-861 / CTA-861 extension types** (`display_types::cea861`)

- `Cea861Flags` — capability flags from byte 3 of a CEA-861 extension block (underscan,
  basic audio, YCbCr 4:4:4/4:2:2)
- `Cea861Capabilities` — all decoded data from a CEA-861 extension block, including VICs,
  audio descriptors, colorimetry, HDR metadata, speaker allocation, vendor-specific blocks,
  and more (`alloc`/`std` only)
- `HdmiAudioBlock` — HDMI Audio Data Block (extended tag `0x12`), carrying Multi-Stream
  Audio support flag and Short Audio Descriptors (`alloc`/`std` only)
- `AudioFormat`, `AudioFormatInfo`, `AudioSampleRates`, `ShortAudioDescriptor` — audio
  descriptor types from CEA Audio Data Blocks (tag `0x01`)
- `HdmiVsdb`, `HdmiVsdbFlags` — HDMI 1.x Vendor-Specific Data Block (OUI `0x000C03`)
- `VideoCapability`, `VideoCapabilityFlags` — Video Capability Data Block (extended tag `0x00`)
- `ColorimetryBlock`, `ColorimetryFlags` — Colorimetry Data Block (extended tag `0x05`)
- `HdrEotf`, `HdrStaticMetadata`, `HdrDynamicMetadataDescriptor` — HDR Static and Dynamic
  Metadata Data Blocks (extended tags `0x06`, `0x07`)
- `SpeakerAllocationFlags`, `SpeakerAllocationFlags2`, `SpeakerAllocationFlags3`,
  `SpeakerAllocation`, `RoomConfigurationBlock`, `SpeakerLocationEntry` — Speaker Allocation,
  Room Configuration, and Speaker Location Data Blocks
- `DtcPointEncoding`, `VesaTransferCharacteristic` — VESA Display Transfer Characteristic
  Data Block (standard tag `0x05`; `VesaTransferCharacteristic` is `alloc`/`std` only)
- `HdmiForumFrl`, `HdmiDscMaxSlices`, `HdmiForumDsc`, `HdmiForumSinkCap` — HDMI Forum
  Sink Capability Data Block (extended tags `0x78`, `0x79`)
- `T7VtdbBlock`, `T8VtdbBlock`, `T10VtdbEntry`, `T10VtdbBlock`, `VtbExtBlock` — DisplayID
  Type VII/VIII/X Video Timing Data Blocks and VESA VTB-EXT (`T8VtdbBlock`, `T10VtdbBlock`,
  `VtbExtBlock` are `alloc`/`std` only)
- `InfoFrameDescriptor`, `infoframe_type` — InfoFrame Data Block (extended tag `0x20`)
- `VendorSpecificBlock` — Vendor-Specific Video/Audio Data Blocks (extended tags `0x01`,
  `0x11`; `alloc`/`std` only)
- `VesaDisplayDeviceBlock` — VESA Display Device Data Block (extended tag `0x02`)
- `vic_to_mode(vic: u8) -> Option<VideoMode>` — resolves a CEA-861 Video Identification
  Code (VICs 1–64 and CTA-861-I VICs 65–127, 193–219) to a `VideoMode`
- `dmt_to_mode(id: u16) -> Option<VideoMode>` — resolves a VESA DMT ID (0x01–0x58) to
  a `VideoMode`

**OUI constants** (`display_types::cea861::oui`)

- `HDMI_LICENSING` (`0x000C03`) — HDMI Licensing, LLC (HDMI 1.x VSDB)
- `HDMI_FORUM` (`0xC45DD8`) — HDMI Forum (HF-VSDB)
- `DOLBY_VISION` (`0x00D046`) — Dolby Laboratories
- `HDR10_PLUS` (`0x90848B`) — Samsung Electronics / HDR10+ Technology

## [0.1.1] - 2026-03-22

### Added

- `VideoMode::new(width, height, refresh_rate, interlaced)` — constructor for simple
  (non-DTD) modes such as those decoded from established timings, standard timings, and SVDs
- `VideoMode::with_detailed_timing(h_front_porch, h_sync_width, v_front_porch, v_sync_width,
  h_border, v_border, stereo, sync) -> Self` — builder that sets the blanking-interval and
  sync fields; intended to be chained after `VideoMode::new` for DTD-sourced modes

### Changed

- `VideoMode` is now marked `#[non_exhaustive]`, consistent with all other public structs
  in this crate. External crates must use `VideoMode::new` (and optionally
  `with_detailed_timing`) instead of struct literal syntax.

## [0.1.0] - 2026-03-22

### Added

**Core capability type**
- `DisplayCapabilities` — the top-level struct carrying all decoded display properties,
  produced by EDID/DisplayID parsers and consumed by negotiation engines
- `ExtensionData` — type-erased trait for extension-specific decoded data attached to
  `DisplayCapabilities`
- `ParseWarning` — `Arc<dyn Error + Send + Sync>` alias for parser-emitted diagnostic values
- `EdidVersion` — EDID standard version and revision
- `VideoMode` — a single supported video timing (resolution, refresh rate, sync, blanking)
- `SyncDefinition` — sync type decoded from a detailed timing descriptor
- `StereoMode` — stereo viewing mode decoded from a detailed timing descriptor

**Color types** (`display_types::color`)
- `Chromaticity` — CIE xy primary and white point coordinates decoded from EDID bytes 25–34
- `ChromaticityPoint` — a single CIE xy coordinate pair
- `WhitePoint` — additional white point descriptor
- `ColorManagementData` — DCM channel coefficients (a3, a2 per channel)
- `DcmChannel` — per-channel DCM coefficients
- `ColorBitDepth` — digital interface color bit depth per primary
- `DigitalColorEncoding` — digital color encoding standard (RGB, YCbCr 4:4:4 / 4:2:2)
- `AnalogColorType` — analog display color type
- `DisplayGamma` — display gamma value (100× encoded integer)

**Input types** (`display_types::input`)
- `VideoInputFlags` — bitflags for video input definition byte
- `VideoInterface` — digital video interface standard
- `AnalogSyncLevel` — analog sync signal level

**Feature flags** (`display_types::features`)
- `DisplayFeatureFlags` — bitflags for EDID Feature Support byte (byte 24)

**Manufacture types** (`display_types::manufacture`)
- `ManufacturerId` — ISA PNP three-letter manufacturer code
- `ManufactureDate` — week and year decoded from EDID bytes 16–17
- `MonitorString` — heap-allocated (`alloc`/`std`) or fixed-length (`no_std`) monitor string

**Screen size** (`display_types::screen`)
- `ScreenSize` — physical dimensions, aspect ratio, or undefined

**Timing formula types** (`display_types::timing`)
- `TimingFormula` — secondary timing formula reported in display range limits (GTF, CVT, or none)
- `GtfSecondaryParams` — secondary GTF curve parameters
- `CvtSupportParams` — CVT support parameters including pixel clock adjustment and scaling
- `CvtAspectRatios` — bitflags for supported CVT aspect ratios
- `CvtAspectRatio` — preferred CVT aspect ratio
- `CvtScaling` — bitflags for display scaling capabilities

**Panel and interface types** (`display_types::panel`)
- `DisplayTechnology` — panel technology (LCD, OLED, etc.)
- `OperatingMode` — display operating mode
- `BacklightType` — backlight technology
- `PhysicalOrientation` — physical panel mounting orientation
- `RotationCapability` — supported rotation angles
- `ZeroPixelLocation` — location of the zero pixel
- `ScanDirection` — horizontal and vertical scan direction
- `SubpixelLayout` — sub-pixel arrangement
- `DisplayInterfaceType` — interface standard (DisplayPort, HDMI, etc.)
- `InterfaceContentProtection` — content protection supported on the interface
- `DisplayIdInterface` — decoded DisplayID interface block fields
- `StereoViewingMode` — stereo viewing method
- `StereoSyncInterface` — sync interface used for stereo
- `DisplayIdStereoInterface` — decoded DisplayID stereo interface block fields
- `PowerSequencing` — display power sequence timing parameters (T1–T6)
- `TileBezelInfo` — bezel widths for a tiled display tile
- `TileTopologyBehavior` — tiled display topology behavior flags
- `DisplayIdTiledTopology` — decoded DisplayID tiled topology block fields
- `DisplayIdStereoInterface` — decoded DisplayID stereo interface block fields

**Transfer characteristic types** (`display_types::transfer`)
- `TransferPointEncoding` — bit depth used to pack luminance sample values (8, 10, or 12 bit)
- `TransferCurve` — luminance transfer curve samples, single or per-primary (`alloc`/`std` only)
- `DisplayIdTransferCharacteristic` — decoded DisplayID Transfer Characteristics block
  (`alloc`/`std` only)

**Feature flags**
- `std` *(default)* — enables `std`-dependent types; implies `alloc`
- `alloc` — enables heap-allocated types (`Vec`, `Arc`, `String`) without full `std`
- `serde` — derives `Serialize`/`Deserialize` for all public types via `serde` and `bitflags`

**Project infrastructure**
- `#![no_std]`, `#![forbid(unsafe_code)]`, `#![deny(missing_docs)]`
- `#[non_exhaustive]` on all public structs and enums for forward-compatible extensibility
- `pub fn new(...)` constructors on all `#[non_exhaustive]` structs
- Full rustdoc coverage enforced via `cargo rustdoc -- -D missing_docs`
- CI workflow: format, clippy, docs, tests across `std`, `std + serde`, `alloc`, and bare `no_std`
- Publish workflow: triggered on version tags, gated to commits reachable from `main`
