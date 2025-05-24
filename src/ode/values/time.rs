//! Provider of [`Time`].

use crate::ode::values::{Float, RF32};
use std::ops::{Add, Div, Mul, Sub};

/// Time (variable of ODE system).
pub trait Time:
    'static
    + Float
    + Add<RF32, Output = Self>
    + Sub<RF32, Output = Self>
    + Mul<RF32, Output = Self>
    + Div<RF32, Output = Self>
{
    // nop.
}

impl<T> Time for T
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
