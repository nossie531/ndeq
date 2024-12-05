use crate::net_parts::Net;
use crate::prelude::*;
use crate::values::{Time, Value};
use std::ops::Mul;

/// Network diffusion simulator.
pub struct NdeqSim<'a, V, T> {
    diffuser: Box<dyn Diffuser<V, T>>,
    net: Net<'a, V>,
}

impl<'a, V, T> NdeqSim<'a, V, T>
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
    pub fn net(&self) -> &Net<'a, V> {
        &self.net
    }

    /// Set target nodes.
    pub fn set_nodes<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'a dyn NdeqNode<V>>,
    {
        self.net = Net::from_nodes(iter);
    }

    /// Calc simulation after specified time.
    ///
    /// Unlike [`run`](Self::run) method, this method does not update
    /// nodes values that are set by [`set_nodes`](Self::set_nodes).
    /// Therefore, values must be checked from [`net`](Self::net).
    pub fn calc(&mut self, p: T) {
        self.diffuser.calc(&mut self.net, p);
    }

    /// Run simulation after specified time.
    ///
    /// Unlike [`calc`](Self::calc) method, this method actually updates
    /// node values that are set by [`set_nodes`](Self::set_nodes).
    pub fn run(&mut self, p: T) {
        self.calc(p);
        self.net.update_originals();
    }
}
