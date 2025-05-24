//! Provider of [`Float`].

use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

/// Float type abstraction trait.
pub trait Float:
    Copy
    + Default
    + Sized
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
    /// Converts self into [`f32`].
    fn as_f32(self) -> f32;

    /// Returns this number with the sign equal to `sign`.
    fn copysign(self, sign: Self) -> Self;

    /// Returns true if this value is infinity.
    fn is_infinite(self) -> bool {
        self.as_f32().is_infinite()
    }

    /// Returns `true` if this value is NaN.
    fn is_nan(self) -> bool {
        self.as_f32().is_nan()
    }

    /// Returns zero value.
    fn zero() -> Self {
        Self::default()
    }

    /// Returns absolute value.
    fn abs(self) -> Self {
        if self < Self::zero() {
            Self::zero() - self
        } else {
            self
        }
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

impl Float for f32 {
    fn as_f32(self) -> f32 {
        self
    }

    fn copysign(self, sign: Self) -> Self {
        self.copysign(sign)
    }
}

impl Float for f64 {
    fn as_f32(self) -> f32 {
        self as f32
    }

    fn copysign(self, sign: Self) -> Self {
        self.copysign(sign)
    }
}
