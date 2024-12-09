//! Provider of [`Diffuser`].

use crate::net_parts::NdeqNet;
use crate::values::{Time, Value};
use std::ops::Mul;

/// Diffusion algorithm.
#[must_use]
pub trait Diffuser<V, T>
where
    V: Value + Mul<T, Output = V>,
    T: Time,
{
    /// Calculate node values after specified time.
    ///
    /// `p` can be negative if algorithm supports it.
    ///
    /// # Panics
    ///
    /// Panics if `p` is NaN or infinity or negative
    /// (if algorithm not supports negative values).
    fn calc(&mut self, net: &mut NdeqNet<V>, p: T);
}
