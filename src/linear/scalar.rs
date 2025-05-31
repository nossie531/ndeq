//! Provider of [`Scalar`].

use std::ops::{Add, AddAssign, Mul, MulAssign};

/// [Scalar] value.
///
/// [Scalar]: https://en.wikipedia.org/wiki/Scalar_(mathematics)
pub trait Scalar:
    'static
    + Copy
    + Default
    + PartialEq
    + Mul<Output = Self>
    + Add<Output = Self>
    + MulAssign
    + AddAssign
{
    // nop.
}
