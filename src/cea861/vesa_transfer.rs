#[cfg(any(feature = "alloc", feature = "std"))]
use crate::prelude::Vec;

pub use crate::TransferPointEncoding as DtcPointEncoding;

/// Decoded VESA Display Transfer Characteristic Data Block (standard tag `0x05`).
///
/// Encodes the display's luminance transfer function as a sequence of sample
/// points at evenly-spaced input levels from 0 (black) to 1 (white).
#[non_exhaustive]
#[cfg(any(feature = "alloc", feature = "std"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)] // no Eq: contains f32
pub struct VesaTransferCharacteristic {
    /// Bit depth used to encode each luminance point.
    pub encoding: DtcPointEncoding,
    /// Luminance values, normalized to [0.0, 1.0].
    pub points: Vec<f32>,
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl VesaTransferCharacteristic {
    /// Constructs a `VesaTransferCharacteristic`.
    pub fn new(encoding: DtcPointEncoding, points: Vec<f32>) -> Self {
        Self { encoding, points }
    }
}
