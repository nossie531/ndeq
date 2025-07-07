//! Provider of [`OdeValue`].

use dyn_compatible::prelude::*;
use std::ops::{AddAssign, MulAssign, SubAssign};

/// ODE value.
///
/// This value can be a vector as well as a scalar.
#[dyn_compatible(false)]
pub trait OdeValue:
    'static
    + Clone
    + Default
    + PartialEq
    + MulAssign<f32>
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
        *self *= 0.0;
    }
}

impl<T> OdeValue for T
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
