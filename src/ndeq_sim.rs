//! Provider of [`NdeqSim`].

use crate::ode::values::{Time, Value};
use crate::prelude::*;
use std::ops::Mul;

/// Network diffusion simulator.
pub struct NdeqSim<V, T> {
    /// ODE solver.
    solver: Box<dyn OdeSolver<V, T>>,

    /// Target network.
    net: NdeqNet<V>,
}

impl<V, T> NdeqSim<V, T>
where
    V: Value + Mul<T, Output = V>,
    T: Time,
{
    /// Creates a new instance with specified ODE solver.
    pub fn new(solver: Box<dyn OdeSolver<V, T>>) -> Self {
        Self {
            solver,
            net: Default::default(),
        }
    }

    /// Returns target network.
    pub fn net(&self) -> &NdeqNet<V> {
        &self.net
    }

    /// Sets target network.
    pub fn set_net(&mut self, value: NdeqNet<V>) {
        self.net = value;
        self.solver.set_yp(self.net.yp());
    }

    /// Advance time and update target network node values.
    pub fn run(&mut self, p: T) {
        self.solver.run(self.net.values_mut(), p);
    }
}
