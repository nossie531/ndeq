//! Provider of [`OdeSolver`].

use crate::ode::values::{Time, Value};
use crate::ode::Yp;
use std::ops::MulAssign;

/// ODE solver.
#[must_use]
pub trait OdeSolver<T, V>
where
    T: Time,
    V: Value + MulAssign<T>,
{
    /// Initialize value dimension.
    fn init_dim(&mut self, value: &V) {
        let _ = value;
    }

    /// Update value to future value.
    ///
    /// `t` can be negative if algorithm supports it.
    ///
    /// # Panics
    ///
    /// Panics if `t` is NaN or infinity or negative
    /// (if algorithm not supports negative values).
    fn run(&mut self, value: &mut V, yp: &Yp<V>, t: T);
}
