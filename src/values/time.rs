//! Provider of [`Time`].

use std::{cmp::Ordering, ops::{Add, Div, Mul, Sub}};

/// Time for diffusion.
pub trait Time:
    'static
    + Copy
    + Default
    + PartialEq
    + PartialOrd
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<f32, Output = Self>
    + Div<f32, Output = Self>
{
    /// Returns zero value.
    fn zero() -> Self {
        Self::default()
    }

    /// Returns `true` if `self` is NaN.
    fn is_num(&self) -> bool {
        self.eq(self)
    }

    /// Returns `true` if this number is neither infinite nor NaN.
    fn is_finite(&self) -> bool {
        self.is_num() && *self != (*self / 2.0)
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
        + PartialEq
        + PartialOrd
        + Add<Self, Output = Self>
        + Sub<Self, Output = Self>
        + Mul<f32, Output = Self>
        + Div<f32, Output = Self>,
{
    // nop.
}
