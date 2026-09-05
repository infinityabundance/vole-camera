//! The canonical sensor sample domain (§17).
//!
//! **Normative.** The initial canonical sensor domain is:
//!
//! - unsigned integer,
//! - 10 active bits,
//! - stored logically in `u16`,
//! - range `0..1023`.
//!
//! Canonical internal identity is independent of transport packing (§18). The
//! packed RAW10 and unpacked `u16` forms are boundary representations only; the
//! canonical sample is a value in `0..=1023`.

/// A canonical sensor sample.
///
/// A 10-bit unsigned photosite value in `0..=1023`, stored in a `u16`. The
/// upper 6 bits are always zero in the canonical domain.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SensorSample(u16);

/// Number of active bits in the canonical sensor domain.
pub const SENSOR_ACTIVE_BITS: u16 = 10;
/// Maximum inclusive value in the canonical sensor domain.
pub const SENSOR_MAX_VALUE: u16 = (1 << SENSOR_ACTIVE_BITS) - 1; // 1023

impl SensorSample {
    /// Construct a canonical sample, erroring on out-of-range values.
    pub fn new(value: u16) -> crate::error::CameraResult<Self> {
        if value > SENSOR_MAX_VALUE {
            return Err(crate::error::CameraError::SampleOutOfRange(value));
        }
        Ok(SensorSample(value))
    }

    /// The raw 10-bit value.
    pub const fn value(self) -> u16 {
        self.0
    }
}

impl TryFrom<u16> for SensorSample {
    type Error = crate::error::CameraError;
    fn try_from(v: u16) -> Result<Self, Self::Error> {
        Self::new(v)
    }
}

impl From<SensorSample> for u16 {
    fn from(s: SensorSample) -> u16 {
        s.0
    }
}

/// The declared finite range of a sensor sample (inclusive).
pub const SAMPLE_RANGE: std::ops::RangeInclusive<u16> = 0..=SENSOR_MAX_VALUE;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_full_range() {
        assert_eq!(SensorSample::new(0).unwrap().value(), 0);
        assert_eq!(SensorSample::new(1023).unwrap().value(), 1023);
    }

    #[test]
    fn rejects_out_of_range() {
        assert!(SensorSample::new(1024).is_err());
        assert!(SensorSample::new(u16::MAX).is_err());
    }
}
