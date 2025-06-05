//! Provider of [`Real`].

use std::ops::Add;

/// Real number.
pub trait Real: Copy + PartialOrd + From<f32> + Add {
    /// Returns zero.
    fn zero() -> Self {
        0.0.into()
    }

    /// Returns one.
    fn one() -> Self {
        1.0.into()
    }
}

impl<T> Real for T
where
    T: Copy + PartialOrd + From<f32> + Add,
{
    // nop.
}
