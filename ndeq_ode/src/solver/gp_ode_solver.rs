use crate::Slope;
use crate::solver::OdeSolver;
use crate::values::{Time, Value};
use std::ops::MulAssign;
use std::rc::Rc;

/// General purpose ODE solver.
pub trait GpOdeSolver<'a, T, V>: OdeSolver<'a, T, V>
where
    T: Time,
    V: Value + MulAssign<T>,
{
    /// Sets slope of this instance.
    fn set_slope(&mut self, value: Rc<Slope<'a, V>>);
}
