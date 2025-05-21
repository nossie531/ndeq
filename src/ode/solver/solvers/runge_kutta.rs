//! Provider of [`RungeKutta`].

use crate::ode::solver::OdeSolver;
use crate::ode::values::{Time, Value};
use crate::ode::{Slope, ode_util};
use crate::util::WorkOn;
use std::ops::MulAssign;
use std::rc::Rc;

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
    slope: Rc<Slope<'a, V>>,

    /// Work for general.
    work: V,

    /// Work for points.
    points: [V; 4],

    /// Work for gradients.
    grads: [V; 4],
}

impl<T, V> RungeKutta<'_, T, V>
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
            points: Default::default(),
            grads: Default::default(),
        })
    }

    /// Advance step.
    fn step(&mut self, h: T, slope: Rc<Slope<V>>) {
        assert!(!h.is_nan());

        self.step0(slope.clone());
        self.step1(slope.clone(), h);
        self.step2(slope.clone(), h);
        self.step3(slope.clone(), h);

        self.grads[0] *= h * 1.0 / 6.0;
        self.grads[1] *= h * 2.0 / 6.0;
        self.grads[2] *= h * 2.0 / 6.0;
        self.grads[3] *= h * 1.0 / 6.0;
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
    fn step0(&mut self, slope: Rc<Slope<V>>) {
        self.points[0].clone_from(&self.old_value);
        slope(&mut self.grads[0], &self.points[0]);
    }

    /// Calculate step 1.
    fn step1(&mut self, slope: Rc<Slope<V>>, h: T) {
        let (points, rest) = self.points.split_at_mut(1);
        let dy = WorkOn(&mut self.work)
            .set(&self.grads[0])
            .calc(|w| *w *= h / 2.0);
        rest[0] += &points[0];
        rest[0] += dy;
        slope(&mut self.grads[1], &mut rest[0]);
    }

    /// Calculate step 2.
    fn step2(&mut self, slope: Rc<Slope<V>>, h: T) {
        let (points, rest) = self.points.split_at_mut(2);
        let dy = WorkOn(&mut self.work)
            .set(&self.grads[1])
            .calc(|w| *w *= h / 2.0);
        rest[0] += &points[0];
        rest[0] += dy;
        slope(&mut self.grads[2], &mut rest[0]);
    }

    /// Calculate step 3.
    fn step3(&mut self, slope: Rc<Slope<V>>, h: T) {
        let (points, rest) = self.points.split_at_mut(3);
        let dy = WorkOn(&mut self.work).set(&self.grads[2]).calc(|w| *w *= h);
        rest[0] += &points[0];
        rest[0] += dy;
        slope(&mut self.grads[3], &mut rest[0]);
    }
}

impl<'a, T, V> OdeSolver<'a, T, V> for RungeKutta<'a, T, V>
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
        self.points.iter_mut().for_each(|x| x.clone_zero(value));
        self.grads.iter_mut().for_each(|x| x.clone_zero(value));
    }

    fn set_slope(&mut self, value: Rc<Slope<'a, V>>) {
        self.slope = value;
    }

    fn run(&mut self, t: T) {
        let h = self.h;
        let mut step = |h| self.step(h, self.slope.clone());
        ode_util::run_steps(t, h, &mut step);
    }
}
