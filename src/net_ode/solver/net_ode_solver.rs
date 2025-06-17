//! Provider of [`NetOdeSolver`].

use crate::parts::NdeqNet;
use ndeq_ode::solver::OdeSolver;
use ndeq_ode::values::VArr;

/// ODE solver for network.
pub trait NetOdeSolver<T, V> {
    /// Creates ODE solver.
    fn create<'a>(&self, net: &'a dyn NdeqNet<V>) -> Box<dyn OdeSolver<T, VArr<V>> + 'a>;
}
