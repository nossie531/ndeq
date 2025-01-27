//! Provider of [`RungeKutta`].

use crate::ode::solver::SsOdeSolver;
use crate::ode::values::{Time, Value};
use crate::ode::Yp;
use crate::util;
use std::ops::MulAssign;

/// ODE solver by [Runge-Kutta methods].
///
/// [Runge-Kutta methods]: https://en.wikipedia.org/wiki/Runge%E2%80%93Kutta_methods
pub struct RungeKutta<T, V> {
    /// Step size.
    h: T,

    /// New value.
    new_value: V,

    /// Work for general.
    work: V,

    /// Work for points.
    points: [V; 4],

    /// Work for slopes.
    slopes: [V; 4],
}

impl<T, V> RungeKutta<T, V>
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
            points: Default::default(),
            slopes: Default::default(),
        })
    }

    /// Calculate step 0.
    fn step0(&mut self, yp: &Yp<V>, value: &V) {
        self.points[0].clone_from(value);
        yp(&mut self.slopes[0], &self.points[0]);
    }

    /// Calculate step 1.
    fn step1(&mut self, yp: &Yp<V>, h: T) {
        let (points, rest) = self.points.split_at_mut(1);
        let slope = util::work_mul(&mut self.work, &self.slopes[0], h / 2.0);
        rest[0] += &points[0];
        rest[0] += &*slope;
        yp(&mut self.slopes[1], &mut rest[0]);
    }

    /// Calculate step 2.
    fn step2(&mut self, yp: &Yp<V>, h: T) {
        let (points, rest) = self.points.split_at_mut(2);
        let slope = util::work_mul(&mut self.work, &self.slopes[1], h / 2.0);
        rest[0] += &points[0];
        rest[0] += &*slope;
        yp(&mut self.slopes[2], &mut rest[0]);
    }

    /// Calculate step 3.
    fn step3(&mut self, yp: &Yp<V>, h: T) {
        let (points, rest) = self.points.split_at_mut(3);
        let slope = util::work_mul(&mut self.work, &self.slopes[2], h);
        rest[0] += &points[0];
        rest[0] += &*slope;
        yp(&mut self.slopes[3], &mut rest[0]);
    }
}

impl<T, V> SsOdeSolver<T, V> for RungeKutta<T, V>
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
        self.points.iter_mut().for_each(|x| x.init_dim(value));
        self.slopes.iter_mut().for_each(|x| x.init_dim(value));
    }

    fn step(&mut self, yp: &Yp<V>, value: &V, h: T) -> &V {
        assert!(!h.is_nan());

        self.step0(yp, value);
        self.step1(yp, h);
        self.step2(yp, h);
        self.step3(yp, h);

        self.slopes[0] *= h * 1.0 / 6.0;
        self.slopes[1] *= h * 2.0 / 6.0;
        self.slopes[2] *= h * 2.0 / 6.0;
        self.slopes[3] *= h * 1.0 / 6.0;
        self.work.fill_zero();
        self.work += &self.slopes[0];
        self.work += &self.slopes[1];
        self.work += &self.slopes[2];
        self.work += &self.slopes[3];
        self.new_value.fill_zero();
        self.new_value += value;
        self.new_value += &self.work;
        &self.new_value
    }
}
