//! Provider of [`OdeSolver`].

use crate::ode::values::{Time, Value};
use std::ops::MulAssign;

/// ODE solver.
#[must_use]
pub trait OdeSolver<'a, T, V>
where
    T: Time,
    V: Value + MulAssign<T>,
{
    /// Returns new value of this instance.
    fn new_value(&self) -> &V;

    /// Sets value of this instance.
    fn set_value(&mut self, value: &V);

    /// Update value to future value.
    ///
    /// `t` can be negative if algorithm supports it.
    ///
    /// # Panics
    ///
    /// Panics if `t` is NaN or infinity or negative
    /// (if algorithm not supports negative values).
    fn run(&mut self, t: T);
}
