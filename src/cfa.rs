//! Color Filter Array (CFA / Bayer) pattern and photosite components.
//!
//! **Normative.** The canonical simulated capture is the CFA sensor lattice,
//! not a pre-demosaiced RGB image (§16). Each photosite samples exactly one
//! color component determined by its spatial coordinates and the declared CFA
//! origin.
//!
//! Support the four 2×2 Bayer orderings (§16) plus the two green components
//! distinguished by neighbor (the "red-row green" vs "blue-row green" split is
//! derived from coordinates, so only four patterns and four component labels
//! are needed).

use crate::error::{CameraError, CameraResult};

/// A single color component sampled at a photosite.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u8)]
pub enum CfaComponent {
    /// Green in the row that also carries red (GR / top-left in RGGB family,
    /// top-right in GRBG family, etc. — resolved by [`CfaPattern::component`]).
    Red = 0,
    /// Green in the row that also carries red.
    GreenRed = 1,
    /// Green in the row that also carries blue.
    GreenBlue = 2,
    Blue = 3,
}

impl CfaComponent {
    pub const ALL: [CfaComponent; 4] = [
        CfaComponent::Red,
        CfaComponent::GreenRed,
        CfaComponent::GreenBlue,
        CfaComponent::Blue,
    ];

    /// Whether this component is a green photosite.
    pub const fn is_green(self) -> bool {
        matches!(self, CfaComponent::GreenRed | CfaComponent::GreenBlue)
    }
}

/// A 2×2 Bayer CFA pattern.
///
/// The four letters name the top-left 2×2 block of the mosaic, reading
/// row-major (top row left-to-right, then bottom row). The pattern repeats
/// identically across the sensor.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum CfaPattern {
    /// R G / G B
    Rggb,
    /// G R / B G
    Grbg,
    /// G B / R G
    Gbrg,
    /// B G / G R
    Bggr,
}

impl CfaPattern {
    pub const ALL: [CfaPattern; 4] = [
        CfaPattern::Rggb,
        CfaPattern::Grbg,
        CfaPattern::Gbrg,
        CfaPattern::Bggr,
    ];

    /// The color component sampled at `(x, y)`.
    ///
    /// `x` is the column (0 = leftmost), `y` the row (0 = topmost). The pattern
    /// is regarded as fixed to the sensor lattice, so a one-pixel shift changes
    /// the component sampled at a spatial position (`§48`).
    pub fn component(self, x: u32, y: u32) -> CfaComponent {
        // bit 0 of (x,y) selects within the 2×2 block; bit 1 combined with the
        // pattern's 4-bit mask selects the concrete channel.
        let x0 = (x & 1) as usize;
        let y0 = (y & 1) as usize;
        // A 2×2 matrix [top-left, top-right, bottom-left, bottom-right] indexed
        // as row*2 + col, where each entry is one of R/Gr/Gb/B.
        const M: [[CfaComponent; 4]; 4] = [
            // RGGB
            [
                CfaComponent::Red,
                CfaComponent::GreenRed,
                CfaComponent::GreenBlue,
                CfaComponent::Blue,
            ],
            // GRBG
            [
                CfaComponent::GreenRed,
                CfaComponent::Red,
                CfaComponent::Blue,
                CfaComponent::GreenBlue,
            ],
            // GBRG
            [
                CfaComponent::GreenBlue,
                CfaComponent::Blue,
                CfaComponent::Red,
                CfaComponent::GreenRed,
            ],
            // BGGR
            [
                CfaComponent::Blue,
                CfaComponent::GreenBlue,
                CfaComponent::GreenRed,
                CfaComponent::Red,
            ],
        ];
        let pi = self as usize;
        M[pi][y0 * 2 + x0]
    }

    /// Short name for reporting (`"RGGB"`, `"GRBG"`, `"GBRG"`, `"BGGR"`).
    pub fn name(self) -> &'static str {
        match self {
            CfaPattern::Rggb => "RGGB",
            CfaPattern::Grbg => "GRBG",
            CfaPattern::Gbrg => "GBRG",
            CfaPattern::Bggr => "BGGR",
        }
    }

    /// Parse a canonical CFA name (case-insensitive). Rejects anything else.
    pub fn parse(s: &str) -> CameraResult<Self> {
        match s.trim().to_ascii_uppercase().as_str() {
            "RGGB" => Ok(CfaPattern::Rggb),
            "GRBG" => Ok(CfaPattern::Grbg),
            "GBRG" => Ok(CfaPattern::Gbrg),
            "BGGR" => Ok(CfaPattern::Bggr),
            _ => Err(CameraError::InvalidCfa),
        }
    }
}

impl std::fmt::Display for CfaPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rggb_2x2_block() {
        let p = CfaPattern::Rggb;
        assert_eq!(p.component(0, 0), CfaComponent::Red);
        assert_eq!(p.component(1, 0), CfaComponent::GreenRed);
        assert_eq!(p.component(0, 1), CfaComponent::GreenBlue);
        assert_eq!(p.component(1, 1), CfaComponent::Blue);
    }

    #[test]
    fn grbg_2x2_block() {
        let p = CfaPattern::Grbg;
        assert_eq!(p.component(0, 0), CfaComponent::GreenRed);
        assert_eq!(p.component(1, 0), CfaComponent::Red);
        assert_eq!(p.component(0, 1), CfaComponent::Blue);
        assert_eq!(p.component(1, 1), CfaComponent::GreenBlue);
    }

    #[test]
    fn gbrg_2x2_block() {
        let p = CfaPattern::Gbrg;
        assert_eq!(p.component(0, 0), CfaComponent::GreenBlue);
        assert_eq!(p.component(1, 0), CfaComponent::Blue);
        assert_eq!(p.component(0, 1), CfaComponent::Red);
        assert_eq!(p.component(1, 1), CfaComponent::GreenRed);
    }

    #[test]
    fn bggr_2x2_block() {
        let p = CfaPattern::Bggr;
        assert_eq!(p.component(0, 0), CfaComponent::Blue);
        assert_eq!(p.component(1, 0), CfaComponent::GreenBlue);
        assert_eq!(p.component(0, 1), CfaComponent::GreenRed);
        assert_eq!(p.component(1, 1), CfaComponent::Red);
    }

    #[test]
    fn pattern_repeats_on_odd_dimensions() {
        // 3×3 RGGB: every 2×2 block repeats, so (2,0) == (0,0), (2,2) == (0,0).
        let p = CfaPattern::Rggb;
        assert_eq!(p.component(2, 0), CfaComponent::Red);
        assert_eq!(p.component(2, 2), CfaComponent::Red);
        assert_eq!(p.component(2, 1), CfaComponent::GreenBlue);
    }

    #[test]
    fn parse_round_trip() {
        for p in CfaPattern::ALL {
            assert_eq!(CfaPattern::parse(p.name()).unwrap(), p);
        }
        assert!(CfaPattern::parse("rggb").is_ok());
        assert!(CfaPattern::parse("YYYS").is_err());
    }

    #[test]
    fn one_pixel_shift_changes_component() {
        let p = CfaPattern::Rggb;
        // Moving right one pixel alternates R/Gr on even rows.
        assert_ne!(p.component(0, 0), p.component(1, 0));
        // Moving down one pixel alternates R/Gb on even columns.
        assert_ne!(p.component(0, 0), p.component(0, 1));
    }
}
