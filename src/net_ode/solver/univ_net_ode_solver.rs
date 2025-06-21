//! Provider of [`UnivNetOdeSolver`].

use crate::net_ode::solver::NetOdeSolver;
use crate::parts::NdeqNet;
use ndeq_ode::solver::UnivOdeSolver;
use ndeq_ode::values::{Time, VArr, Value};
use std::ops::MulAssign;

/// Universal ODE solver for network.
pub struct UnivNetOdeSolver<'a, T, V> {
    base: Box<dyn UnivOdeSolver<'a, T, VArr<V>> + 'a>,
    net: Option<&'a dyn NdeqNet<V>>,
    values: VArr<V>,
}

impl<'a, T, V> UnivNetOdeSolver<'a, T, V>
where
    T: Time,
    V: Value + MulAssign<T>,
{
    /// Creates a new instance.
    pub fn new(base: Box<dyn UnivOdeSolver<'a, T, VArr<V>> + 'a>) -> Self {
        Self {
            base: base,
            net: Default::default(),
            values: Default::default(),
        }
    }
}

impl<'a, T, V> NetOdeSolver<'a, T, V> for UnivNetOdeSolver<'a, T, V>
where
    T: Time,
    V: Value + MulAssign<T>,
{
    fn new_values(&self) -> &[V] {
        self.base.new_value().as_ref()
    }
    
    fn set_net(&mut self, net: &'a dyn NdeqNet<V>) {
        self.net = Some(net);
        self.base.set_slope(net.slope());
    }
    
    fn run(&mut self, t: T) {
        self.net.unwrap().export_values(self.values.as_mut());
        self.base.set_value(&self.values);
        self.base.run(t);
    }
}
