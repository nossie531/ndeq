//! Provider of [`Time`].

use std::ops::{Add, Div, Mul, Sub};

/// Time for diffusion.
pub trait Time:
    'static
    + Copy
    + Default
    + PartialEq
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<f32, Output = Self>
    + Div<f32, Output = Self>
{
    /// Returns `true` if `self` is NaN.
    fn is_num(&self) -> bool {
        #[allow(clippy::eq_op)]
        {
            self == self
        }
    }
}

impl<T> Time for T
where
    T: 'static
        + Copy
        + Default
        + PartialEq
        + Add<Self, Output = Self>
        + Sub<Self, Output = Self>
        + Mul<f32, Output = Self>
        + Div<f32, Output = Self>,
{
    // nop.
}
