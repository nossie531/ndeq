//! Provider of [`Euler`].

use crate::solver::{OdeSolver, UnivOdeSolver};
use crate::util::Work;
use crate::values::{OdeTime, OdeValue};
use crate::{FnSlope, tools};
use std::ops::MulAssign;

/// ODE solver by [Euler methods].
///
/// [Euler methods]: https://en.wikipedia.org/wiki/Euler_method
pub struct Euler<'a, T, V> {
    /// Step size.
    h: T,

    /// Old value.
    old_value: V,

    /// New value.
    new_value: V,

    /// Slope closure.
    slope: FnSlope<'a, V>,

    /// Work for general.
    work: V,

    /// Work for gradient.
    grad: V,
}

impl<T, V> Euler<'_, T, V>
where
    T: OdeTime,
    V: OdeValue + MulAssign<T>,
{
    /// Creates a new value.
    ///
    /// # Panics
    ///
    /// Panics if `h` is zero or negative or NaN or infinity.
    #[must_use]
    pub fn new(h: T) -> Self {
        assert!(!h.is_nan());
        assert!(!h.is_infinite());
        assert!(h > T::zero());
        Self {
            h,
            old_value: Default::default(),
            new_value: Default::default(),
            slope: tools::flat_slope(),
            work: Default::default(),
            grad: Default::default(),
        }
    }

    /// Advance step.
    fn step(&mut self, h: T, slope: FnSlope<V>) {
        slope(&mut self.grad, &self.old_value);
        let dy = Work(&mut self.work, &self.grad).exec(|x| *x *= h);
        self.new_value.clone_from(&self.old_value);
        self.new_value += dy;
    }
}

impl<'a, T, V> OdeSolver<T, V> for Euler<'a, T, V>
where
    T: OdeTime,
    V: OdeValue + MulAssign<T>,
{
    fn new_value(&self) -> &V {
        &self.new_value
    }

    fn set_value(&mut self, value: &V) {
        self.old_value.clone_from(value);
        self.new_value.clone_zero(value);
        self.work.clone_zero(value);
        self.grad.clone_zero(value);
    }

    fn run(&mut self, t: T) {
        let h = self.h;
        let mut step = |h| self.step(h, self.slope.clone());
        tools::run_steps(t, h, &mut step);
    }
}

impl<'a, T, V> UnivOdeSolver<'a, T, V> for Euler<'a, T, V>
where
    T: OdeTime,
    V: OdeValue + MulAssign<T>,
{
    fn set_slope(&mut self, value: FnSlope<'a, V>) {
        self.slope = value;
    }
}
