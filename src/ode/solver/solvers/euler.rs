//! Provider of [`Euler`].

use crate::ode::solver::SsOdeSolver;
use crate::ode::values::{Time, Value};
use crate::ode::Yp;
use crate::util;
use std::ops::MulAssign;

/// ODE solver by [Euler methods].
///
/// [Euler methods]: https://en.wikipedia.org/wiki/Euler_method
pub struct Euler<'a, T, V> {
    /// Step size.
    h: T,

    /// Derivative function.
    yp: Yp<'a, V>,

    /// New value.
    new_value: V,

    /// Work for general.
    work: V,

    /// Work for slope.
    slope: V,
}

impl<'a, T, V> Euler<'a, T, V>
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
    pub fn new(h: T, yp: Yp<'a, V>) -> Box<Self> {
        assert!(!h.is_nan());
        assert!(!h.is_infinite());
        assert!(h > T::zero());
        Box::new(Self {
            h,
            yp,
            new_value: Default::default(),
            work: Default::default(),
            slope: Default::default(),
        })
    }
}

impl<'a, T, V> SsOdeSolver<T, V> for Euler<'a, T, V>
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

    fn step(&mut self, value: &V, h: T) -> &V {
        (self.yp)(&mut self.slope, value);
        let slope = util::work_mul(&mut self.work, &self.slope, h);
        self.new_value.clone_from(value);
        self.new_value += &*slope;
        &self.new_value
    }
}
