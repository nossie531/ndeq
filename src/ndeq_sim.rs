//! Provider of [`NdeqSim`].

use crate::ode::solver::OdeSolver;
use crate::ode::values::{Time, VArr, Value};
use crate::prelude::*;
use std::ops::MulAssign;

/// Network diffusion simulator.
pub struct NdeqSim<'a, T, V> {
    /// Network.
    net: &'a dyn NdeqNet<V>,

    /// ODE solver.
    solver: Box<dyn OdeSolver<'a, T, VArr<V>> + 'a>,

    /// Network node values.
    values: VArr<V>,
}

impl<'a, T, V> NdeqSim<'a, T, V>
where
    T: Time,
    V: Value + MulAssign<T>,
{
    /// Creates a new instance.
    pub fn new(
        net: &'a dyn NdeqNet<V>,
        mut solver: Box<dyn OdeSolver<'a, T, VArr<V>> + 'a>,
    ) -> Self {
        solver.set_slope(net.slope());
        Self {
            net,
            solver,
            values: Default::default(),
        }
    }

    /// Returns target network.
    pub fn net<'s: 'a>(&'s self) -> &'a dyn NdeqNet<V> {
        self.net
    }

    /// Update target network node values to future values.
    pub fn run(&mut self, t: T) {
        self.net.export_values(self.values.as_mut());
        self.solver.set_value(&self.values);
        self.solver.run(t);
        self.net.import_values(self.solver.new_value().as_ref());
    }
}
