//! Provider of [`Time`].

use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

use super::FloatApprox;

/// Time (input value of ODE system).
pub trait Time:
    'static
    + Copy
    + Default
    + PartialOrd
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<f32, Output = Self>
    + Div<f32, Output = Self>
    + FloatApprox
{
    /// Returns zero value.
    fn zero() -> Self {
        Self::default()
    }

    /// Returns `true` if this value is NaN.
    fn is_nan(self) -> bool {
        self.approx_into_float().is_nan()
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
        self.approx_into_float().is_infinite()
    }

    /// Returns this number with the sign equal to `sign`.
    fn copysign(self, sign: Self) -> Self {
        let this = self.approx_into_float();
        let sign = sign.approx_into_float();
        Self::approx_from_float(this.copysign(sign))
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
        + FloatApprox,
{
    // nop.
}
