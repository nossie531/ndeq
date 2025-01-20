//! Provider of [`OdeSolver`].

use crate::ode::values::{Time, Value};
use std::ops::Mul;

/// ODE solver.
#[must_use]
pub trait OdeSolver<V, T>
where
    V: Value + Mul<T, Output = V>,
    T: Time,
{
    /// Initialize with the number of network nodes.
    fn init(&mut self, len: usize);

    /// Update values to future values.
    ///
    /// `p` can be negative if algorithm supports it.
    ///
    /// # Panics
    ///
    /// Panics if `p` is NaN or infinity or negative
    /// (if algorithm not supports negative values).
    fn run(&mut self, values: &mut [V], p: T);
}
