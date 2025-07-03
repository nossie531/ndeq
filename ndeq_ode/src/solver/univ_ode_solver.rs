use crate::FnSlope;
use crate::solver::OdeSolver;
use crate::values::{OdeTime, OdeValue};
use dyn_compatible::prelude::*;
use std::ops::MulAssign;

/// Universal ODE solver.
#[dyn_compatible(true)]
pub trait UnivOdeSolver<'a, T, V>: OdeSolver<T, V>
where
    T: OdeTime,
    V: OdeValue + MulAssign<T>,
{
    /// Sets slope of this instance.
    fn set_slope(&mut self, value: FnSlope<'a, V>);
}
