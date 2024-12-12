//! Provider of [`NdeqSim`].

use crate::net_parts::NdeqNet;
use crate::prelude::*;
use crate::values::{Time, Value};
use std::ops::Mul;

/// Network diffusion simulator.
pub struct NdeqSim<V, T> {
    /// Diffusion algorithm.
    diffuser: Box<dyn Diffuser<V, T>>,

    /// Target network.
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

    /// Advance target network status for specified time.
    pub fn run(&mut self, p: T) {
        self.diffuser.run(&mut self.net, p);
    }
}
