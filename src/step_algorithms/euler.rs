//! Provider of [`ExplicitEuler`].

use crate::prelude::*;
use crate::util::sum_values;
use crate::values::{Time, Value};
use std::marker::PhantomData;
use std::ops::Mul;

/// Diffusion calc approach with [Euler methods].
///
/// [Euler methods]: https://en.wikipedia.org/wiki/Euler_method
#[derive(Default)]
pub struct Euler<V, T> {
    /// Calculated node values.
    values: Vec<V>,

    /// Dummy.
    pd: PhantomData<T>,
}

impl<V, T> Euler<V, T>
where
    V: Value,
    T: Time,
{
    /// Create new value.
    #[must_use]
    pub fn new() -> Box<Self> {
        Box::default()
    }
}

impl<V, T> StepAlgorithm<V, T> for Euler<V, T>
where
    V: Value + Mul<T, Output = V>,
    T: Time,
{
    fn values(&self) -> &[V] {
        self.values.as_slice()
    }

    fn step(&mut self, nodes: &[&dyn NdeqNode<V>], width: T) {
        assert!(width.is_num());

        self.values.clear();

        for node in nodes.iter() {
            let value = node.value();
            let flows = node.edges().map(|(v, w)| (v - value) * w);
            let slope = sum_values(flows);
            self.values.push(value + slope * width);
        }
    }
}
