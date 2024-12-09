use crate::net_parts::NdeqNet;
use crate::prelude::*;
use crate::values::{Time, Value};
use std::ops::Mul;

/// Network diffusion simulator.
pub struct NdeqSim<V, T> {
    diffuser: Box<dyn Diffuser<V, T>>,
    net: NdeqNet<V>,
}

impl<V, T> NdeqSim<V, T>
where
    V: Value + Mul<T, Output = V>,
    T: Time,
{
    /// Create a new simulation with specified diffusion alogorithm.
    pub fn new(diffuser: Box<dyn Diffuser<V, T>>) -> Self {
        Self {
            diffuser,
            net: Default::default(),
        }
    }

    /// Returns target network.
    pub fn net(&self) -> &NdeqNet<V> {
        &self.net
    }

    /// Set target network.
    pub fn set_net(&mut self, value: NdeqNet<V>) {
        self.net = value;
    }

    /// Calc simulation until specified time.
    pub fn calc(&mut self, p: T) {
        self.diffuser.calc(&mut self.net, p);
    }
}
