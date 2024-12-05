//! Provider of [`Time`].

use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

/// Time for diffusion.
pub trait Time:
    'static
    + Copy
    + Default
    + PartialOrd
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<f32, Output = Self>
    + Div<f32, Output = Self>
    + From<f32>
    + Into<f32>
{
    /// Returns zero value.
    fn zero() -> Self {
        Self::default()
    }

    /// Returns `true` if this value is NaN.
    fn is_nan(self) -> bool {
        self.into().is_nan()
    }

    /// Returns absolute value.
    fn abs(self) -> Self {
        if self < Self::zero() {
            Self::zero() - self
        } else {
            self
        }
    }

    /// Returns true if this value is infinity.
    fn is_infinite(self) -> bool {
        self.into().is_infinite()
    }

    /// Returns this number with the sign equal to `sign`.
    fn copysign(self, sign: Self) -> Self {
        Self::from(self.into().copysign(sign.into()))
    }

    /// Compares and returns the minimum of two values.
    fn min(self, other: Self) -> Option<Self> {
        match self.partial_cmp(&other) {
            Some(Ordering::Equal) => Some(self),
            Some(Ordering::Less) => Some(self),
            Some(Ordering::Greater) => Some(other),
            _ => None,
        }
    }
}

impl<T> Time for T
where
    T: 'static
        + Copy
        + Default
        + PartialOrd
        + Add<Self, Output = Self>
        + Sub<Self, Output = Self>
        + Mul<f32, Output = Self>
        + Div<f32, Output = Self>
        + From<f32>
        + Into<f32>,
{
    // nop.
}
