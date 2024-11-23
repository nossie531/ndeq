//! Provider of [`ExplicitEuler`].

use crate::prelude::*;
use crate::util::sum_values;
use crate::values::{Time, Value};
use std::marker::PhantomData;
use std::ops::Mul;
use std::rc::Rc;

/// Explicit Euler method.
#[derive(Default)]
pub struct ExplicitEuler<V, T> {
    /// Calculated node values.
    values: Vec<V>,

    /// Dummy.
    pd: PhantomData<T>,
}

impl<V, T> ExplicitEuler<V, T>
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

impl<V, T> StepAlgorithm<V, T> for ExplicitEuler<V, T>
where
    V: Value + Mul<T, Output = V>,
    T: Time,
{
    fn values(&self) -> &[V] {
        self.values.as_slice()
    }

    fn step(&mut self, nodes: &[Rc<dyn NdeqNode<V>>], width: T) {
        assert!(width.is_num());

        self.values.clear();

        for node in nodes.iter() {
            let value = node.value();
            let flows = node.in_edges().map(|(n, w)| (n.value() - value) * w);
            let slope = sum_values(flows);
            self.values.push(value + slope * width);
        }
    }
}
