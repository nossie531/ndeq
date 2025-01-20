//! Provider of [`Value`].

use std::ops::{Add, Mul, Sub};

/// Value (function value of ODE system).
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
