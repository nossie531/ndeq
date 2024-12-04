//! Provider of [`ExplicitEuler`].

use crate::prelude::*;
use crate::values::{Time, Value};
use std::marker::PhantomData;
use std::ops::Mul;

/// Diffusion calc approach with [Euler methods].
///
/// [Euler methods]: https://en.wikipedia.org/wiki/Euler_method
pub struct Euler<V, T> {
    /// Step size.
    h: T,

    /// Calculated node values.
    values: Vec<V>,

    /// Dummy.
    pd: PhantomData<T>,
}

impl<V, T> Euler<V, T>
where
    V: Value + Mul<T, Output = V>,
    T: Time,
{
    /// Create new value.
    /// 
    /// # Panics
    /// 
    /// Panics if `h` is zero or NaN or infinity.
    #[must_use]
    pub fn new(h: T) -> Box<Self> {
        assert!(h.is_num());
        assert!(h.is_finite());
        assert!(h != T::zero());
        Box::new(Self {
            h,
            values: vec![],
            pd: Default::default()
        })
    }

    /// Calculate node values after small time.
    ///
    /// # Panics
    ///
    /// Panics if node values are borrowed.
    fn step(&mut self, nodes: &[&dyn NdeqNode<V>], h: T) {
        self.values.clear();

        for node in nodes.iter() {
            let value = node.value();
            let flows = node.edges().map(|(v, w)| (v - value) * w);
            let slope = V::sum(flows);
            self.values.push(value + slope * h);
        }
    }

    /// Update node values by simulation results.
    ///
    /// # Panics
    ///
    /// Panics if calculation target nodes values are borrowed.
    fn update_nodes(&self, nodes: &[&dyn NdeqNode<V>]) {
        for i in 0..nodes.len() {
            nodes[i].set_value(self.values[i]);
        }
    }
}

impl<V, T> DiffusionSim<V, T> for Euler<V, T>
where
    V: Value + Mul<T, Output = V>,
    T: Time,
{
    fn run(&mut self, nodes: &[&dyn NdeqNode<V>], p: T) {
        assert!(p.is_num());

        let mut t = T::zero();
        while t < p {
            let h = (p - t).min(self.h).unwrap_or(self.h);

            self.step(nodes, h);
            self.update_nodes(nodes);

            t = t + h;
        }
    }
}
