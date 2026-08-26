//! The scalar backend behind [`ByteSize`](crate::ByteSize) arithmetic.
//!
//! Two float representations sit behind the `lossless` feature: plain `f64`
//! when it is off, and an exact [`fraction::GenericFraction`] when it is on.
//! Rather than branch on the feature at every arithmetic site with textual
//! macros (which is what previously let the non-lossless build break silently,
//! since the macro bodies were never type-checked together), both backends
//! implement one [`Numeric`] trait. The compiler then type-checks the byte
//! math across every feature set, and a reader can offload the backend choice
//! onto the trait and trust it.

use core::str::FromStr;

#[cfg(feature = "lossless")]
use crate::Float;
use crate::Int;

/// The arithmetic surface [`ByteSize`](crate::ByteSize) needs from its scalar
/// backend, implemented uniformly by both the `f64` and the exact-fraction
/// representations so the byte math is written once and checked everywhere.
pub(crate) trait Numeric: Copy + PartialOrd + Sized {
    /// Lift a backing integer into the scalar domain.
    fn from_int(value: Int) -> Self;

    /// Lift a small unsigned magnitude (a size-variant factor) into the domain.
    fn from_small(value: u8) -> Self;

    /// Collapse back to a whole byte count, saturating rather than panicking on
    /// any non-finite or negative state (overflow to infinity to [`Int::MAX`],
    /// NaN or negative to zero).
    fn to_int(self) -> Int;

    /// Whether the value is exactly zero.
    fn is_zero(self) -> bool;

    /// Whether the value is exactly one.
    fn is_one(self) -> bool;

    /// Multiply, saturating to the representation's maximum on overflow when the
    /// `no-panic` feature is on and multiplying plainly otherwise.
    fn saturating_mul(self, rhs: Self) -> Self;

    /// Divide, saturating to the representation's maximum on overflow when the
    /// `no-panic` feature is on and dividing plainly otherwise.
    fn saturating_div(self, rhs: Self) -> Self;

    /// Parse a decimal string into the scalar domain, `None` if it is not a
    /// number the backend understands.
    fn parse(s: &str) -> Option<Self>;
}

#[cfg(not(feature = "lossless"))]
impl Numeric for f64 {
    #[inline]
    fn from_int(value: Int) -> Self {
        value as f64
    }

    #[inline]
    fn from_small(value: u8) -> Self {
        value as f64
    }

    #[inline]
    fn to_int(self) -> Int {
        // `as` already saturates finite out-of-range values and maps NaN to 0.
        self as Int
    }

    #[inline]
    fn is_zero(self) -> bool {
        self == 0.0
    }

    #[inline]
    fn is_one(self) -> bool {
        self == 1.0
    }

    #[inline]
    fn saturating_mul(self, rhs: Self) -> Self {
        // f64 saturates to infinity on overflow either way; there is no checked
        // form to branch on, so the `no-panic` distinction is a no-op here.
        self * rhs
    }

    #[inline]
    fn saturating_div(self, rhs: Self) -> Self {
        self / rhs
    }

    #[inline]
    fn parse(s: &str) -> Option<Self> {
        f64::from_str(s).ok()
    }
}

#[cfg(feature = "lossless")]
impl Numeric for Float {
    #[inline]
    fn from_int(value: Int) -> Self {
        Float::from(value)
    }

    #[inline]
    fn from_small(value: u8) -> Self {
        Float::from(value)
    }

    #[inline]
    fn to_int(self) -> Int {
        match self {
            fraction::GenericFraction::Rational(fraction::Sign::Plus, r) => r.numer() / r.denom(),
            fraction::GenericFraction::Infinity(fraction::Sign::Plus) => Int::MAX,
            _ => 0,
        }
    }

    #[inline]
    fn is_zero(self) -> bool {
        fraction::Zero::is_zero(&self)
    }

    #[inline]
    fn is_one(self) -> bool {
        fraction::One::is_one(&self)
    }

    #[inline]
    fn saturating_mul(self, rhs: Self) -> Self {
        #[cfg(feature = "no-panic")]
        {
            use fraction::CheckedMul;
            self.checked_mul(&rhs).unwrap_or_else(max_saturate)
        }
        #[cfg(not(feature = "no-panic"))]
        {
            self * rhs
        }
    }

    #[inline]
    fn saturating_div(self, rhs: Self) -> Self {
        #[cfg(feature = "no-panic")]
        {
            use fraction::CheckedDiv;
            self.checked_div(&rhs).unwrap_or_else(max_saturate)
        }
        #[cfg(not(feature = "no-panic"))]
        {
            self / rhs
        }
    }

    #[inline]
    fn parse(s: &str) -> Option<Self> {
        Float::from_str(s).ok()
    }
}

/// The representation's maximum, used as the overflow ceiling under `no-panic`.
#[cfg(all(feature = "lossless", feature = "no-panic"))]
#[inline]
fn max_saturate() -> Float {
    fraction::Bounded::max_value()
}
