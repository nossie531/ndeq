//! Provider of [`Value`].

use std::ops::{Add, Mul, Sub};

/// Node value.
pub trait Value:
    'static
    + Copy
    + Default
    + PartialEq
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<f32, Output = Self>
{
    // nop.
}

impl<T> Value for T
where
    T: 'static
        + Copy
        + Default
        + PartialEq
        + Add<Self, Output = Self>
        + Sub<Self, Output = Self>
        + Mul<f32, Output = Self>,
{
    // nop.
}
