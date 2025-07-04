//! Provider of [`NetOdeSolver`].

use crate::prelude::*;
use dyn_compatible::prelude::*;

/// ODE solver for network.
#[dyn_compatible(false)]
pub trait NetOdeSolver<'a, T, V> {
    /// Returns new node values of network.
    fn new_values(&self) -> &[V];

    /// Sets network flow.
    fn set_flow(&mut self, value: &'a dyn NdeqFlow<V>);

    /// Calculate new node values.
    ///
    /// `t` can be negative if algorithm supports it.
    ///
    /// # Panics
    ///
    /// Panics if `t` is NaN or infinity or negative
    /// (if algorithm not supports negative values).    
    fn run(&mut self, t: T);
}
