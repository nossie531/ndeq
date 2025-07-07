//! Provider of [`UnivNetOdeSolver`].

use crate::prelude::*;
use crate::{NodeVal, NodeVec};
use ndeq_ode::solver::UnivOdeSolver;
use ndeq_ode::values::OdeTime;

/// Universal ODE solver for network.
pub struct UnivNetOdeSolver<'a, T, V> {
    base: Box<dyn UnivOdeSolver<'a, T, NodeVec<V>> + 'a>,
    flow: Option<&'a dyn NetFlow<V>>,
    values: NodeVec<V>,
}

impl<'a, T, V> UnivNetOdeSolver<'a, T, V>
where
    T: OdeTime,
    V: NodeVal<T>,
{
    /// Creates a new value.
    pub fn new(base: Box<dyn UnivOdeSolver<'a, T, NodeVec<V>> + 'a>) -> Self {
        Self {
            base: base,
            flow: Default::default(),
            values: Default::default(),
        }
    }
}

impl<'a, T, V> NetOdeSolver<'a, T, V> for UnivNetOdeSolver<'a, T, V>
where
    T: OdeTime,
    V: NodeVal<T>,
{
    fn new_values(&self) -> &[V] {
        self.base.new_value().as_ref()
    }

    fn set_flow(&mut self, value: &'a dyn NetFlow<V>) {
        self.flow = Some(value);
        self.base.set_slope(value.slope());
    }

    fn run(&mut self, t: T) {
        self.flow.unwrap().export_last_values(self.values.as_mut());
        self.base.set_value(&self.values);
        self.base.run(t);
    }
}
