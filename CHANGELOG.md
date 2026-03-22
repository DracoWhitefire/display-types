# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
