//! Provider of [`StepAlgorithm`].

use crate::prelude::*;
use crate::values::{Time, Value};
use std::ops::Mul;

/// Step algorithm.
#[must_use]
pub trait StepAlgorithm<V, T>
where
    V: Value + Mul<T, Output = V>,
    T: Time,
{
    /// Returns calculated node values.
    ///
    /// If no calculation is performed, the result is empty.
    #[must_use]
    fn values(&self) -> &[V];

    /// Calculate node values after small time.
    ///
    /// # Panics
    ///
    /// Panics if any of the following occur.
    ///
    /// * `width` is NaN.
    /// * Calculation target nodes values are borrowed.
    fn step(&mut self, nodes: &[&dyn NdeqNode<V>], width: T);
}
