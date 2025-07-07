//! Provider of [`OdeTime`].

use dyn_compatible::prelude::*;
use ndeq_num::prelude::*;

/// ODE system time.
#[dyn_compatible(false)]
pub trait OdeTime: 'static + Float + From<f32> {
    // nop.
}

impl<T> OdeTime for T
where
    T: 'static + Float + From<f32>,
{
    // nop.
}
