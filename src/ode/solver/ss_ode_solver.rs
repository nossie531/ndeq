//! Provider of [`SsOdeSolver`].

use crate::ode::solver::OdeSolver;
use crate::ode::values::{Time, Value};
use crate::ode::Yp;
use std::ops::MulAssign;

/// Single step ODE solver.
#[must_use]
pub trait SsOdeSolver<T, V>: OdeSolver<T, V>
where
    T: Time,
    V: Value + MulAssign<T>,
{
    /// Returns step size.
    fn h(&self) -> T;

    /// Initialize value dimension.
    fn init_dim(&mut self, value: &V);

    /// Updates value to a moment later value.
    fn step(&mut self, yp: &Yp<V>, value: &V, h: T) -> &V;
}

impl<B, T, V> OdeSolver<T, V> for B
where
    B: SsOdeSolver<T, V>,
    T: Time,
    V: Value + MulAssign<T>,
{
    fn init_dim(&mut self, value: &V) {
        SsOdeSolver::init_dim(self, value);
    }

    fn run(&mut self, value: &mut V, yp: &Yp<V>, p: T) {
        assert!(!p.is_nan());
        assert!(!p.is_infinite());

        OdeSolver::init_dim(self, value);

        let mut t = T::zero();
        while t.abs() < p.abs() {
            let h = adjust_h(self.h(), p, t);
            value.clone_from(self.step(yp, value, h));
            t = t + h;
        }
    }
}

/// Adjust calculation step size.
fn adjust_h<T: Time>(h: T, goal: T, curr: T) -> T {
    let size = (goal - curr).abs().min(h).unwrap_or(h);
    size.copysign(goal)
}
