//! Provider of [`NdeqSim`].

use crate::ode::solver::OdeSolver;
use crate::ode::values::{Time, VArr, Value};
use crate::prelude::*;
use std::ops::MulAssign;

/// Network diffusion simulator.
pub struct NdeqSim<'a, T, V> {
    /// ODE solver.
    solver: Box<dyn OdeSolver<T, VArr<V>> + 'a>,

    /// Network.
    net: &'a dyn NetView<V>,

    /// Network node values.
    values: VArr<V>,
}

impl<'a, T, V> NdeqSim<'a, T, V>
where
    T: Time,
    V: Value + MulAssign<T>,
{
    /// Creates a new instance.
    pub fn new(solver: Box<dyn OdeSolver<T, VArr<V>> + 'a>, net: &'a dyn NetView<V>) -> Self {
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

    /// Update target network node values to future values.
    pub fn run(&mut self, p: T) {
        self.net.load_values(self.values.as_mut());
        self.solver.run(&mut self.values, p);
        self.net.set_values(self.values.as_ref());
    }
}
