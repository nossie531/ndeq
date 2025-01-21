//! Crate's utility.

use std::ops::MulAssign;

/// Multiplies two values with work (for memory saving).
pub fn work_mul<'w, X, Y>(w: &'w mut X, x: &X, y: Y) -> &'w mut X
where
    X: Clone,
    X: MulAssign<Y>,
{
    w.clone_from(x);
    *w *= y;
    w
}
