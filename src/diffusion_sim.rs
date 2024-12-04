//! Provider of [`DiffusionSim`].

use crate::prelude::*;
use crate::values::{Time, Value};
use std::ops::Mul;

/// Diffusion simulator.
#[must_use]
pub trait DiffusionSim<V, T>
where
    V: Value + Mul<T, Output = V>,
    T: Time,
{
    /// Calculate node values after specified time.
    ///
    /// # Panics
    ///
    /// Panics if any of the following occur.
    ///
    /// * `p` is NaN.
    /// * Node values are borrowed.
    fn run(&mut self, nodes: &[&dyn NdeqNode<V>], p: T);
}
