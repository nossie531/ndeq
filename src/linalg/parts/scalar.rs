//! Provider of [`Scalar`].

use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Mul, MulAssign};

/// [Scalar] value.
///
/// [Scalar]: https://en.wikipedia.org/wiki/Scalar_(mathematics)
pub trait Scalar:
    'static
    + Copy
    + Debug
    + Default
    + Display
    + PartialEq
    + Mul<Output = Self>
    + Add<Output = Self>
    + MulAssign
    + AddAssign
{
    // nop.
}

impl<T> Scalar for T
where
    T: 'static
        + Copy
        + Debug
        + Default
        + Display
        + PartialEq
        + Mul<Output = Self>
        + Add<Output = Self>
        + MulAssign
        + AddAssign,
{
    // nop.
}
