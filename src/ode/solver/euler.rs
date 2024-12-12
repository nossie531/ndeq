//! Provider of [`Euler`].

use crate::ode::df::{flat, Yp};
use crate::ode::values::{Time, Value};
use crate::prelude::*;
use crate::util;
use std::marker::PhantomData;
use std::ops::Mul;

/// ODE solver by [Euler methods].
///
/// [Euler methods]: https://en.wikipedia.org/wiki/Euler_method
pub struct Euler<V, T> {
    /// Step size.
    h: T,

    /// Derivative function.
    yp: Yp<V>,

    /// Work slopes.
    slopes: Vec<V>,

    /// New values.
    new_values: Vec<V>,

    /// Dummy.
    pd: PhantomData<(V, T)>,
}

impl<V, T> Euler<V, T>
where
    V: Value + Mul<T, Output = V>,
    T: Time,
{
    /// Creates a new instance with step size.
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
            yp: Box::new(flat),
            slopes: Default::default(),
            new_values: Default::default(),
            pd: Default::default(),
        })
    }

    /// Init works.
    fn init_works(&mut self, len: usize) {
        self.slopes.resize(len, V::default());
        self.new_values.resize(len, V::default());
    }

    /// Updates node new values to their values after small time.
    fn step(&mut self, values: &[V], h: T) {
        (self.yp)(&mut self.slopes, values);

        for (i, &value) in values.iter().enumerate() {
            let new_value = value + self.slopes[i] * h;
            self.new_values[i] = new_value;
        }
    }
}

impl<V, T> OdeSolver<V, T> for Euler<V, T>
where
    V: Value + Mul<T, Output = V>,
    T: Time,
{
    fn set_yp(&mut self, value: Yp<V>) {
        self.yp = value;
    }

    fn run(&mut self, values: &mut [V], p: T) {
        assert!(!p.is_nan());

        self.init_works(values.len());

        let mut t = T::zero();
        while t.abs() < p.abs() {
            let h = util::adjust_h(self.h, p, t);
            self.step(values, h);
            values.copy_from_slice(&self.new_values);
            t = t + h;
        }
    }
}
