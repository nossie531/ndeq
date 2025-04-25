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
    solver: Box<dyn OdeSolver<T, VArr<V>> + 'a>,

    /// Network node values.
    values: VArr<V>,
}

impl<'a, T, V> NdeqSim<'a, T, V>
where
    T: Time,
    V: Value + MulAssign<T>,
{
    /// Creates a new instance.
    pub fn new(net: &'a dyn NdeqNet<V>, solver: Box<dyn OdeSolver<T, VArr<V>> + 'a>) -> Self {
        let values = Default::default();
        Self {
            net,
            solver,
            values,
        }
    }

    /// Returns target network.
    pub fn net<'s: 'a>(&'s self) -> &'a dyn NdeqNet<V> {
        self.net
    }

    /// Update target network node values to future values.
    pub fn run(&mut self, t: T) {
        self.net.export_values(self.values.as_mut());
        self.solver.run(&mut self.values, &self.net.yp(), t);
        self.net.import_values(self.values.as_ref());
    }
}
