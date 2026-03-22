#![no_std]
#![forbid(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

/// Re-exports of `alloc`/`std` collection types used across the crate.
pub mod prelude;
#[cfg(any(feature = "alloc", feature = "std"))]
pub use prelude::{Arc, Box, String, Vec};

/// Color-related model types.
pub mod color;
pub use color::{
    AnalogColorType, Chromaticity, ChromaticityPoint, ColorBitDepth, ColorManagementData,
    DcmChannel, DigitalColorEncoding, DisplayGamma, WhitePoint,
};

/// Input interface model types.
pub mod input;
pub use input::{AnalogSyncLevel, VideoInputFlags, VideoInterface};

/// Display feature flags.
pub mod features;
pub use features::DisplayFeatureFlags;

/// Manufacture date model types.
pub mod manufacture;
pub use manufacture::{ManufactureDate, ManufacturerId, MonitorString};

/// Screen size and aspect ratio.
pub mod screen;
pub use screen::ScreenSize;

/// Video timing formula types.
pub mod timing;
pub use timing::{
    CvtAspectRatio, CvtAspectRatios, CvtScaling, CvtSupportParams, GtfSecondaryParams,
    TimingFormula,
};

/// Panel hardware characteristic types.
pub mod panel;
pub use panel::{
    BacklightType, DisplayIdInterface, DisplayIdStereoInterface, DisplayIdTiledTopology,
    DisplayInterfaceType, DisplayTechnology, InterfaceContentProtection, OperatingMode,
    PhysicalOrientation, PowerSequencing, RotationCapability, ScanDirection, StereoSyncInterface,
    StereoViewingMode, SubpixelLayout, TileBezelInfo, TileTopologyBehavior, ZeroPixelLocation,
};

/// Luminance transfer characteristic types.
pub mod transfer;

/// Consumer-facing capability types.
pub mod capabilities;
pub use capabilities::EdidVersion;
pub use transfer::TransferPointEncoding;
#[cfg(any(feature = "alloc", feature = "std"))]
pub use transfer::{DisplayIdTransferCharacteristic, TransferCurve};
