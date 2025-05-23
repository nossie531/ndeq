//! Provider of [`NetOdeSolver`].

use crate::ode::solver::OdeSolver;
use crate::ode::values::VArr;
use crate::parts::NdeqNet;

/// ODE solver for network.
pub trait NetOdeSolver<T, V> {
    /// Creates ODE solver.
    fn create<'a>(&self, net: &'a dyn NdeqNet<V>) -> Box<dyn OdeSolver<'a, T, VArr<V>> + 'a>;
}
