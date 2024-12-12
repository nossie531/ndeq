//! Provider of [`Value`].

use std::ops::{Add, Mul, Sub};

/// Value (output component of ODE system).
pub trait Value:
    'static
    + Copy
    + Default
    + PartialEq
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<f32, Output = Self>
{
    /// Returns zero value.
    fn zero() -> Self {
        Self::default()
    }

    /// Sums the elements of an iterator.
    fn sum<V: Value>(values: impl Iterator<Item = V>) -> V {
        let mut ret = V::zero();
        for value in values {
            ret = ret + value
        }

        ret
    }
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
