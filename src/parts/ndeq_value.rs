use std::ops::{AddAssign, SubAssign};

use dyn_compatible::prelude::*;
use ndeq_num::prelude::*;

/// Abstraction trait for Network node value.
#[dyn_compatible(false)]
pub trait NdeqValue: 'static + for<'a> AddAssign<&'a Self> + for<'a> SubAssign<&'a Self> {
    fn len(&self) -> usize;
}

impl<T> NdeqValue for T
where
    T: Float,
{
    fn len(&self) -> usize {
        1
    }
    // nop.
}
