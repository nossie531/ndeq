//! Provider of [`NetEuler`].

use crate::net_ode::solver::UnivNetOdeSolver;
use crate::prelude::*;
use ndeq_ode::prelude::*;
use ndeq_ode::values::{OdeTime, OdeValue};
use std::ops::MulAssign;

/// ODE solver for network with Euler method.
pub struct NetEuler<'a, T, V> {
    adapter: UnivNetOdeSolver<'a, T, V>,
}

impl<'a, T, V> NetEuler<'a, T, V>
where
    T: OdeTime,
    V: OdeValue + MulAssign<T>,
{
    /// Creates a new value.
    pub fn new(h: T) -> Self {
        let base = Box::new(Euler::new(h));
        let adapter = UnivNetOdeSolver::new(base);
        Self { adapter }
    }
}

impl<'a, T, V> NetOdeSolver<'a, T, V> for NetEuler<'a, T, V>
where
    T: OdeTime,
    V: OdeValue + MulAssign<T>,
{
    fn new_values(&self) -> &[V] {
        self.adapter.new_values()
    }

    fn set_flow(&mut self, value: &'a dyn NdeqFlow<V>) {
        self.adapter.set_flow(value);
    }

    fn run(&mut self, t: T) {
        self.adapter.run(t);
    }
}
