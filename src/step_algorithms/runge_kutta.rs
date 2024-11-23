//! Provider of [`RungeKutta`].

use crate::prelude::*;
use crate::util::sum_values;
use crate::values::{Time, Value};
use std::ops::Mul;
use std::{marker::PhantomData, rc::Rc};

/// Runge Kutta method.
#[derive(Default)]
pub struct RungeKutta<V, T> {
    /// Calculated node values.
    values: Vec<V>,

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
    /// Create new value.
    #[must_use]
    pub fn new() -> Box<Self> {
        Box::default()
    }

    /// Calculate slope at values.
    fn make_slope(slope: &mut Vec<V>, values: &[V], nodes: &[Rc<dyn NdeqNode<V>>]) {
        slope.clear();
        for i in 0..nodes.len() {
            let node = nodes[i].clone();
            let value = values[i];
            let flows = node.in_edges().map(|(n, w)| (n.value() - value) * w);
            slope.push(sum_values(flows));
        }
    }

    /// Calculate step 0.
    fn step0(&mut self, nodes: &[Rc<dyn NdeqNode<V>>]) {
        let iter = nodes.iter().map(|x| x.value());
        self.points[0].clear();
        self.points[0].extend(iter);
        Self::make_slope(&mut self.slopes[0], &self.points[0], nodes);
    }

    /// Calculate step 1.
    fn step1(&mut self, nodes: &[Rc<dyn NdeqNode<V>>], width: T) {
        let (olds, news) = self.points.split_at_mut(1);
        let f = |i| olds[0][i] + self.slopes[0][i] * (width / 2.0);
        news[0].clear();
        news[0].extend((0..nodes.len()).map(f));
        Self::make_slope(&mut self.slopes[1], &news[0], nodes);
    }

    /// Calculate step 2.
    fn step2(&mut self, nodes: &[Rc<dyn NdeqNode<V>>], width: T) {
        let (olds, news) = self.points.split_at_mut(2);
        let f = |i| olds[0][i] + self.slopes[1][i] * (width / 2.0);
        news[0].clear();
        news[0].extend((0..nodes.len()).map(f));
        Self::make_slope(&mut self.slopes[2], &news[0], nodes);
    }

    /// Calculate step 3.
    fn step3(&mut self, nodes: &[Rc<dyn NdeqNode<V>>], width: T) {
        let (olds, news) = self.points.split_at_mut(3);
        let f = |i| olds[0][i] + self.slopes[2][i] * width;
        news[0].clear();
        news[0].extend((0..nodes.len()).map(f));
        Self::make_slope(&mut self.slopes[3], &news[0], nodes);
    }
}

impl<V, T> StepAlgorithm<V, T> for RungeKutta<V, T>
where
    V: Value + Mul<T, Output = V>,
    T: Time,
{
    fn values(&self) -> &[V] {
        self.values.as_slice()
    }

    fn step(&mut self, nodes: &[Rc<dyn NdeqNode<V>>], width: T) {
        assert!(width.is_num());
        self.step0(nodes);
        self.step1(nodes, width);
        self.step2(nodes, width);
        self.step3(nodes, width);
        self.values.clear();
        self.values.extend((0..nodes.len()).map(|i| {
            let k1 = self.slopes[0][i];
            let k2 = self.slopes[1][i];
            let k3 = self.slopes[2][i];
            let k4 = self.slopes[3][i];
            let slope = (k1 + (k2 + k3) * 2.0 + k4) * (1.0 / 6.0);
            nodes[i].value() + slope * width
        }))
    }
}
