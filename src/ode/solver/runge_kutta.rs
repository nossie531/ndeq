//! Provider of [`RungeKutta`].

use crate::ode::df::{flat, Yp};
use crate::ode::values::{Time, Value};
use crate::prelude::*;
use crate::util;
use std::marker::PhantomData;
use std::ops::Mul;

/// ODE solver by [Runge-Kutta methods].
///
/// [Runge-Kutta methods]: https://en.wikipedia.org/wiki/Runge%E2%80%93Kutta_methods
pub struct RungeKutta<V, T> {
    /// Step size.
    h: T,

    /// Derivative function.
    yp: Yp<V>,

    /// Work points.
    points: [Vec<V>; 4],

    /// Work slopes.
    slopes: [Vec<V>; 4],

    /// New values.
    new_values: Vec<V>,

    /// Dummy.
    pd: PhantomData<T>,
}

impl<V, T> RungeKutta<V, T>
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
            points: Default::default(),
            slopes: Default::default(),
            new_values: Default::default(),
            pd: Default::default(),
        })
    }

    /// Init works.
    fn init_works(&mut self, len: usize) {
        self.points
            .iter_mut()
            .for_each(|x| x.resize(len, V::default()));
        self.slopes
            .iter_mut()
            .for_each(|x| x.resize(len, V::default()));
        self.new_values.resize(len, V::default());
    }

    /// Updates node values to their values after small time.
    fn step(&mut self, values: &[V], h: T) {
        assert!(!h.is_nan());

        self.step0(values);
        self.step1(h);
        self.step2(h);
        self.step3(h);

        for (i, &value) in values.iter().enumerate() {
            let k1 = self.slopes[0][i];
            let k2 = self.slopes[1][i];
            let k3 = self.slopes[2][i];
            let k4 = self.slopes[3][i];
            let slope = (k1 + (k2 + k3) * 2.0 + k4) * (1.0 / 6.0);
            let new_value = value + slope * h;
            self.new_values[i] = new_value;
        }
    }

    /// Calculate step 0.
    fn step0(&mut self, values: &[V]) {
        self.points[0].copy_from_slice(values);
        (self.yp)(&mut self.slopes[0], &self.points[0]);
    }

    /// Calculate step 1.
    fn step1(&mut self, h: T) {
        let (points, rest) = self.points.split_at_mut(1);
        let new_points = &mut rest.first_mut().unwrap();

        for (i, new_point) in new_points.iter_mut().enumerate() {
            *new_point = points[0][i] + self.slopes[0][i] * (h / 2.0);
        }

        (self.yp)(&mut self.slopes[1], new_points);
    }

    /// Calculate step 2.
    fn step2(&mut self, h: T) {
        let (points, rest) = self.points.split_at_mut(2);
        let new_points = &mut rest.first_mut().unwrap();

        for (i, new_point) in new_points.iter_mut().enumerate() {
            *new_point = points[0][i] + self.slopes[1][i] * (h / 2.0);
        }

        (self.yp)(&mut self.slopes[2], new_points);
    }

    /// Calculate step 3.
    fn step3(&mut self, h: T) {
        let (points, rest) = self.points.split_at_mut(3);
        let new_points = &mut rest.first_mut().unwrap();

        for (i, new_point) in new_points.iter_mut().enumerate() {
            *new_point = points[0][i] + self.slopes[2][i] * h;
        }

        (self.yp)(&mut self.slopes[3], new_points);
    }
}

impl<V, T> OdeSolver<V, T> for RungeKutta<V, T>
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
