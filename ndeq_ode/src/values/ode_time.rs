//! Provider of [`OdeTime`].

use crate::values::RF32;
use dyn_compatible::prelude::*;
use ndeq_num::prelude::*;
use std::ops::{Add, Div, Mul, Sub};

/// ODE system time.
#[dyn_compatible(false)]
pub trait OdeTime:
    'static
    + Float
    + Add<RF32, Output = Self>
    + Sub<RF32, Output = Self>
    + Mul<RF32, Output = Self>
    + Div<RF32, Output = Self>
{
    // nop.
}

impl<T> OdeTime for T
where
    T: 'static
        + Float
        + Add<RF32, Output = Self>
        + Sub<RF32, Output = Self>
        + Mul<RF32, Output = Self>
        + Div<RF32, Output = Self>,
{
    // nop.
}
