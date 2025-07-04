//! Provider of [`Scalar`].

use dyn_compatible::prelude::*;
use ndeq_num::prelude::*;
use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Mul, MulAssign};

/// [Scalar] value.
///
/// [Scalar]: https://en.wikipedia.org/wiki/Scalar_(mathematics)
#[dyn_compatible(false)]
pub trait Scalar:
    'static
    + Copy
    + Debug
    + Display
    + PartialEq
    + From<f32>
    + From<Self::Real>
    + Mul<Output = Self>
    + Add<Output = Self>
    + MulAssign
    + AddAssign
{
    /// Absolute value type.
    type Real: Float;

    /// Returns zero.
    fn zero() -> &'static Self;

    /// Returns one.
    fn one() -> &'static Self;

    /// Returns absolute value.
    fn abs(&self) -> Self::Real;
}

impl Scalar for f32 {
    type Real = Self;

    fn zero() -> &'static Self {
        &0.0
    }

    fn one() -> &'static Self {
        &1.0
    }

    fn abs(&self) -> Self::Real {
        f32::abs(*self)
    }
}

impl Scalar for f64 {
    type Real = Self;

    fn zero() -> &'static Self {
        &0.0
    }

    fn one() -> &'static Self {
        &1.0
    }

    fn abs(&self) -> Self::Real {
        f64::abs(*self)
    }
}
