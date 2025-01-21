//! Provider of [`Value`].

use std::ops::{AddAssign, MulAssign, SubAssign};

/// Value (function value of ODE system).
pub trait Value:
    'static
    + Clone
    + Default
    + PartialEq
    + MulAssign<f32>
    + for<'a> AddAssign<&'a Self>
    + for<'a> SubAssign<&'a Self>
{
    /// Copys dimension and fill this value with zero.
    fn init_dim(&mut self, x: &Self) {
        *self = x.clone();
        self.fill_zero();
    }

    /// Fills this value with zero.
    fn fill_zero(&mut self) {
        *self *= 0.0;
    }
}

impl<T> Value for T
where
    T: 'static
        + Clone
        + Default
        + PartialEq
        + MulAssign<f32>
        + for<'a> AddAssign<&'a Self>
        + for<'a> SubAssign<&'a Self>,
{
    // nop.
}
