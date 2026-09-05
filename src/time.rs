//! Exact rational time and rational arithmetic.
//!
//! **Normative.** Camera sensor timing is represented exactly as reduced
//! rational numbers, never as `f64` seconds (§19). A row readout time, exposure
//! duration, or frame period is a ratio of integers in fixed units (ticks).
//!
//! This module deliberately **does not** name a physical unit. The tick is an
//! abstract clock; a concrete sensor profile binds it to a real clock (e.g.
//! "1 tick = 1 pixel clock, line period = `line_period_num/line_period_den`").
//!
//! All arithmetic is checked and panics on overflow rather than silently
//! wrapping (see §210).

use std::fmt;
use std::ops::{Add, Mul};

use crate::error::CameraResult;

/// A reduced non-negative rational number `num / den` (`den > 0`).
///
/// Used for exact timing. Not limited to seconds; the unit is abstract.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rational {
    num: u64,
    den: u64,
}

impl Rational {
    /// Construct a rational, reducing to lowest terms. `den` must be non-zero.
    pub fn new(num: u64, den: u64) -> crate::error::CameraResult<Self> {
        if den == 0 {
            return Err(crate::error::CameraError::InvalidRational);
        }
        let g = gcd(num, den);
        Ok(Rational {
            num: num / g,
            den: den / g,
        })
    }

    /// The rational `n / 1`.
    pub const fn from_u64(n: u64) -> Self {
        Rational { num: n, den: 1 }
    }

    pub const fn numerator(&self) -> u64 {
        self.num
    }

    pub const fn denominator(&self) -> u64 {
        self.den
    }

    /// Exact multiplication (checked); reduces the result to lowest terms.
    pub fn checked_mul(self, other: Self) -> CameraResult<Self> {
        // (a/b)*(c/d) reduced: multiply numerators and denominators with checked
        // u64 arithmetic, then reduce.
        let n = self
            .num
            .checked_mul(other.num)
            .ok_or(crate::error::CameraError::ArithmeticOverflow)?;
        let d = self
            .den
            .checked_mul(other.den)
            .ok_or(crate::error::CameraError::ArithmeticOverflow)?;
        Self::new(n, d)
    }

    /// Exact addition (checked); reduces the result to lowest terms.
    pub fn checked_add(self, other: Self) -> CameraResult<Self> {
        // a/b + c/d = (a*d + c*b) / (b*d)
        let ad = self
            .num
            .checked_mul(other.den)
            .ok_or(crate::error::CameraError::ArithmeticOverflow)?;
        let cb = other
            .num
            .checked_mul(self.den)
            .ok_or(crate::error::CameraError::ArithmeticOverflow)?;
        let n = ad
            .checked_add(cb)
            .ok_or(crate::error::CameraError::ArithmeticOverflow)?;
        let d = self
            .den
            .checked_mul(other.den)
            .ok_or(crate::error::CameraError::ArithmeticOverflow)?;
        Self::new(n, d)
    }

    /// Exact subtraction (checked), returning an error on underflow.
    ///
    /// `Rational` is a non-negative magnitude, so `a - b` with `b > a` is not
    /// representable and is reported rather than silently wrapped.
    pub fn checked_sub(self, other: Self) -> CameraResult<Self> {
        let ad = self
            .num
            .checked_mul(other.den)
            .ok_or(crate::error::CameraError::ArithmeticOverflow)?;
        let cb = other
            .num
            .checked_mul(self.den)
            .ok_or(crate::error::CameraError::ArithmeticOverflow)?;
        let n = ad
            .checked_sub(cb)
            .ok_or(crate::error::CameraError::ArithmeticOverflow)?;
        let d = self
            .den
            .checked_mul(other.den)
            .ok_or(crate::error::CameraError::ArithmeticOverflow)?;
        Self::new(n, d)
    }
}

impl Add for Rational {
    type Output = Rational;
    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs)
            .expect("rational Add overflow (use checked_add for fallible paths)")
    }
}

impl Mul for Rational {
    type Output = Rational;
    fn mul(self, rhs: Self) -> Self::Output {
        self.checked_mul(rhs)
            .expect("rational Mul overflow (use checked_mul for fallible paths)")
    }
}

impl fmt::Debug for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.den == 1 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

/// Greatest common divisor (non-negative args; `gcd(0,0)` is defined as 0).
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// A timestamp and duration carried as exact rational ticks.
///
/// `RationalTime` pairs a frame-absolute start tick with a duration, both in a
/// shared abstract tick unit. The rolling-shutter model derives per-row times
/// as `t0 + y * line_period` (§20, §26).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct RationalTime {
    /// Start of the interval, in ticks from the recording epoch.
    start: Rational,
    /// Duration of the interval (may describe an exposure, a frame, or a row).
    duration: Rational,
}

impl RationalTime {
    pub fn new(start: Rational, duration: Rational) -> Self {
        RationalTime { start, duration }
    }

    pub const fn start(&self) -> Rational {
        self.start
    }

    pub const fn duration(&self) -> Rational {
        self.duration
    }

    /// End tick (start + duration).
    pub fn end(&self) -> crate::error::CameraResult<Rational> {
        self.start.checked_add(self.duration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduces_to_lowest_terms() {
        let r = Rational::new(2, 4).unwrap();
        assert_eq!(r.numerator(), 1);
        assert_eq!(r.denominator(), 2);
    }

    #[test]
    fn zero_denominator_rejected() {
        assert!(Rational::new(1, 0).is_err());
    }

    #[test]
    fn add_and_mul_are_exact() {
        let a = Rational::new(1, 3).unwrap();
        let b = Rational::new(1, 6).unwrap();
        assert_eq!(a.checked_add(b).unwrap(), Rational::new(1, 2).unwrap());
        assert_eq!(a.checked_mul(b).unwrap(), Rational::new(1, 18).unwrap());
    }

    #[test]
    fn checked_sub_underflows_cleanly() {
        let a = Rational::new(1, 3).unwrap();
        let b = Rational::new(1, 2).unwrap();
        assert!(a.checked_sub(b).is_err());
    }

    #[test]
    fn rolling_shutter_row_time() {
        // t_y = t0 + y * T_line, computed exactly.
        let t0 = Rational::from_u64(1000);
        let line = Rational::new(10, 3).unwrap();
        let t_y = t0
            .checked_add(line.checked_mul(Rational::from_u64(5)).unwrap())
            .unwrap();
        // 1000 + 5*(10/3) = 1000 + 50/3 = (3000+50)/3 = 3050/3
        assert_eq!(t_y, Rational::new(3050, 3).unwrap());
    }
}
