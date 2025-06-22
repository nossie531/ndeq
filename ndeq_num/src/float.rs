//! Provider of [`Float`].

use crate::util;
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

/// Floating point number.
pub trait Float:
    Copy
    + From<f32>
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
    /// Returns zero.
    fn zero() -> Self {
        0.0.into()
    }

    /// Returns one.
    fn one() -> Self {
        1.0.into()
    }

    /// Converts self into [`f32`].
    fn as_f32(self) -> f32;

    /// Returns 2^(self).
    fn exp2(self) -> Self;

    /// Returns the base 2 logarithm of the number.
    fn log2(self) -> Self;

    /// Returns this number with the sign equal to `sign`.
    fn copysign(self, sign: Self) -> Self;

    /// Returns exponential part.
    fn exponent(self) -> i32;

    /// Returns true if this value is infinity.
    fn is_infinite(self) -> bool {
        self.as_f32().is_infinite()
    }

    /// Returns `true` if this value is NaN.
    fn is_nan(self) -> bool {
        self.as_f32().is_nan()
    }

    /// Returns absolute value.
    fn abs(self) -> Self {
        if self < Self::zero() {
            Self::zero() - self
        } else {
            self
        }
    }

    /// Returns the maximum of the two numbers, ignoring NaN.
    fn max(self, other: Self) -> Option<Self> {
        match self.partial_cmp(&other) {
            Some(Ordering::Equal) => Some(self),
            Some(Ordering::Less) => Some(other),
            Some(Ordering::Greater) => Some(self),
            _ => None,
        }
    }

    /// Returns the minimum of the two numbers, ignoring NaN.
    fn min(self, other: Self) -> Option<Self> {
        match self.partial_cmp(&other) {
            Some(Ordering::Equal) => Some(self),
            Some(Ordering::Less) => Some(self),
            Some(Ordering::Greater) => Some(other),
            _ => None,
        }
    }
}

impl Float for f32 {
    fn as_f32(self) -> f32 {
        self
    }

    fn exp2(self) -> Self {
        self.exp2()
    }

    fn log2(self) -> Self {
        self.log2()
    }

    fn copysign(self, sign: Self) -> Self {
        self.copysign(sign)
    }

    fn exponent(self) -> i32 {
        util::exponent::<u32, { f32::MANTISSA_DIGITS }>(self.to_bits())
    }
}

impl Float for f64 {
    fn as_f32(self) -> f32 {
        self as f32
    }

    fn exp2(self) -> Self {
        self.exp2()
    }

    fn log2(self) -> Self {
        self.log2()
    }

    fn copysign(self, sign: Self) -> Self {
        self.copysign(sign)
    }

    fn exponent(self) -> i32 {
        util::exponent::<u64, { f64::MANTISSA_DIGITS }>(self.to_bits())
    }
}
