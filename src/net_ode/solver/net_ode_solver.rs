//! Provider of [`NetOdeSolver`].

use crate::parts::NdeqNet;
use dyn_compatible::prelude::*;

/// ODE solver for network.
#[dyn_compatible(true)]
pub trait NetOdeSolver<'a, T, V> {
    /// Returns new node values of network.
    fn new_values(&self) -> &[V];

    /// Sets network of this instance.
    fn set_net(&mut self, net: &'a dyn NdeqNet<V>);

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
