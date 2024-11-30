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
    /// Sums the elements of an iterator.
    fn sum<V: Value>(values: impl Iterator<Item = V>) -> V {
        let mut ret = V::default();
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
