//! Provider of [`Euler`].

use crate::ode::solver::SsOdeSolver;
use crate::ode::values::{Time, Value};
use crate::ode::Yp;
use std::ops::Mul;

/// ODE solver by [Euler methods].
///
/// [Euler methods]: https://en.wikipedia.org/wiki/Euler_method
pub struct Euler<'a, V, T> {
    /// Step size.
    h: T,

    /// Derivative function.
    yp: Yp<'a, V>,

    /// Work slopes.
    slopes: Vec<V>,

    /// New values.
    new_values: Vec<V>,
}

impl<'a, V, T> Euler<'a, V, T>
where
    V: Value + Mul<T, Output = V>,
    T: Time,
{
    /// Creates a new instance.
    ///
    /// # Panics
    ///
    /// Panics if `h` is zero or negative or NaN or infinity.
    #[must_use]
    pub fn new(h: T, yp: Yp<'a, V>) -> Box<Self> {
        assert!(!h.is_nan());
        assert!(!h.is_infinite());
        assert!(h > T::zero());
        Box::new(Self {
            h,
            yp,
            slopes: Default::default(),
            new_values: Default::default(),
        })
    }
}

impl<'a, V, T> SsOdeSolver<V, T> for Euler<'a, V, T>
where
    V: Value + Mul<T, Output = V>,
    T: Time,
{
    fn h(&self) -> T {
        self.h
    }

    fn init(&mut self, len: usize) {
        self.slopes.resize(len, V::default());
        self.new_values.resize(len, V::default());
    }

    fn step(&mut self, values: &[V], h: T) -> &[V] {
        (self.yp)(&mut self.slopes, values);

        for (i, &value) in values.iter().enumerate() {
            let new_value = value + self.slopes[i] * h;
            self.new_values[i] = new_value;
        }

        &self.new_values
    }
}
