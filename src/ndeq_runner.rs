//! Provider of [`NdeqRunner`].

use crate::prelude::*;
use crate::values::{Time, Value};
use std::ops::Mul;

/// Network diffusion runner.
pub struct NdeqRunner<V, T> {
    /// Step algorithm.
    algorithm: Box<dyn StepAlgorithm<V, T>>,
}

impl<V, T> NdeqRunner<V, T>
where
    V: Value + Mul<T, Output = V>,
    T: Time,
{
    /// Create new value.
    pub fn new() -> Self {
        Self {
            algorithm: Euler::new(),
        }
    }

    /// Set algorithm.
    pub fn set_algorithm(&mut self, value: Box<dyn StepAlgorithm<V, T>>) {
        self.algorithm = value;
    }

    /// Returns calculated node values.
    ///
    /// If no calculation is performed, the result is empty.
    #[must_use]
    pub fn values(&self) -> &[V] {
        self.algorithm.values()
    }

    /// Calculate node values after small time.
    ///
    /// This method is not update nodes values. Only [`values`](Self::values)
    /// are updated. On the other hand, [`run`](Self::run) method changes the
    /// actual values of the nodes.
    ///
    /// # Panics
    ///
    /// Panics if any of the following occur.
    ///
    /// * `width` is NaN.
    /// * Calculation target nodes values are mutably borrowed.
    pub fn calc(&mut self, nodes: &[&dyn NdeqNode<V>], width: T) {
        assert!(width.is_num());
        self.algorithm.step(nodes, width);
    }

    /// Run simulation.
    ///
    /// # Panics
    ///
    /// Panics if any of the following occur.
    ///
    /// * `width` is NaN.
    /// * Calculation target nodes values are borrowed.
    pub fn run(&mut self, nodes: &[&dyn NdeqNode<V>], width: T) {
        assert!(width.is_num());
        self.calc(nodes, width);
        self.update_values(nodes);
    }

    /// Update node values by simulation results.
    ///
    /// # Panics
    ///
    /// Panics if calculation target nodes values are borrowed.
    fn update_values(&self, nodes: &[&dyn NdeqNode<V>]) {
        let values = self.values();
        for i in 0..nodes.len() {
            nodes[i].set_value(values[i]);
        }
    }
}

impl<V, T> Default for NdeqRunner<V, T>
where
    V: Value + Mul<T, Output = V>,
    T: Time,
{
    fn default() -> Self {
        Self::new()
    }
}
