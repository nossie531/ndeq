//! Provider of [`NdeqSim`].

use crate::ode::values::{Time, Value};
use crate::prelude::*;
use std::ops::Mul;

/// Network diffusion simulator.
pub struct NdeqSim<'a, V, T> {
    /// ODE solver.
    solver: Box<dyn OdeSolver<V, T> + 'a>,

    /// Target network.
    net: &'a dyn NetView<V>,

    values: Vec<V>,
}

impl<'a, V, T> NdeqSim<'a, V, T>
where
    V: Value + Mul<T, Output = V>,
    T: Time,
{
    /// Creates a new instance with specified ODE solver.
    pub fn new(solver: Box<dyn OdeSolver<V, T> + 'a>, net: &'a dyn NetView<V>) -> Self {
        let values = Default::default();
        Self {
            solver,
            net,
            values,
        }
    }

    /// Returns target network.
    pub fn net<'s: 'a>(&'s self) -> &'a dyn NetView<V> {
        self.net
    }

    /// Advance time and update target network node values.
    pub fn run(&mut self, p: T) {
        self.net.load_values(&mut self.values);
        self.solver.run(&mut self.values, p);
        self.net.set_values(&self.values);
    }
}
