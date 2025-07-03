//! Provider of [`RungeKutta`].

use crate::solver::{OdeSolver, UnivOdeSolver};
use crate::util::Work;
use crate::values::{OdeTime, OdeValue};
use crate::{FnSlope, tools};
use std::ops::MulAssign;

/// ODE solver by [Runge-Kutta methods].
///
/// [Runge-Kutta methods]: https://en.wikipedia.org/wiki/Runge%E2%80%93Kutta_methods
pub struct RungeKutta<'a, T, V> {
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

    /// Work for points.
    points: [V; 4],

    /// Work for gradients.
    grads: [V; 4],
}

impl<T, V> RungeKutta<'_, T, V>
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
            points: Default::default(),
            grads: Default::default(),
        }
    }

    /// Advance step.
    fn step(&mut self, h: T, slope: FnSlope<V>) {
        assert!(!h.is_nan());

        self.step0(slope.clone());
        self.step1(slope.clone(), h);
        self.step2(slope.clone(), h);
        self.step3(slope.clone(), h);

        self.grads[0] *= h * 1.0.into() / 6.0.into();
        self.grads[1] *= h * 2.0.into() / 6.0.into();
        self.grads[2] *= h * 2.0.into() / 6.0.into();
        self.grads[3] *= h * 1.0.into() / 6.0.into();
        self.work.fill_zero();
        self.work += &self.grads[0];
        self.work += &self.grads[1];
        self.work += &self.grads[2];
        self.work += &self.grads[3];
        self.new_value.fill_zero();
        self.new_value += &self.old_value;
        self.new_value += &self.work;
    }

    /// Calculate step 0.
    fn step0(&mut self, slope: FnSlope<V>) {
        self.points[0].clone_from(&self.old_value);
        slope(&mut self.grads[0], &self.points[0]);
    }

    /// Calculate step 1.
    fn step1(&mut self, slope: FnSlope<V>, h: T) {
        let (points, rest) = self.points.split_at_mut(1);
        let dy = Work(&mut self.work, &self.grads[0]).exec(|w| *w *= h / 2.0.into());
        rest[0] += &points[0];
        rest[0] += dy;
        slope(&mut self.grads[1], &mut rest[0]);
    }

    /// Calculate step 2.
    fn step2(&mut self, slope: FnSlope<V>, h: T) {
        let (points, rest) = self.points.split_at_mut(2);
        let dy = Work(&mut self.work, &self.grads[1]).exec(|w| *w *= h / 2.0.into());
        rest[0] += &points[0];
        rest[0] += dy;
        slope(&mut self.grads[2], &mut rest[0]);
    }

    /// Calculate step 3.
    fn step3(&mut self, slope: FnSlope<V>, h: T) {
        let (points, rest) = self.points.split_at_mut(3);
        let dy = Work(&mut self.work, &self.grads[2]).exec(|w| *w *= h);
        rest[0] += &points[0];
        rest[0] += dy;
        slope(&mut self.grads[3], &mut rest[0]);
    }
}

impl<'a, T, V> OdeSolver<T, V> for RungeKutta<'a, T, V>
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
        self.points.iter_mut().for_each(|x| x.clone_zero(value));
        self.grads.iter_mut().for_each(|x| x.clone_zero(value));
    }

    fn run(&mut self, t: T) {
        let h = self.h;
        let mut step = |h| self.step(h, self.slope.clone());
        tools::run_steps(t, h, &mut step);
    }
}

impl<'a, T, V> UnivOdeSolver<'a, T, V> for RungeKutta<'a, T, V>
where
    T: OdeTime,
    V: OdeValue + MulAssign<T>,
{
    fn set_slope(&mut self, value: FnSlope<'a, V>) {
        self.slope = value;
    }
}
