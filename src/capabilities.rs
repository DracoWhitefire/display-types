/// A reference-counted, type-erased warning value.
///
/// Any type that implements [`core::error::Error`] + [`Send`] + [`Sync`] + `'static` can be
/// wrapped in a `ParseWarning`. The built-in library variants use `EdidWarning`, but
/// custom handlers may push their own error types without wrapping them in `EdidWarning`.
///
/// Using [`Arc`][crate::prelude::Arc] (rather than `Box`) means `ParseWarning` is
/// [`Clone`], which lets warnings be copied from a parsed representation into
/// [`DisplayCapabilities`] without consuming the parsed result.
///
/// To inspect a specific variant, use the inherent `downcast_ref` method available on
/// `dyn core::error::Error + Send + Sync + 'static` in `std` builds:
///
/// ```text
/// for w in caps.iter_warnings() {
///     if let Some(ew) = (**w).downcast_ref::<EdidWarning>() { ... }
/// }
/// ```
#[cfg(any(feature = "alloc", feature = "std"))]
pub type ParseWarning = crate::prelude::Arc<dyn core::error::Error + Send + Sync + 'static>;

/// EDID specification version and revision, decoded from base block bytes 18–19.
///
/// Most displays in use report version 1 with revision 3 or 4.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EdidVersion {
    /// EDID version number (byte 18). Always `1` for all current displays.
    pub version: u8,
    /// EDID revision number (byte 19).
    pub revision: u8,
}

impl core::fmt::Display for EdidVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}", self.version, self.revision)
    }
}
