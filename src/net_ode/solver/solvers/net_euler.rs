//! Provider of [`NetEuler`].

use crate::net_ode::solver::{NetOdeSolver, UnivNetOdeSolver};
use crate::parts::NdeqNet;
use ndeq_ode::solver::solvers::Euler;
use ndeq_ode::values::{Time, Value};
use std::ops::MulAssign;

/// ODE solver for network with Euler method.
pub struct NetEuler<'a, T, V> {
    adapter: UnivNetOdeSolver<'a, T, V>
}

impl<'a, T, V> NetEuler<'a, T, V>
where
    T: Time,
    V: Value + MulAssign<T>,
{
    /// Creates a new instance.
    pub fn new(h: T) -> Self {
        let base = Box::new(Euler::new(h));
        let adapter = UnivNetOdeSolver::new(base);
        Self { adapter }
    }
}

impl<'a, T, V> NetOdeSolver<'a, T, V> for NetEuler<'a, T, V>
where
    T: Time,
    V: Value + MulAssign<T>,
{
    fn new_values(&self) -> &[V] {
        self.adapter.new_values()
    }
    
    fn set_net(&mut self, net: &'a dyn NdeqNet<V>) {
        self.adapter.set_net(net);
    }
    
    fn run(&mut self, t: T) {
        self.adapter.run(t);
    }
}
