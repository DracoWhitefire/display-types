//! DisplayID 1.x display product primary use case constants.
//!
//! These are the values of the `product_type` field in
//! `DisplayIdCapabilities`, decoded from bits 2:0 of byte 3 of the
//! DisplayID section header (DisplayID 1.x §4.1).

/// Extension-only section; no primary display product use case.
pub const EXTENSION: u8 = 0x00;

/// Test structure / reserved.
pub const TEST: u8 = 0x01;

/// Computer monitor.
pub const MONITOR: u8 = 0x02;

/// TV receiver.
pub const TV: u8 = 0x03;

/// Repeater / translator (e.g. AV receiver, dock).
pub const REPEATER: u8 = 0x04;

/// Direct-drive monitor (panel without external scaler).
pub const DIRECT_DRIVE: u8 = 0x05;
