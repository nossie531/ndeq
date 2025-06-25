//! Provider of [`OdeSolver`].

use crate::values::{OdeTime, OdeValue};
use dyn_compatible::prelude::*;
use std::ops::MulAssign;

/// ODE solver.
#[must_use]
#[dyn_compatible(true)]
pub trait OdeSolver<T, V>
where
    T: OdeTime,
    V: OdeValue + MulAssign<T>,
{
    /// Returns new value.
    fn new_value(&self) -> &V;

    /// Sets value.
    fn set_value(&mut self, value: &V);

    /// Calculate new values.
    ///
    /// `t` can be negative if algorithm supports it.
    ///
    /// # Panics
    ///
    /// Panics if `t` is NaN or infinity or negative
    /// (if algorithm not supports negative values).
    fn run(&mut self, t: T);
}
