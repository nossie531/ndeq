//! Provider of [`OdeTime`].

use dyn_compatible::prelude::*;
use ndeq_num::prelude::*;
use std::ops::{Add, Div, Mul, Sub};

/// ODE system time.
#[dyn_compatible(false)]
pub trait OdeTime:
    'static
    + Float
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + From<f32>
{
    // nop.
}

impl<T> OdeTime for T
where
    T: 'static
        + Float
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + From<f32>,
{
    // nop.
}
