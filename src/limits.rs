//! Typed execution limits (§89, §211–§212).
//!
//! **Normative.** Every recording declares and obeys explicit bounds for
//! geometry, bit depth, duration, observation count, object count, calibration
//! size, transition count, tile count, dependency depth, and work. A
//! conforming decoder rejects any declaration exceeding [`Limits`] **before**
//! allocating, so hostile input can never drive an out-of-memory condition from
//! an unchecked length.

/// Global execution envelope shared by encoder and decoder.
///
/// These values are deliberately conservative for the first (simulator)
/// milestone and will be tightened or extended by evidence. They are the single
/// source of truth for "is this declared geometry safe to even consider?".
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    /// Maximum sensor width (pixels).
    pub max_width: u64,
    /// Maximum sensor height (pixels).
    pub max_height: u64,
    /// Maximum active bit depth (samples are `1..=active_bits` unsigned).
    pub max_bit_depth: u16,
    /// Maximum number of observations in a single recording.
    pub max_observation_count: u64,
    /// Maximum calibration object payload bytes in a recording.
    pub max_calibration_bytes: u64,
    /// Maximum object count in a recording.
    pub max_object_count: u64,
    /// Maximum transition count in a recording.
    pub max_transition_count: u64,
    /// Maximum residual tile count in a recording.
    pub max_tile_count: u64,
    /// Maximum model/dependency depth (procedural recursion bound).
    pub max_dependency_depth: u16,
    /// Maximum bytes of a single residual tile (guard against absurd tiles).
    pub max_tile_bytes: u64,
}

impl Default for Limits {
    fn default() -> Self {
        Limits {
            // Full IMX415 geometry (3864×2192) plus headroom, with a hard cap that
            // still rejects "4 billion × 4 billion" declarations (§212).
            max_width: 16_384,
            max_height: 16_384,
            // The canonical domain is 10-bit; the ceiling leaves room for a
            // cautious 16-bit future without weakening the default.
            max_bit_depth: 16,
            // A 15-minute capture at a generous 120 fps is 108000 observations.
            max_observation_count: 10_000_000,
            max_calibration_bytes: 256 * 1024 * 1024,
            max_object_count: 1_000_000,
            max_transition_count: 10_000_000,
            max_tile_count: 100_000_000,
            max_dependency_depth: 32,
            max_tile_bytes: 256 * 1024 * 1024,
        }
    }
}

impl Limits {
    /// Validate a declared geometry against the envelope, returning the checked
    /// product (width×height) without overflow so callers can size scratch.
    pub fn validate_geometry(&self, width: u64, height: u64) -> crate::error::CameraResult<u64> {
        if width == 0 || height == 0 {
            return Err(crate::error::CameraError::Limit(
                "zero dimension in geometry",
            ));
        }
        if width > self.max_width {
            return Err(crate::error::CameraError::Limit("width exceeds Limits"));
        }
        if height > self.max_height {
            return Err(crate::error::CameraError::Limit("height exceeds Limits"));
        }
        width
            .checked_mul(height)
            .ok_or(crate::error::CameraError::Limit(
                "geometry product overflow",
            ))
    }

    /// Validate a bit depth against the envelope.
    pub fn validate_bit_depth(&self, depth: u16) -> crate::error::CameraResult<()> {
        if depth == 0 || depth > self.max_bit_depth {
            return Err(crate::error::CameraError::Limit("bit depth exceeds Limits"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_zero_dimension() {
        let l = Limits::default();
        assert!(l.validate_geometry(0, 10).is_err());
        assert!(l.validate_geometry(10, 0).is_err());
    }

    #[test]
    fn rejects_absurd_geometry() {
        let l = Limits::default();
        assert!(l.validate_geometry(4_000_000_000, 4_000_000_000).is_err());
    }

    #[test]
    fn accepts_imx415_geometry() {
        let l = Limits::default();
        assert_eq!(l.validate_geometry(3864, 2192).unwrap(), 3864 * 2192);
    }

    #[test]
    fn product_does_not_overflow_then_get_silently_accepted() {
        // Even a valid width×height near the cap must not overflow u64; here it
        // is within u64 but the product is checked explicitly.
        let l = Limits::default();
        let w = l.max_width;
        let h = l.max_height;
        assert_eq!(l.validate_geometry(w, h).unwrap(), w * h);
    }
}
