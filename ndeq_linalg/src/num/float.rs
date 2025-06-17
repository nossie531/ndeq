//! Provider of [`Float`].

use std::ops::{Add, Div, Mul, Sub};
use crate::num;

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

    /// Returns 2^(self).
    fn exp2(self) -> Self;

    /// Returns the base 2 logarithm of the number.
    fn log2(self) -> Self;

    // Returns the maximum of the two numbers, ignoring NaN.
    fn max(self, other: Self) -> Self;

    /// Returns exponential part.
    fn exponent(self) -> i32;
}

impl Float for f32 {
    fn exp2(self) -> Self {
        self.exp2()
    }

    fn log2(self) -> Self {
        self.log2()
    }

    fn max(self, other: Self) -> Self {
        self.max(other)
    }
    
    fn exponent(self) -> i32 {
        num::exponent::<u32, 8>(self.to_bits())
    }
}

impl Float for f64 {
    fn exp2(self) -> Self {
        self.exp2()
    }

    fn log2(self) -> Self {
        self.log2()
    }

    fn max(self, other: Self) -> Self {
        self.max(other)
    }
    
    fn exponent(self) -> i32 {
        num::exponent::<u64, 11>(self.to_bits())
    }
}
