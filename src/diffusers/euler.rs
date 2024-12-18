//! Provider of [`Euler`].

use crate::net_parts::NdeqNet;
use crate::prelude::*;
use crate::util::time_util;
use crate::values::{Time, Value};
use std::marker::PhantomData;
use std::ops::Mul;

/// Diffusion algorithm by [Euler methods].
///
/// [Euler methods]: https://en.wikipedia.org/wiki/Euler_method
pub struct Euler<V, T> {
    /// Step size.
    h: T,

    /// Dummy.
    pd: PhantomData<(V, T)>,
}

impl<V, T> Euler<V, T>
where
    V: Value + Mul<T, Output = V>,
    T: Time,
{
    /// Create new instance with step size.
    ///
    /// # Panics
    ///
    /// Panics if `h` is zero or negative or NaN or infinity.
    #[must_use]
    pub fn new(h: T) -> Box<Self> {
        assert!(!h.is_nan());
        assert!(!h.is_infinite());
        assert!(h > T::zero());
        Box::new(Self {
            h,
            pd: Default::default(),
        })
    }

    /// Calculate node values after small time.
    fn step(&mut self, net: &mut NdeqNet<V>, h: T) {
        for i in 0..net.nodes().len() {
            let curr = net.nodes()[i].value();
            let flows = net.edges_of(i).map(|(v, w)| (v - curr) * w);
            let value = curr + V::sum(flows) * h;
            net.nodes_mut()[i].set_new_value(value);
        }
    }
}

impl<V, T> Diffuser<V, T> for Euler<V, T>
where
    V: Value + Mul<T, Output = V>,
    T: Time,
{
    fn run(&mut self, net: &mut NdeqNet<V>, p: T) {
        assert!(!p.is_nan());

        let mut t = T::zero();
        while t.abs() < p.abs() {
            let h = time_util::adjust_h(self.h, p, t);
            self.step(net, h);
            net.update_values();
            t = t + h;
        }
    }
}
