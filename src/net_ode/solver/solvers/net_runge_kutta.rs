//! Provider of [`NetRungeKutta`].

use crate::net_ode::solver::NetOdeSolver;
use crate::ode::solver::solvers::RungeKutta;
use crate::ode::solver::{GpOdeSolver, OdeSolver};
use crate::ode::values::{Time, VArr, Value};
use crate::parts::NdeqNet;
use std::marker::PhantomData;
use std::ops::MulAssign;

/// Runge-Kutta ODE solver for network.
pub struct NetRungeKutta<T, V> {
    h: T,
    pd: PhantomData<V>,
}

impl<T, V> NetRungeKutta<T, V> {
    /// Creates a new instance.
    pub fn new(h: T) -> Self {
        Self {
            h,
            pd: Default::default(),
        }
    }
}

impl<T, V> NetOdeSolver<T, V> for NetRungeKutta<T, V>
where
    T: Time,
    V: Value + MulAssign<T>,
{
    fn create<'a>(&self, net: &'a dyn NdeqNet<V>) -> Box<dyn OdeSolver<'a, T, VArr<V>> + 'a> {
        let mut ret = RungeKutta::new(self.h);
        ret.set_slope(net.slope());
        ret
    }
}
