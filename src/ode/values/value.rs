//! Provider of [`Value`].

use crate::ode::values::RF32;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

/// Value (function value of ODE system).
pub trait Value:
    'static
    + Clone
    + Default
    + PartialEq
    + MulAssign<RF32>
    + DivAssign<RF32>
    + for<'a> AddAssign<&'a Self>
    + for<'a> SubAssign<&'a Self>
{
    /// Copys dimension and fill this value with zero.
    fn clone_zero(&mut self, x: &Self) {
        self.clone_from(x);
        self.fill_zero();
    }

    /// Fills this value with zero.
    fn fill_zero(&mut self) {
        *self *= RF32(0.0);
    }
}

impl<T> Value for T
where
    T: 'static
        + Clone
        + Default
        + PartialEq
        + MulAssign<RF32>
        + DivAssign<RF32>
        + for<'a> AddAssign<&'a Self>
        + for<'a> SubAssign<&'a Self>,
{
    // nop.
}
