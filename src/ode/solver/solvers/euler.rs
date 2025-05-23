//! Provider of [`Euler`].

use crate::ode::solver::{GpOdeSolver, OdeSolver};
use crate::ode::values::{Time, Value};
use crate::ode::{Slope, ode_util};
use crate::util::WorkOn;
use std::ops::MulAssign;
use std::rc::Rc;

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
    slope: Rc<Slope<'a, V>>,

    /// Work for general.
    work: V,

    /// Work for gradient.
    grad: V,
}

impl<T, V> Euler<'_, T, V>
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
            old_value: Default::default(),
            new_value: Default::default(),
            slope: ode_util::flat_slope(),
            work: Default::default(),
            grad: Default::default(),
        })
    }

    /// Advance step.
    fn step(&mut self, h: T, slope: Rc<Slope<V>>) {
        slope(&mut self.grad, &self.old_value);
        let dy = WorkOn(&mut self.work).set(&self.grad).calc(|x| *x *= h);
        self.new_value.clone_from(&self.old_value);
        self.new_value += dy;
    }
}

impl<'a, T, V> OdeSolver<'a, T, V> for Euler<'a, T, V>
where
    T: Time,
    V: Value + MulAssign<T>,
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
        ode_util::run_steps(t, h, &mut step);
    }
}

impl<'a, T, V> GpOdeSolver<'a, T, V> for Euler<'a, T, V>
where
    T: Time,
    V: Value + MulAssign<T>,
{
    fn set_slope(&mut self, value: Rc<Slope<'a, V>>) {
        self.slope = value;
    }
}
