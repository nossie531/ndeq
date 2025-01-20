//! Provider of [`SsOdeSolver`].

use crate::ode::solver::OdeSolver;
use crate::ode::values::{Time, Value};
use std::ops::Mul;

/// Single step ODE solver.
#[must_use]
pub trait SsOdeSolver<V, T>: OdeSolver<V, T>
where
    V: Value + Mul<T, Output = V>,
    T: Time,
{
    /// Returns step size.
    fn h(&self) -> T;

    /// Initialize with the number of network nodes.
    fn init(&mut self, len: usize);

    /// Updates values to a moment later values.
    fn step(&mut self, values: &[V], h: T) -> &[V];
}

impl<S, V, T> OdeSolver<V, T> for S
where
    S: SsOdeSolver<V, T>,
    V: Value + Mul<T, Output = V>,
    T: Time,
{
    fn init(&mut self, len: usize) {
        SsOdeSolver::init(self, len);
    }

    fn run(&mut self, values: &mut [V], p: T) {
        assert!(!p.is_nan());

        OdeSolver::init(self, values.len());

        let mut t = T::zero();
        while t.abs() < p.abs() {
            let h = adjust_h(self.h(), p, t);
            let new_values = self.step(values, h);
            values.copy_from_slice(new_values);
            t = t + h;
        }
    }
}

/// Adjust calculation step size.
fn adjust_h<T: Time>(h: T, goal: T, curr: T) -> T {
    let size = (goal - curr).abs().min(h).unwrap_or(h);
    size.copysign(goal)
}
