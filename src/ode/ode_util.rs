//! Utility for ODE.

use crate::ode::Slope;
use crate::ode::values::{Time, Value};
use std::rc::Rc;

/// Create flat slope.
pub fn flat_slope<V>() -> Rc<Slope<'static, V>>
where
    V: Value,
{
    Rc::new(|grad, values| grad.clone_zero(values))
}

/// Run `step` with `h` until the total reaches `t`.
pub fn run_steps<T>(t: T, h: T, step: &mut dyn FnMut(T))
where
    T: Time,
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
fn adjust_h<T: Time>(h: T, goal: T, curr: T) -> T {
    let size = (goal - curr).abs().min(h).unwrap_or(h);
    size.copysign(goal)
}
