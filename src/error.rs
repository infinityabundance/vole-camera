//! Typed error and result surface.
//!
//! **Normative.** Every fallible path returns a [`CameraResult`] carrying a
//! typed [`CameraError`]. A conforming decoder never panics on hostile input;
//! it returns one of these errors (§90, §233).

use thiserror::Error;

/// The crate-wide result alias.
pub type CameraResult<T> = Result<T, CameraError>;

/// Typed, deterministic error for every fallible VOLE-Camera path.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CameraError {
    /// A zero-denominator rational was constructed (would be division by zero).
    #[error("invalid rational: zero denominator")]
    InvalidRational,

    /// Checked integer arithmetic overflowed (never silently wraps).
    #[error("arithmetic overflow")]
    ArithmeticOverflow,

    /// A sensor sample value exceeded the canonical 10-bit domain (`0..=1023`).
    #[error("sensor sample {0} out of range 0..=1023")]
    SampleOutOfRange(u16),

    /// An unknown/unsupported CFA pattern name was provided.
    #[error("invalid CFA pattern")]
    InvalidCfa,

    /// A malformed or unsupported input was encountered.
    #[error("malformed input: {0}")]
    Malformed(&'static str),

    /// A declared geometry/size violates [`crate::limits::Limits`].
    #[error("limit exceeded: {0}")]
    Limit(&'static str),

    /// A declared geometry is internally inconsistent (e.g. stride/row order).
    #[error("inconsistent geometry: {0}")]
    Inconsistent(&'static str),

    /// A requested coordinate or region lies outside the sensor array.
    #[error("out of bounds: {0}")]
    OutOfBounds(&'static str),
}
