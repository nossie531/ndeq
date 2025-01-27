//! Provider of [`Euler`].

use crate::ode::solver::SsOdeSolver;
use crate::ode::values::{Time, Value};
use crate::ode::Yp;
use crate::util;
use std::ops::MulAssign;

/// ODE solver by [Euler methods].
///
/// [Euler methods]: https://en.wikipedia.org/wiki/Euler_method
pub struct Euler<T, V> {
    /// Step size.
    h: T,

    /// New value.
    new_value: V,

    /// Work for general.
    work: V,

    /// Work for slope.
    slope: V,
}

impl<T, V> Euler<T, V>
where
    T: Time,
    V: Value + MulAssign<T>,
{
    /// Creates a new instance.
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
            new_value: Default::default(),
            work: Default::default(),
            slope: Default::default(),
        })
    }
}

impl<T, V> SsOdeSolver<T, V> for Euler<T, V>
where
    T: Time,
    V: Value + MulAssign<T>,
{
    fn h(&self) -> T {
        self.h
    }

    fn init_dim(&mut self, value: &V) {
        self.new_value.init_dim(value);
        self.work.init_dim(value);
        self.slope.init_dim(value);
    }

    fn step(&mut self, yp: &Yp<V>, value: &V, h: T) -> &V {
        yp(&mut self.slope, value);
        let slope = util::work_mul(&mut self.work, &self.slope, h);
        self.new_value.clone_from(value);
        self.new_value += &*slope;
        &self.new_value
    }
}
