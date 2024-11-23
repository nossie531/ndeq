//! Provider of [`NdeqRunner`].

use crate::prelude::*;
use crate::values::{Time, Value};
use std::ops::Mul;
use std::rc::Rc;

/// Network diffusion runner.
pub struct NdeqRunner<V, T> {
    /// Step algorithm.
    algorithm: Box<dyn StepAlgorithm<V, T>>,

    /// Target nodes.
    nodes: Vec<Rc<dyn NdeqNode<V>>>,
}

impl<V, T> NdeqRunner<V, T>
where
    V: Value + Mul<T, Output = V>,
    T: Time,
{
    /// Create new value.
    pub fn new(nodes: impl IntoIterator<Item = Rc<dyn NdeqNode<V>>>) -> Self {
        let algorithm = ExplicitEuler::new();
        let nodes = nodes.into_iter().collect::<Vec<_>>();
        Self { algorithm, nodes }
    }

    /// Set algorithm.
    pub fn set_algorithm(&mut self, value: Box<dyn StepAlgorithm<V, T>>) {
        self.algorithm = value;
    }

    /// Returns target nodes.
    #[must_use]
    pub fn nodes(&self) -> &[Rc<dyn NdeqNode<V>>] {
        self.nodes.as_slice()
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
    pub fn calc(&mut self, width: T) {
        assert!(width.is_num());
        self.algorithm.step(self.nodes.as_slice(), width);
    }

    /// Run simulation.
    ///
    /// # Panics
    ///
    /// Panics if any of the following occur.
    ///
    /// * `width` is NaN.
    /// * Calculation target nodes values are borrowed.
    pub fn run(&mut self, width: T) {
        assert!(width.is_num());
        self.calc(width);
        self.update_values();
    }

    /// Update node values by simulation results.
    ///
    /// # Panics
    ///
    /// Panics if calculation target nodes values are borrowed.
    fn update_values(&self) {
        let nodes = self.nodes();
        let values = self.values();
        for i in 0..nodes.len() {
            nodes[i].set_value(values[i]);
        }
    }
}
