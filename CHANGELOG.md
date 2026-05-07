# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `DisplayCapabilities::take_extension_data<T>` — removes and returns a stored extension
  data entry by tag for take-mutate-restore patterns where multiple input sources contribute
  to a single extension's capability struct (e.g. CTA-861 data delivered both via the
  CEA-861 extension block and via the DisplayID 2.x CTA DisplayID block 0x81). Requires
  `T: ExtensionData + Clone`; the entry is left in place if the stored type does not match.
- `RefreshRate` — exact rational refresh rate type replacing `VideoMode::refresh_rate: u16`.
  Stored as `(numer: u32, denom: u32)` in lowest terms (fields private; use `numer()` /
  `denom()` accessors). Constructors: `integral(hz: u32)`, `fractional(numer, denom)`, and
  `from_ratio(numer: u64, denom: u64) -> Option<Self>` for computing rates from large
  intermediate values such as `pixel_clock_hz / (h_total × v_total)`. `Deserialize` also
  reduces via `fractional`. Implements `Ord` via cross-multiplication, `Display` as `"60 Hz"`
  / `"60000/1001 Hz"`, and `From<u32>` / `From<u16>` for ergonomic construction. `as_f64()`
  returns the normalised value. Deliberately does **not** implement `Default` — there is no
  meaningful default refresh rate, and the field is now `Option<RefreshRate>` so an unset
  rate has its own representation.
- `ChromaticityPoint12` — 12-bit fixed-point chromaticity coordinate pair for DisplayID 2.x
  block 0x21. Accessor methods `x()` and `y()` normalise to `[0.0, 1.0)` by dividing by 4096.
- `Chromaticity12` — four `ChromaticityPoint12` values (three primaries and white point).
- `DisplayParamsV2` — display parameters from 2.x block 0x21: factory-calibrated chromaticity,
  IEEE 754 half-precision luminance (max full/10%, min as `Option<f32>`), color bit depth,
  display technology, gamma EOTF, scan orientation, and audio jack flag.
- `DisplayTechnology` enum — decodes byte 10 of block 0x21 into `Unspecified`, `Amlcd`,
  `Amoled`, or `Other(u8)` for reserved/vendor-specific values. Provides `from_byte` /
  `as_byte` for round-trippable decoding.
- `ScanOrientation` enum — decodes bits 2:0 of byte 11 of block 0x21 into the eight
  spec-defined fast-axis/slow-axis combinations (e.g. `LeftRightTopBottom` for conventional
  raster order). Provides `from_bits` / `as_bits`.
- `DynamicTimingRange` — dynamic timing range from 2.x block 0x25: min/max pixel clock in kHz
  (3-byte LE, 1 kHz resolution), min/max vertical refresh rate in Hz (9-bit), VRR support flag.
- `DisplayInterfaceFeatures` — interface features from 2.x block 0x26: per-encoding color depth
  bitmasks (RGB, YCbCr 4:4:4/4:2:2/4:2:0), minimum 4:2:0 pixel rate, audio flags, and color
  space/EOTF combination bitmask. Field doc comments now reference the source payload byte index.
  Color depth fields use typed `bitflags!` wrappers `ColorDepthsFull` (RGB and YCbCr 4:4:4 —
  6/8/10/12/14/16 bpc) and `ColorDepthsSubsampled` (YCbCr 4:2:2 and 4:2:0 — 8/10/12/14/16 bpc).
  `audio_flags` and `color_space_eotf_combos` (renamed from `color_space_eotf_1` — the
  `_1` suffix incorrectly implied sequel bytes; payload byte 6 is a single defined-combinations
  bitmask) remain `u8` pending full spec typing.
- `DisplayIdVendorSpecific` — envelope for 2.x block 0x7E: 3-byte IEEE OUI plus opaque
  vendor-defined `data: Vec<u8>`. The crate does not interpret payload semantics; consumers
  match on `oui` to dispatch to vendor-specific parsers (e.g. Dolby `00-D0-46`,
  Microsoft `CA-12-5C`). `requires alloc` (or `std`).
- `DisplayIdStereoInterfaceV2` — stereo display interface from 2.x block 0x27, alongside
  supporting types `StereoTimingScopeV2` (4 variants from revision bits 7:6),
  `StereoViewingMethodV2` (FieldSequential / SideBySide / PixelInterleaved / DualInterface /
  MultiView / StackedFrame / Proprietary / Reserved with method-specific parameters),
  `StereoEye` (Left / Right), and `DualInterfaceMirroring` (None / LeftRight / TopBottom /
  Reserved). `FieldSequential` carries `eye_on_high_half: StereoEye` (parallel to
  `SideBySide.left_half`) rather than a `right_eye_polarity_high: bool`. Inline timing-code
  list (when present) is detectable via `DisplayIdStereoInterfaceV2::has_timing_codes` but
  not currently parsed.
- `Default` derive (or impl) on `DisplayParamsV2`, `DynamicTimingRange`,
  `DisplayInterfaceFeatures`, and `DisplayIdStereoInterfaceV2`. Required to construct these
  `#[non_exhaustive]` structs from downstream crates (struct expressions are forbidden across
  crate boundaries); decoders use `Default::default()` plus field assignment.
- `DisplayIdCapabilities` gains six new `Option` fields and one `Vec` field:
  `manufacturer_oui: Option<[u8; 3]>`, `display_params_v2: Option<DisplayParamsV2>`,
  `dynamic_timing_range: Option<DynamicTimingRange>`,
  `interface_features: Option<DisplayInterfaceFeatures>`,
  `stereo_interface_v2: Option<DisplayIdStereoInterfaceV2>`, `container_id: Option<[u8; 16]>`,
  and `vendor_specific: Vec<DisplayIdVendorSpecific>`. Options default to `None`, the Vec
  defaults to empty; `new()` initialises them accordingly.
- `tag` module: V2 tag constants `V2_PRODUCT_ID` (0x20) through `V2_CONTAINER_ID` (0x29),
  `V2_VENDOR_SPECIFIC` (0x7E), and `V2_CTA_DISPLAYID` (0x81).
- `CvtAlgorithm` enum — CVT formula selector for DisplayID 2.x Type V (`0x11`) and Type IX
  (`0x24`) descriptors: `Cvt` (encoding `0`, standard CVT, no reduced blanking), `CvtRb`
  (encoding `1`, CVT-RB v1), `CvtR2` (encoding `2`, CVT-RB v2 / "CVT-R2"), plus
  `Reserved(u8)` for spec-reserved encodings (`3`–`7`). Provides `from_bits(b)` decoding the
  3-bit field; marked `#[non_exhaustive]`.
- `TypeIxStereoMode` enum — per-mode stereo indicator decoded from Type V and Type IX
  descriptor byte 0 bits 6:5: `Mono` (`0b00`), `Stereo` (`0b01`),
  `MonoOrStereoByUser` (`0b10`), `Reserved` (`0b11`). Marked `#[non_exhaustive]`.
- `VideoMode::cvt_algorithm: Option<CvtAlgorithm>` — populated from Type V/IX byte 0 bits 2:0;
  `None` for all other sources.
- `VideoMode::y420: bool` — `true` for YCbCr 4:2:0-only modes. Set by CTA-861 Y420 VDB /
  capability map signalling and by DisplayID 2.x Type VII (block revision ≥ 2, byte 3 bit 7).
  Defaults to `false` for all other sources.
- `VideoMode::ntsc_fractional_refresh: bool` — `true` when NTSC-style fractional refresh
  (× 1000/1001) is supported alongside this timing. Decoded from Type V and Type IX
  descriptor byte 0 bit 4. Defaults to `false` for all other sources.
- `VideoMode::type_ix_stereo: Option<TypeIxStereoMode>` — per-mode stereo indicator from
  Type V/IX byte 0 bits 6:5. `None` for all other timing sources.
- `VideoMode::with_cvt_algorithm(alg)`, `VideoMode::with_y420(b)`,
  `VideoMode::with_ntsc_fractional_refresh(b)`, and `VideoMode::with_type_ix_stereo(s)` —
  builders for the new fields, consistent with `with_pixel_clock` / `with_source` /
  `with_detailed_timing`.
- `compute_type_ix_timing(width, height, refresh_rate, algorithm) -> Option<ComputedTiming>`
  in `display_types::timing` — evaluates the named CVT formula to derive pixel clock and
  blanking parameters from the four-tuple a DisplayID 2.x Type IX descriptor carries.
  Implements **CVT-RB v1** (`CvtRb`, VESA CVT 1.1 §3.4) and **CVT-RB v2 / CVT-R2**
  (`CvtR2`, VESA CVT 1.2 §4). Reference values: RB v1 1920×1080@60 = 138.500 MHz,
  2560×1440@60 = 241.500 MHz, 3840×2160@30 = 262.750 MHz; RB v2 1920×1080@60 =
  133.320 MHz, 2560×1440@120 = 483.120 MHz, 3840×2160@60 = 522.614 MHz. RB v2 differs
  from v1 in halved H blanking (80 vs 160 px), 1 kHz pixel-clock step (vs 0.25 MHz),
  wider V_SYNC (8 vs 4 lines), and slack lives in V_FPORCH rather than V_BPORCH.
  Standard CVT (`Cvt`) and reserved algorithm codes return `None`.
- `ComputedTiming` struct — `pixel_clock_khz`, `h_total`, `v_total`, `h_front_porch`,
  `h_sync_width`, `v_front_porch`, `v_sync_width`. Designed to feed
  `VideoMode::with_detailed_timing` directly. Marked `#[non_exhaustive]`.
- `CustomColorSpaceEotfCombo` — custom `(color space, EOTF)` pair from DisplayID 2.x block
  0x26 payload bytes 9+. Fields `color_space: u8` (bits 7:4, values 0–7) and `eotf: u8`
  (bits 3:0, values 0–10) hold raw nibble-sized indices as defined in the DisplayID 2.x §4.6
  table. Constructor: `CustomColorSpaceEotfCombo::new(color_space, eotf)`.
- `DisplayInterfaceFeatures::custom_color_space_eotf_combos: [CustomColorSpaceEotfCombo; 7]`
  and `custom_color_space_eotf_count: u8` — the custom `(color space, EOTF)` pairs from 0x26
  payload byte 8 (count, 0–7) and bytes 9+ (one packed byte per pair). Entries beyond the
  count are uninitialised and must not be read; iterate `0..custom_color_space_eotf_count`.
- `StereoTimingCodeType` enum — code-space selector for a [`StereoTimingCode`]: `Dmt`
  (bits 7:6 = `0b00`), `Vic` (`0b01`), `HdmiVic` (`0b10`), `Reserved` (`0b11`).
- `StereoTimingCode` — one entry from the inline timing-code list in a DisplayID 2.x block
  0x27. Fields `code_type: StereoTimingCodeType` and `code: u8`. Constructor:
  `StereoTimingCode::new(code_type, code)`.
- `DisplayIdStereoInterfaceV2::timing_codes: Vec<StereoTimingCode>` — decoded inline timing
  codes from block 0x27 when `has_timing_codes()` is `true`. Each record in the payload is a
  1-byte header (bits 7:6 = type, bits 4:0 = count) followed by that many 1-byte code values.
  Available in `alloc`/`std` builds only; empty in no_alloc builds.
- **SLSA Build Level 2 provenance** — release artifacts are attested via
  `actions/attest-build-provenance` and verified with
  `gh attestation verify <file> --repo DracoWhitefire/display-types`.

### Breaking changes

- `DisplayIdStereoInterfaceV2` no longer implements `Copy`. The new
  `timing_codes: Vec<StereoTimingCode>` field (alloc-gated) is not `Copy`; callers that
  relied on implicit copy semantics must switch to `.clone()`.
- `CvtAlgorithm` variants corrected to match the DisplayID 2.x spec (cross-referenced
  against `edid-decode`). The prior variant set (`CvtRb1` / `CvtRb2` / `CvtRb3` /
  `ReducedBlankingCvtRb1` / `ReducedBlankingCvtRb2`) was based on a misreading of the
  spec — bits 2:0 = `0` is standard CVT (no reduced blanking), not CVT-RB v1. The
  corrected variants are `Cvt` (0), `CvtRb` (1), `CvtR2` (2), `Reserved(u8)` (3–7).
  `compute_type_ix_timing` dispatch updated accordingly: `Cvt` returns `None` (no
  evaluator), `CvtRb` → CVT-RB v1 evaluator, `CvtR2` → CVT-RB v2 evaluator.
- `VideoMode::refresh_rate` changed from `u16` to `Option<RefreshRate>`. `VideoMode::new` now
  accepts `impl Into<RefreshRate>` for the refresh rate parameter and stores it as `Some(...)`,
  so integer literals require a `u32` suffix (e.g. `60u32`) or explicit `RefreshRate::integral(60)`.
  Default-constructed `VideoMode` (and any code reading the field) must handle the `None` case.
  `pixel_clock_khz` returns `0` when `refresh_rate` is `None` and no DTD pixel clock is set.
- DMT 0x58 (4096×2160) is now stored as `RefreshRate::fractional(60000, 1001)` (≈ 59.94 Hz)
  rather than the truncated `60`.
- `DisplayIdCapabilities` no longer derives `Eq` (only `PartialEq`). The new
  `display_params_v2: Option<DisplayParamsV2>` field contains `Option<f32>` luminance values,
  which are `PartialEq` but not `Eq`. Downstream code that required `Eq`
  (e.g. `HashSet<DisplayIdCapabilities>`, trait bounds) must switch to `PartialEq`.

### Internal

- Fixed coverage ratchet CI: added `LC_NUMERIC=C` to the baseline `printf` to prevent
  locale-dependent decimal separators from corrupting `.coverage-baseline` on non-C locales.

## [0.3.1] - 2026-03-28

### Added

- `ResolvedDisplayConfig` — hardware-ready output type produced by a negotiation engine,
  carrying the fields a DRM driver or InfoFrame encoder needs to configure a link:
  `VideoMode`, `ColorFormat`, `ColorBitDepth`, `HdmiForumFrl`, `dsc_required: bool`, and
  `vrr_applicable: bool`. Lives in `display-types` so downstream consumers can depend on
  it without a direct dependency on the negotiation engine. Constructor:
  `ResolvedDisplayConfig::new(mode, color_encoding, bit_depth, frl_rate, dsc_required,
  vrr_applicable)`. Marked `#[non_exhaustive]`.
- `HdmiForumFrl` re-exported at the crate root (`display_types::HdmiForumFrl`) for
  convenience alongside `ColorFormat`, `ColorBitDepth`, and the other negotiation types;
  previously only accessible as `display_types::cea861::HdmiForumFrl`.
- **Dependency audit pipeline** - dependencies get checked on cargo manifest changes.

## [0.3.0] - 2026-03-25

### Added

- `ModeSource` — enum recording the source from which a `VideoMode` was decoded:
  `Vic(u8)` for CTA-861 Video Identification Codes, `DmtId(u16)` for VESA DMT table
  entries, and `DtdIndex(u8)` for Detailed Timing Descriptors (zero-based index within
  the containing EDID block). Marked `#[non_exhaustive]`.
- `VideoMode::source: Option<ModeSource>` — populated automatically by `vic_to_mode` and
  `dmt_to_mode`; parsers decoding DTDs should set it via `with_source`. `None` for modes
  constructed directly via `VideoMode::new`. Preserves the identifier that was silently
  dropped before, enabling reliable KMS mode correlation and per-mode capability checks
  (e.g. CTA Y420 VDB / CMDB).
- `VideoMode::with_source(ModeSource) -> Self` — builder for setting the source, consistent
  with `with_pixel_clock` and `with_detailed_timing`.
- `VideoMode::with_pixel_clock(pixel_clock_khz: u32) -> Self` — builder that sets the exact
  pixel clock in kHz, bypassing the CVT-RB fallback in `pixel_clock_khz()`. Intended for
  firmware and embedded callers that have the exact clock from a hardware PLL or timing
  register but do not have full Detailed Timing Descriptor fields. Chain after
  `VideoMode::new`: `VideoMode::new(w, h, r, i).with_pixel_clock(clk)`.

### Breaking changes

- `pixel_clock_khz_cvt_rb_estimate` was renamed to `pixel_clock_khz` to avoid the suggestion that it always estimates.
- `VideoMode::refresh_rate` is now `u16` (was `u8`). The `VideoMode::new` constructor signature changes accordingly.
 Values previously clamped to 255 now reflect the true encoded rate; 360 Hz panels and the 256 Hz maximum of DisplayID
 Type V descriptors are represented correctly.

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
