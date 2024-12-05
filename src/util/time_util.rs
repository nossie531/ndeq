//! Time utility.

use crate::values::Time;

/// Adjust calculation step size.
pub fn adjust_h<T: Time>(h: T, goal: T, curr: T) -> T {
    let size = (goal - curr).abs().min(h).unwrap_or(h);
    size.copysign(goal)
}
