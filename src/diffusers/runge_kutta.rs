//! Provider of [`RungeKutta`].

use crate::net_parts::Net;
use crate::prelude::*;
use crate::util::time_util;
use crate::values::{Time, Value};
use std::marker::PhantomData;
use std::ops::Mul;

/// Diffusion algorithm by [Runge-Kutta methods].
///
/// [Runge-Kutta methods]: https://en.wikipedia.org/wiki/Runge%E2%80%93Kutta_methods
pub struct RungeKutta<V, T> {
    /// Step size.
    h: T,

    /// Calculated point values.
    points: [Vec<V>; 4],

    /// Slopes.
    slopes: [Vec<V>; 4],

    // Dummy.
    pd: PhantomData<T>,
}

impl<V, T> RungeKutta<V, T>
where
    V: Value + Mul<T, Output = V>,
    T: Time,
{
    /// Create new instance with step size.
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
            points: Default::default(),
            slopes: Default::default(),
            pd: Default::default(),
        })
    }

    /// Calculate slope at values.
    fn make_slope(slope: &mut Vec<V>, values: &[V], net: &Net<V>) {
        slope.clear();
        for (i, _) in net.nodes().iter().enumerate() {
            let curr = values[i];
            let flows = net.edges_of(i).map(|(v, w)| (v - curr) * w);
            slope.push(V::sum(flows));
        }
    }

    /// Calculate node values after small time.
    fn step(&mut self, net: &mut Net<V>, h: T) {
        assert!(!h.is_nan());
        self.step0(net);
        self.step1(net, h);
        self.step2(net, h);
        self.step3(net, h);
        for i in 0..net.nodes().len() {
            let k1 = self.slopes[0][i];
            let k2 = self.slopes[1][i];
            let k3 = self.slopes[2][i];
            let k4 = self.slopes[3][i];
            let slope = (k1 + (k2 + k3) * 2.0 + k4) * (1.0 / 6.0);
            let value = net.nodes()[i].value();
            let value = value + slope * h;
            net.nodes_mut()[i].set_calced_value(value);
        }
    }

    /// Calculate step 0.
    fn step0(&mut self, net: &Net<V>) {
        let iter = net.nodes().iter().map(|x| x.value());
        self.points[0].clear();
        self.points[0].extend(iter);
        Self::make_slope(&mut self.slopes[0], &self.points[0], net);
    }

    /// Calculate step 1.
    fn step1(&mut self, net: &Net<V>, h: T) {
        let (olds, news) = self.points.split_at_mut(1);
        let f = |i| olds[0][i] + self.slopes[0][i] * (h / 2.0);
        news[0].clear();
        news[0].extend((0..net.nodes().len()).map(f));
        Self::make_slope(&mut self.slopes[1], &news[0], net);
    }

    /// Calculate step 2.
    fn step2(&mut self, net: &Net<V>, h: T) {
        let (olds, news) = self.points.split_at_mut(2);
        let f = |i| olds[0][i] + self.slopes[1][i] * (h / 2.0);
        news[0].clear();
        news[0].extend((0..net.nodes().len()).map(f));
        Self::make_slope(&mut self.slopes[2], &news[0], net);
    }

    /// Calculate step 3.
    fn step3(&mut self, net: &Net<V>, h: T) {
        let (olds, news) = self.points.split_at_mut(3);
        let f = |i| olds[0][i] + self.slopes[2][i] * h;
        news[0].clear();
        news[0].extend((0..net.nodes().len()).map(f));
        Self::make_slope(&mut self.slopes[3], &news[0], net);
    }
}

impl<V, T> Diffuser<V, T> for RungeKutta<V, T>
where
    V: Value + Mul<T, Output = V>,
    T: Time,
{
    fn calc(&mut self, net: &mut Net<V>, p: T) {
        assert!(!p.is_nan());

        let mut t = T::zero();
        while t.abs() < p.abs() {
            let h = time_util::adjust_h(self.h, p, t);
            self.step(net, h);
            net.update_values();
            t = t + h;
        }
    }
}
