//! Provider of [`NetRungeKutta`].

use crate::NodeVal;
use crate::prelude::*;
use crate::solver::UnivNetOdeSolver;
use ndeq_ode::prelude::*;
use ndeq_ode::values::OdeTime;

/// ODE solver for network with Runge-Kutta method.
pub struct NetRungeKutta<'a, T, V> {
    adapter: UnivNetOdeSolver<'a, T, V>,
}

impl<'a, T, V> NetRungeKutta<'a, T, V>
where
    T: OdeTime,
    V: NodeVal<T>,
{
    /// Creates a new value.
    pub fn new(h: T) -> Self {
        let base = Box::new(RungeKutta::new(h));
        let adapter = UnivNetOdeSolver::new(base);
        Self { adapter }
    }
}

impl<'a, T, V> NetOdeSolver<'a, T, V> for NetRungeKutta<'a, T, V>
where
    T: OdeTime,
    V: NodeVal<T>,
{
    fn new_values(&self) -> &[V] {
        self.adapter.new_values()
    }

    fn set_flow(&mut self, value: &'a dyn NetFlow<V>) {
        self.adapter.set_flow(value);
    }

    fn run(&mut self, t: T) {
        self.adapter.run(t);
    }
}
