//! Provider of [`OdeSolver`].

use crate::ode::values::{Time, Value};
use std::ops::MulAssign;

/// ODE solver.
#[must_use]
pub trait OdeSolver<T, V>
where
    T: Time,
    V: Value + MulAssign<T>,
{
    /// Initialize value dimension.
    fn init_dim(&mut self, value: &V);

    /// Update value to future value.
    ///
    /// `p` can be negative if algorithm supports it.
    ///
    /// # Panics
    ///
    /// Panics if `p` is NaN or infinity or negative
    /// (if algorithm not supports negative values).
    fn run(&mut self, value: &mut V, p: T);
}
