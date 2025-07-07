//! Provider of [`Float`].

use crate::util;
use dyn_compatible::prelude::*;
use std::cmp::Ordering;
use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Floating point number.
#[dyn_compatible(false)]
pub trait Float:
    'static
    + Copy
    + Default
    + From<f32>
    + PartialEq
    + PartialOrd
    + Sum
    + Product
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + for<'a> Sum<&'a Self>
    + for<'a> Product<&'a Self>
    + for<'a> AddAssign<&'a Self>
    + for<'a> SubAssign<&'a Self>
    + for<'a> MulAssign<&'a Self>
    + for<'a> DivAssign<&'a Self>
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> Mul<&'a Self, Output = Self>
    + for<'a> Div<&'a Self, Output = Self>
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
