//! Tools for ODE.

use crate::FnSlope;
use crate::values::{OdeTime, OdeValue};
use std::rc::Rc;

/// Create flat slope.
pub fn flat_slope<V>() -> FnSlope<'static, V>
where
    V: OdeValue,
{
    Rc::new(|grad, values| grad.clone_zero(values))
}

/// Run `step` with `h` until the total reaches `t`.
pub fn run_steps<T>(t: T, h: T, step: &mut dyn FnMut(T))
where
    T: OdeTime,
{
    assert!(!t.is_nan());
    assert!(!t.is_infinite());

    let mut x = T::zero();
    while x.abs() < t.abs() {
        let h = adjust_h(h, t, x);
        step(h);
        x = x + h;
    }
}

/// Adjust calculation step size.
fn adjust_h<T: OdeTime>(h: T, goal: T, curr: T) -> T {
    let size = (goal - curr).abs().min(h).unwrap_or(h);
    size.copysign(goal)
}
