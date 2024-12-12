//! Provider of [`flat`].

use crate::ode::values::Value;

/// Zero slope derivative function.
pub fn flat<V: Value>(slopes: &mut [V], _values: &[V]) {
    for slope in slopes.iter_mut() {
        *slope = V::zero();
    }
}
