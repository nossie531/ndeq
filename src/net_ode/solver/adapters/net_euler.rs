//! Provider of [`NetEuler`].

use crate::net_ode::solver::NetOdeSolver;
use crate::ode::solver::solvers::Euler;
use crate::ode::solver::{GpOdeSolver, OdeSolver};
use crate::ode::values::{Time, VArr, Value};
use crate::parts::NdeqNet;
use std::marker::PhantomData;
use std::ops::MulAssign;

/// ODE solver for network with Euler method.
pub struct NetEuler<T, V> {
    h: T,
    pd: PhantomData<V>,
}

impl<T, V> NetEuler<T, V> {
    /// Creates a new instance.
    pub fn new(h: T) -> Self {
        Self {
            h,
            pd: Default::default(),
        }
    }
}

impl<T, V> NetOdeSolver<T, V> for NetEuler<T, V>
where
    T: Time,
    V: Value + MulAssign<T>,
{
    fn create<'a>(&self, net: &'a dyn NdeqNet<V>) -> Box<dyn OdeSolver<'a, T, VArr<V>> + 'a> {
        let mut ret = Euler::new(self.h);
        ret.set_slope(net.slope());
        ret
    }
}
