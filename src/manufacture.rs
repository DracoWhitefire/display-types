/// Manufacture date or model year, decoded from EDID base block bytes 16–17.
///
/// | Byte 16 | Meaning                                              |
/// |---------|------------------------------------------------------|
/// | `0x00`  | Week unspecified; byte 17 is the manufacture year.  |
/// | `0x01`–`0x36` | Week of manufacture (1–54).               |
/// | `0xFF`  | Byte 17 is a model year, not a manufacture year.    |
///
/// Year is encoded as `byte_17 + 1990`.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManufactureDate {
    /// The display was manufactured in the given year.
    /// `week` is `None` if byte 16 was `0x00` (week unspecified).
    Manufactured {
        /// Week of manufacture (1–54), if specified.
        week: Option<u8>,
        /// Year of manufacture.
        year: u16,
    },
    /// The year identifies a model year rather than a manufacture date.
    ModelYear(u16),
}

/// A three-character PNP manufacturer identifier, decoded from EDID base block bytes `0x08`–`0x09`.
///
/// Each character is an ASCII uppercase letter (A–Z). Valid IDs are registered with the IANA
/// PNP registry. Well-known examples: `GSM` (LG), `SAM` (Samsung), `DEL` (Dell).
///
/// # Invariant
///
/// All three bytes must be ASCII uppercase letters (`b'A'`–`b'Z'`, i.e. `0x41`–`0x5A`).
/// The library only constructs this type after validating that constraint, and
/// deserialization (with the `serde` feature) rejects bytes that violate it. If you construct
/// one manually via the public field, you are responsible for maintaining the invariant:
/// [`as_str`][Self::as_str] panics in debug builds and returns `""` in release builds if it
/// is violated, and the `Display` impl renders the raw bytes escaped (e.g. `\xff\x00\x00`).
///
/// Use [`ManufacturerId::from_ascii`] or [`TryFrom<[u8; 3]>`][TryFrom] for a checked
/// construction path.
///
/// Available in all build configurations including bare `no_std`. The `Display` impl renders
/// the three-character string directly, so `format!("{}", id)` and `id.to_string()` both work
/// wherever a `Display` bound is satisfied.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "[u8; 3]", into = "[u8; 3]"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManufacturerId(pub [u8; 3]);

impl ManufacturerId {
    /// Constructs a `ManufacturerId` from three raw bytes, returning `None` if any byte is
    /// not an ASCII uppercase letter (`A`–`Z`).
    pub fn from_ascii(bytes: [u8; 3]) -> Option<Self> {
        if bytes.iter().all(|&b| b.is_ascii_uppercase()) {
            Some(Self(bytes))
        } else {
            None
        }
    }

    /// Returns the ID as a `&str` slice.
    ///
    /// If the stored bytes are not ASCII uppercase letters (the type invariant was violated
    /// at construction time), panics in debug builds and returns `""` in release builds.
    pub fn as_str(&self) -> &str {
        let valid = self.is_valid();
        debug_assert!(
            valid,
            "ManufacturerId invariant violated: bytes must be ASCII uppercase A-Z, got {:?}",
            self.0
        );
        if valid {
            // ASCII uppercase bytes are always valid UTF-8.
            core::str::from_utf8(&self.0).unwrap_or("")
        } else {
            ""
        }
    }

    fn is_valid(&self) -> bool {
        self.0.iter().all(u8::is_ascii_uppercase)
    }
}

impl TryFrom<[u8; 3]> for ManufacturerId {
    type Error = &'static str;

    /// Checked construction; equivalent to [`ManufacturerId::from_ascii`].
    fn try_from(bytes: [u8; 3]) -> Result<Self, Self::Error> {
        Self::from_ascii(bytes).ok_or("ManufacturerId bytes must be ASCII uppercase A-Z")
    }
}

impl From<ManufacturerId> for [u8; 3] {
    fn from(id: ManufacturerId) -> Self {
        id.0
    }
}

impl core::fmt::Display for ManufacturerId {
    /// Renders the three-letter ID. Bytes that violate the type invariant are rendered
    /// escaped (e.g. `\xff\x00\x00`) rather than panicking.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.is_valid() {
            f.write_str(self.as_str())
        } else {
            write!(f, "{}", self.0.escape_ascii())
        }
    }
}

/// A monitor descriptor string, decoded from one of the 18-byte descriptor slots in the
/// EDID base block (`0xFC` monitor name, `0xFF` serial number, `0xFE` unspecified text).
///
/// The text payload occupies bytes 5–17 of the descriptor (13 bytes), terminated by `0x0A`
/// and padded with spaces. The `as_str()` method strips both the terminator and trailing
/// spaces, returning a clean `&str`.
///
/// Available in all build configurations including bare `no_std`. `Deref<Target = str>`
/// is implemented so `Option<MonitorString>::as_deref()` returns `Option<&str>` directly.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonitorString(pub [u8; 13]);

impl MonitorString {
    /// Returns the string content with the `0x0A` terminator and trailing spaces stripped.
    ///
    /// Returns an empty `&str` if the payload is all padding or not valid UTF-8.
    pub fn as_str(&self) -> &str {
        let bytes = &self.0;
        let end = bytes.iter().position(|&b| b == 0x0A).unwrap_or(bytes.len());
        let trimmed = match bytes[..end].iter().rposition(|&b| b != b' ') {
            Some(i) => &bytes[..=i],
            None => &[][..],
        };
        core::str::from_utf8(trimmed).unwrap_or("")
    }
}

impl core::fmt::Display for MonitorString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl core::ops::Deref for MonitorString {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(any(feature = "alloc", feature = "std"))]
    use crate::alloc::string::ToString;

    // --- ManufacturerId ---

    #[test]
    fn manufacturer_id_valid_ascii_uppercase() {
        let id = ManufacturerId::from_ascii(*b"DEL").unwrap();
        assert_eq!(id.as_str(), "DEL");
    }

    #[test]
    fn manufacturer_id_rejects_lowercase() {
        assert!(ManufacturerId::from_ascii(*b"del").is_none());
    }

    #[test]
    fn manufacturer_id_rejects_digit() {
        assert!(ManufacturerId::from_ascii(*b"D3L").is_none());
    }

    #[test]
    #[cfg(any(feature = "alloc", feature = "std"))]
    fn manufacturer_id_display() {
        let id = ManufacturerId::from_ascii(*b"SAM").unwrap();
        assert_eq!(id.to_string(), "SAM");
    }

    #[test]
    fn manufacturer_id_try_from_checks_invariant() {
        assert_eq!(
            ManufacturerId::try_from(*b"GSM"),
            Ok(ManufacturerId(*b"GSM"))
        );
        assert!(ManufacturerId::try_from([0xFF, 0x00, 0x00]).is_err());
    }

    #[test]
    #[cfg(any(feature = "alloc", feature = "std"))]
    fn manufacturer_id_display_escapes_invalid_bytes() {
        // Reachable only by bypassing the invariant via the public field. Display must
        // render the real bytes without panicking, in debug and release builds alike.
        assert_eq!(
            ManufacturerId([0xFF, 0x00, 0x00]).to_string(),
            r"\xff\x00\x00"
        );
        assert_eq!(ManufacturerId(*b"del").to_string(), "del");
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn manufacturer_id_as_str_invalid_bytes_is_empty_in_release() {
        assert_eq!(ManufacturerId([0xFF, 0x00, 0x00]).as_str(), "");
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "ManufacturerId invariant violated")]
    fn manufacturer_id_as_str_invalid_bytes_panics_in_debug() {
        let _ = ManufacturerId([0xFF, 0x00, 0x00]).as_str();
    }

    #[test]
    #[cfg(all(feature = "serde", any(feature = "alloc", feature = "std")))]
    fn manufacturer_id_serde_wire_format_is_byte_array() {
        let id = ManufacturerId(*b"DEL");
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "[68,69,76]");
        assert_eq!(serde_json::from_str::<ManufacturerId>(&json).unwrap(), id);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn manufacturer_id_deserialize_rejects_invalid_bytes() {
        // Previously deserialized successfully and then panicked in `Display`.
        assert!(serde_json::from_str::<ManufacturerId>("[255,0,0]").is_err());
        assert!(serde_json::from_str::<ManufacturerId>("[100,101,108]").is_err()); // "del"
    }

    // --- MonitorString ---

    #[test]
    fn monitor_string_strips_terminator_and_spaces() {
        let mut buf = [b' '; 13];
        let name = b"DELL U2722D";
        buf[..name.len()].copy_from_slice(name);
        buf[name.len()] = 0x0A;
        assert_eq!(MonitorString(buf).as_str(), "DELL U2722D");
    }

    #[test]
    fn monitor_string_no_terminator_strips_trailing_spaces() {
        let mut buf = [b' '; 13];
        buf[..3].copy_from_slice(b"ABC");
        assert_eq!(MonitorString(buf).as_str(), "ABC");
    }

    #[test]
    fn monitor_string_all_padding_gives_empty() {
        assert_eq!(MonitorString([b' '; 13]).as_str(), "");
    }

    #[test]
    fn monitor_string_deref() {
        let mut buf = [b' '; 13];
        buf[..3].copy_from_slice(b"LEN");
        buf[3] = 0x0A;
        let ms = MonitorString(buf);
        let s: &str = &ms;
        assert_eq!(s, "LEN");
    }

    #[test]
    #[cfg(any(feature = "alloc", feature = "std"))]
    fn monitor_string_display() {
        let mut buf = [b' '; 13];
        buf[..3].copy_from_slice(b"GSM");
        buf[3] = 0x0A;
        assert_eq!(MonitorString(buf).to_string(), "GSM");
    }
}
