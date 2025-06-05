//! Provider of [`Scalar`].

use super::real::Real;
use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Mul, MulAssign};

/// [Scalar] value.
///
/// [Scalar]: https://en.wikipedia.org/wiki/Scalar_(mathematics)
pub trait Scalar:
    'static
    + Copy
    + Debug
    + Display
    + PartialEq
    + From<f32>
    + Mul<Output = Self>
    + Add<Output = Self>
    + MulAssign
    + AddAssign
{
    type Real: Real;

    /// Returns zero.
    fn zero() -> Self {
        0.0.into()
    }

    /// Returns one.
    fn one() -> Self {
        1.0.into()
    }

    /// Returns absolute value.
    fn abs(&self) -> Self::Real;
}

impl Scalar for f32 {
    type Real = Self;

    fn abs(&self) -> Self::Real {
        f32::abs(*self)
    }
}

impl Scalar for f64 {
    type Real = Self;

    fn abs(&self) -> Self::Real {
        f64::abs(*self)
    }
}
