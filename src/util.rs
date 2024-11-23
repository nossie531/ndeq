//! Crate's utility.

use crate::values::Value;

/// Sums the elements of an iterator.
pub fn sum_values<V: Value>(values: impl Iterator<Item = V>) -> V {
    let mut ret = V::default();
    for value in values {
        ret = ret + value
    }

    ret
}
